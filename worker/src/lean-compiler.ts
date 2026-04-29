/**
 * LeanCompiler — Cloudflare Container Durable Object class.
 *
 * The container itself is built from ../services/lean/Dockerfile and exposes a
 * tiny HTTP service on :8080 (see ../services/lean/server.ts). This file only
 * declares the DO class so wrangler can bind it; routing happens in index.ts.
 *
 * Lifecycle: Cloudflare spins the container up on first request and lets it
 * idle until `sleepAfter`. Compiles take ~1–3 s once warm; cold starts add
 * ~2–5 s. The Worker caches successful compiles in `LEAN_CACHE` KV keyed by
 * sha256(code) to dodge the container roundtrip on demo replays.
 */

import { Container } from "@cloudflare/containers";

export class LeanCompiler extends Container {
  defaultPort = 8080;

  // Keep the container warm for 10 minutes after the last request. Most
  // showcase visits cluster within a session, so this trades a small idle
  // cost for big latency wins on subsequent compiles.
  sleepAfter = "10m";

  // No env vars needed yet — server.ts reads PORT/LEAN_TIMEOUT_MS with sane
  // defaults. When we add Mathlib (v0.2.6) we'll likely set MATHLIB_DIR here.
  envVars = {};
}
