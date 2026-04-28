/**
 * MoonMath — Cloudflare Worker entrypoint.
 *
 * Responsibilities (per specs/prd.md → "Deployment (Cloudflare Workers)"):
 *  - Serve the prerendered SSG output from the Static Assets binding.
 *  - Apply HTTP cache headers: immutable for hashed /pkg/* assets, short TTL
 *    for HTML.
 *  - Provide an optional `POST /api/lean/compile` fallback that proxies to a
 *    managed Lean service when configured, with per-IP rate limiting and a
 *    body size cap.
 *  - Fall back to dist/404.html for unknown paths (handled by the Static
 *    Assets binding's `not_found_handling = "404-page"`).
 */

import { getContainer } from "@cloudflare/containers";
import { LeanCompiler } from "./lean-compiler";

export { LeanCompiler };

interface Env {
  ASSETS: Fetcher;
  LEAN_RATE_LIMIT?: KVNamespace;
  LEAN_CACHE?: KVNamespace;
  LEAN_COMPILER?: DurableObjectNamespace<LeanCompiler>;
  LEAN_FALLBACK_ENABLED: string;
  LEAN_REQUEST_MAX_BYTES: string;
  LEAN_FALLBACK_URL?: string;
}

const RATE_LIMIT_WINDOW_S = 60;
const RATE_LIMIT_MAX_REQUESTS = 20;
// Successful compiles are deterministic — cache them aggressively. KV TTL is
// the floor; the Worker also stamps Cache-Control on the HTTP response.
const COMPILE_CACHE_TTL_S = 86_400;

// Path the Leptos client posts to (server-fn name + "/api" prefix from the
// #[server(CompileLean, "/api")] attribute in showcase_detail.rs).
const COMPILE_LEAN_PATH = "/api/CompileLean";

export default {
  async fetch(request: Request, env: Env, _ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);

    if (url.pathname === COMPILE_LEAN_PATH) {
      return handleCompileLean(request, env);
    }
    if (url.pathname === "/api/lean/compile") {
      return handleLeanCompile(request, env);
    }

    const assetResponse = await env.ASSETS.fetch(request);
    return decorateAssetResponse(url.pathname, assetResponse);
  },
};

function decorateAssetResponse(pathname: string, response: Response): Response {
  // Don't mutate non-OK responses (e.g. the 404.html served via
  // not_found_handling) — pass them through untouched.
  if (!response.ok) return response;

  const headers = new Headers(response.headers);

  if (pathname.startsWith("/pkg/")) {
    // cargo-leptos emits content-hashed filenames under /pkg/, so they're
    // safe to cache forever.
    headers.set("Cache-Control", "public, max-age=31536000, immutable");
  } else if (pathname.startsWith("/data/")) {
    // Generated SSG JSON — short TTL, must-revalidate so content updates
    // surface within a minute.
    headers.set("Cache-Control", "public, max-age=60, must-revalidate");
  } else if (
    pathname.endsWith(".html") ||
    pathname === "/" ||
    !pathname.includes(".")
  ) {
    // Pretty URLs (resolved to index.html) — short TTL with revalidation.
    headers.set("Cache-Control", "public, max-age=60, must-revalidate");
  }

  // Always set basic security headers on HTML.
  if ((headers.get("Content-Type") ?? "").startsWith("text/html")) {
    headers.set("X-Content-Type-Options", "nosniff");
    headers.set("Referrer-Policy", "strict-origin-when-cross-origin");
    headers.set("Permissions-Policy", "interest-cohort=()");
  }

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

/**
 * Server-fn endpoint for `compile_lean` (showcase_detail.rs).
 *
 * Wire format (Leptos default `PostUrl` input + `Json` output):
 *   request:  POST /api/CompileLean
 *             Content-Type: application/x-www-form-urlencoded
 *             body: code=<urlencoded source>
 *   success:  200 application/json   → CompileResponse
 *   failure:  4xx/5xx text/plain     → "ServerError|<message>"
 *
 * On error we use the Leptos-recognised `ServerError|<msg>` body so the
 * client surfaces a real error (server_fn-0.7.x parses this in `de(&str)`)
 * instead of the cryptic "Could not deserialize error \"\"" we'd get from an
 * empty 4xx body.
 */
async function handleCompileLean(request: Request, env: Env): Promise<Response> {
  if (request.method !== "POST") {
    return serverFnError("method_not_allowed", 405);
  }

  const maxBytes = Number.parseInt(env.LEAN_REQUEST_MAX_BYTES ?? "32768", 10);
  const formText = await request.text();
  if (formText.length > maxBytes) {
    return serverFnError(`payload too large (limit ${maxBytes} bytes)`, 413);
  }

  let code: string;
  try {
    code = decodeFormCode(formText);
  } catch (e) {
    return serverFnError(`invalid request body: ${(e as Error).message}`, 400);
  }

  const ip =
    request.headers.get("CF-Connecting-IP") ??
    request.headers.get("X-Forwarded-For") ??
    "unknown";
  if (!(await checkRateLimit(env, ip))) {
    return serverFnError(
      `rate limited — retry in ${RATE_LIMIT_WINDOW_S}s`,
      429,
    );
  }

  const cacheKey = await sha256Hex(code);
  if (env.LEAN_CACHE) {
    const hit = await env.LEAN_CACHE.get(cacheKey);
    if (hit) {
      return jsonOk(hit, { cacheHit: true });
    }
  }

  if (!env.LEAN_COMPILER) {
    return serverFnError(
      "Lean compile container is not bound on this deployment. " +
        "Set [[durable_objects.bindings]] LEAN_COMPILER in wrangler.toml.",
      503,
    );
  }

  let upstream: Response;
  try {
    const container = getContainer(env.LEAN_COMPILER);
    upstream = await container.fetch(
      new Request("http://lean-compiler.internal/compile", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code }),
      }),
    );
  } catch (e) {
    return serverFnError(
      `Lean compile container unreachable: ${(e as Error).message}`,
      502,
    );
  }

  const upstreamText = await upstream.text();
  if (!upstream.ok) {
    return serverFnError(
      `Lean compile container returned ${upstream.status}: ${upstreamText.slice(0, 256)}`,
      502,
    );
  }

  // Cache successful responses only — including compile failures (where
  // success: false) so users see consistent diagnostics on repeat clicks.
  // The cache key is sha256(code) so any source change busts it.
  if (env.LEAN_CACHE) {
    await env.LEAN_CACHE.put(cacheKey, upstreamText, {
      expirationTtl: COMPILE_CACHE_TTL_S,
    });
  }

  return jsonOk(upstreamText, { cacheHit: false });
}

