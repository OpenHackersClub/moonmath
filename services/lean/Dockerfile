# Lean compile service — runs inside a Cloudflare Container instance.
#
# Lean toolchain version mirrors lean-project/lean-toolchain so showcase
# snippets compile against the same Lean as the local dev workflow.
#
# Mathlib is NOT bundled in v0: the resulting image fits comfortably under the
# 2 GB Cloudflare Container limit (~350 MB compressed). Showcase snippets that
# rely on Mathlib (Real, ℝ, norm_num, …) will return Lean import errors until
# the v0.2.6 Mathlib follow-up — see docs/lean-service.md.

FROM ubuntu:24.04 AS lean-base

ENV DEBIAN_FRONTEND=noninteractive \
    ELAN_HOME=/opt/elan \
    PATH=/opt/elan/bin:/usr/local/bin:/usr/bin:/bin

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        curl ca-certificates git unzip \
 && rm -rf /var/lib/apt/lists/*

# Install elan (the Lean toolchain manager) and pin to the project's Lean.
ARG LEAN_VERSION=v4.28.0
RUN curl -sSfL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh \
        -o /tmp/elan-init.sh \
 && sh /tmp/elan-init.sh -y --default-toolchain "leanprover/lean4:${LEAN_VERSION}" \
 && rm /tmp/elan-init.sh

# Verify the toolchain is on PATH and warm up the .olean cache for `lean`.
RUN lean --version && lake --version

# ─── Server stage ────────────────────────────────────────────────────────────

FROM lean-base AS server

# Bun ships as a single static binary — smaller than node + npm and good
# enough for a tiny HTTP wrapper around `lean --json`.
RUN curl -fsSL https://bun.sh/install | bash \
 && mv /root/.bun/bin/bun /usr/local/bin/bun \
 && rm -rf /root/.bun

WORKDIR /srv
COPY server.ts ./server.ts

EXPOSE 8080

# `bun run` JIT-compiles the TS file on first hit; that's fine for a long-lived
# container instance. If startup latency becomes a concern we can `bun build`
# at image time instead.
CMD ["bun", "run", "server.ts"]
