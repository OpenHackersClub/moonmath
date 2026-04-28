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

interface Env {
  ASSETS: Fetcher;
  LEAN_RATE_LIMIT?: KVNamespace;
  LEAN_FALLBACK_ENABLED: string;
  LEAN_REQUEST_MAX_BYTES: string;
  LEAN_FALLBACK_URL?: string;
}

const RATE_LIMIT_WINDOW_S = 60;
const RATE_LIMIT_MAX_REQUESTS = 20;

export default {
  async fetch(request: Request, env: Env, _ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);

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