function decodeFormCode(formBody: string): string {
  // Server-fn `PostUrl` input is a URL-encoded form. For
  // compile_lean(code: String) the body is `code=<urlencoded>`.
  const params = new URLSearchParams(formBody);
  const code = params.get("code");
  if (code === null) throw new Error("missing `code` field");
  return code;
}

async function sha256Hex(input: string): Promise<string> {
  const buf = await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(input),
  );
  return Array.from(new Uint8Array(buf))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

function jsonOk(body: string, opts: { cacheHit: boolean }): Response {
  return new Response(body, {
    status: 200,
    headers: {
      "Content-Type": "application/json; charset=utf-8",
      "Cache-Control": "no-store",
      "X-Lean-Cache": opts.cacheHit ? "hit" : "miss",
    },
  });
}

function serverFnError(message: string, status: number): Response {
  return new Response(`ServerError|${message}`, {
    status,
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
      "Cache-Control": "no-store",
    },
  });
}

async function handleLeanCompile(request: Request, env: Env): Promise<Response> {
  if (request.method !== "POST") {
    return json({ error: "method_not_allowed" }, 405);
  }

  if (env.LEAN_FALLBACK_ENABLED !== "true" || !env.LEAN_FALLBACK_URL) {
    return json(
      {
        error: "fallback_disabled",
        message:
          "Server-side Lean compile is not configured for this deployment. " +
          "Use the in-browser WASM compiler instead.",
      },
      503,
    );
  }

  const maxBytes = Number.parseInt(env.LEAN_REQUEST_MAX_BYTES ?? "32768", 10);
  const bodyText = await request.text();
  if (bodyText.length > maxBytes) {
    return json({ error: "payload_too_large", limit_bytes: maxBytes }, 413);
  }

  const ip =
    request.headers.get("CF-Connecting-IP") ??
    request.headers.get("X-Forwarded-For") ??
    "unknown";
  const allowed = await checkRateLimit(env, ip);
  if (!allowed) {
    return json(
      {
        error: "rate_limited",
        retry_after_s: RATE_LIMIT_WINDOW_S,
      },
      429,
    );
  }

  const upstream = await fetch(env.LEAN_FALLBACK_URL, {
    method: "POST",
    headers: {
      "Content-Type": request.headers.get("Content-Type") ?? "application/json",
    },
    body: bodyText,
  });

  // Pass the upstream response through; cap headers we forward.
  const out = new Headers();
  out.set(
    "Content-Type",
    upstream.headers.get("Content-Type") ?? "application/json",
  );
  out.set("Cache-Control", "no-store");
  return new Response(upstream.body, {
    status: upstream.status,
    headers: out,
  });
}

async function checkRateLimit(env: Env, ip: string): Promise<boolean> {
  if (!env.LEAN_RATE_LIMIT) return true;

  const key = `lean:${ip}`;
  const current = await env.LEAN_RATE_LIMIT.get(key);
  const count = current ? Number.parseInt(current, 10) : 0;
  if (count >= RATE_LIMIT_MAX_REQUESTS) return false;

  await env.LEAN_RATE_LIMIT.put(key, String(count + 1), {
    expirationTtl: RATE_LIMIT_WINDOW_S,
  });
  return true;
}

function json(body: unknown, status: number): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: {
      "Content-Type": "application/json; charset=utf-8",
      "Cache-Control": "no-store",
    },
  });
}
