// Lean compile service — minimal HTTP wrapper around `lean`.
//
// Runs inside the Cloudflare Container instance (see ./Dockerfile). The Worker
// posts to /compile with JSON { code: string }; we shell out to `lean` and
// return the moonmath-types CompileResponse shape so the Worker can pass the
// body straight back to the Leptos client.
//
// Error parsing mirrors crates/moonmath-lean/src/compiler.rs::parse_lean_errors
// so the two paths produce identical CompileError records.

import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn } from "node:child_process";

const PORT = Number.parseInt(process.env.PORT ?? "8080", 10);
const TIMEOUT_MS = Number.parseInt(process.env.LEAN_TIMEOUT_MS ?? "30000", 10);
const MAX_CODE_BYTES = Number.parseInt(
  process.env.LEAN_MAX_CODE_BYTES ?? "32768",
  10,
);

type Severity = "Error" | "Warning" | "Info";

interface CompileError {
  line: number;
  column: number;
  message: string;
  severity: Severity;
}

interface CompileResponse {
  success: boolean;
  output: string | null;
  errors: CompileError[];
  latex: string | null;
}

interface CompileRequest {
  code: string;
}

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json; charset=utf-8" },
  });
}

function parseLeanErrors(output: string): CompileError[] {
  const errors: CompileError[] = [];
  for (const line of output.split("\n")) {
    // Lean diagnostics: `<file>:<line>:<col>: error|warning|information: <msg>`
    const parts = line.split(":");
    if (parts.length < 5) continue;
    const lineNum = Number.parseInt(parts[1].trim(), 10);
    const col = Number.parseInt(parts[2].trim(), 10);
    if (!Number.isFinite(lineNum) || !Number.isFinite(col)) continue;
    const sev = parts[3].trim();
    let severity: Severity;
    if (sev === "error") severity = "Error";
    else if (sev === "warning") severity = "Warning";
    else if (sev === "information") severity = "Info";
    else continue;
    errors.push({
      line: lineNum,
      column: col,
      message: parts.slice(4).join(":").trim(),
      severity,
    });
  }
  return errors;
}

interface LeanRunResult {
  exit: number;
  stdout: string;
  stderr: string;
  timedOut: boolean;
}

function runLean(file: string): Promise<LeanRunResult> {
  return new Promise((resolve) => {
    const child = spawn("lean", [file], { stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    let timedOut = false;
    const timer = setTimeout(() => {
      timedOut = true;
      child.kill("SIGKILL");
    }, TIMEOUT_MS);
    child.stdout.on("data", (b) => (stdout += b.toString()));
    child.stderr.on("data", (b) => (stderr += b.toString()));
    child.on("close", (code) => {
      clearTimeout(timer);
      resolve({ exit: code ?? -1, stdout, stderr, timedOut });
    });
  });
}

async function compile(code: string): Promise<CompileResponse> {
  const dir = await mkdtemp(join(tmpdir(), "lean-"));
  const file = join(dir, "scratch.lean");
  try {
    await writeFile(file, code, "utf8");
    const { exit, stdout, stderr, timedOut } = await runLean(file);

    if (timedOut) {
      return {
        success: false,
        output: `Compilation timed out after ${TIMEOUT_MS}ms`,
        errors: [],
        latex: null,
      };
    }

    const errors = [
      ...parseLeanErrors(stdout),
      ...parseLeanErrors(stderr),
    ];
    const success = exit === 0 && !errors.some((e) => e.severity === "Error");

    // If lean failed but we couldn't parse structured errors, surface raw text
    // so the user sees what went wrong (mirrors moonmath-lean compiler.rs).
    const rawOutput =
      !success && errors.length === 0
        ? `${stdout}${stderr}`.slice(0, 4096) || null
        : null;

    return { success, output: rawOutput, errors, latex: null };
  } finally {
    await rm(dir, { recursive: true, force: true }).catch(() => {});
  }
}

const server = Bun.serve({
  port: PORT,
  hostname: "0.0.0.0",
  async fetch(req) {
    const url = new URL(req.url);

    if (url.pathname === "/health") {
      return jsonResponse({ ok: true });
    }

    if (url.pathname !== "/compile") {
      return jsonResponse({ error: "not_found" }, 404);
    }
    if (req.method !== "POST") {
      return jsonResponse({ error: "method_not_allowed" }, 405);
    }

    let body: CompileRequest;
    try {
      body = await req.json();
    } catch {
      return jsonResponse({ error: "invalid_json" }, 400);
    }
    if (typeof body.code !== "string") {
      return jsonResponse({ error: "missing_code" }, 400);
    }
    if (body.code.length > MAX_CODE_BYTES) {
      return jsonResponse(
        { error: "payload_too_large", limit_bytes: MAX_CODE_BYTES },
        413,
      );
    }

    try {
      const resp = await compile(body.code);
      return jsonResponse(resp);
    } catch (e) {
      return jsonResponse(
        { error: "compile_failed", message: String(e) },
        500,
      );
    }
  },
});

console.log(`[lean-svc] listening on :${server.port}`);
