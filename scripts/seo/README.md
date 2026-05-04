# SEO automation — Search Console + IndexNow

Drives `googleapis` for Google Search Console (sitemap submit, URL inspection, property verification) and a flat HTTP `POST` for IndexNow (Bing, DuckDuckGo, Yandex, Naver). Wired into `.github/workflows/deploy.yml` so every successful production deploy re-pings the search engines.

## Layout

| File | Purpose |
| --- | --- |
| `src/auth.ts` | `googleapis` auth helper. Honours `IMPERSONATE_SA` per `CLAUDE.md` GCP rule. |
| `src/verify-property.ts` | Item 1 — verify a Search Console domain property via DNS TXT (auto-writes the Cloudflare DNS record when `CLOUDFLARE_API_TOKEN` is set). |
| `src/post-deploy.ts` | Item 2 — submit `sitemap.xml`, URL-inspect a smoke list, ping IndexNow. |
| `smoke-urls.json` | Five paths URL Inspection runs against. Edit when the route shape changes. |

## One-time setup (truly manual)

The bits below have **no API**, so the human still has to do them. Once.

1. **Service account.** Create one in the deploy GCP project (e.g. `moonmath-seo@<project>.iam.gserviceaccount.com`).
2. **Search Console Owner.** Open `https://search.google.com/search-console/users` for the property and add the SA email as **Owner** (User-level access can `urlInspection`, but not `sitemaps.submit`). This is the only step that requires the Search Console UI.
3. **IndexNow key.** `openssl rand -hex 16` once. Save as the `INDEXNOW_KEY` repo secret on GitHub (Actions → Secrets → New repository secret). The deploy workflow writes `dist/${INDEXNOW_KEY}.txt` containing that key before `wrangler deploy` so the file is published at the site root, which IndexNow validates against on every ping.
4. **CI auth — pick one.** Workload Identity Federation is preferred per `CLAUDE.md` (no long-lived keys); SA key fallback is documented but discouraged.
   - **WIF (preferred):** create a pool + provider for the `OpenHackersClub/moonmath` repo, grant the SA `roles/iam.workloadIdentityUser` on that provider, then set the GitHub *variables* (not secrets — they aren't sensitive):
     - `vars.GCP_WIF_PROVIDER` — full resource path (`projects/.../providers/...`)
     - `vars.GCP_SEARCH_CONSOLE_SA` — the SA email
     - The deploy job already has `permissions: id-token: write` for this.
   - **SA key fallback:** download a JSON key, paste into the `GCP_SEARCH_CONSOLE_SA_KEY` secret, swap the `auth@v2` step's `workload_identity_provider` for `credentials_json`. Rotate quarterly. Avoid if at all possible.

## One-time setup (scriptable, run once)

5. **Verify the domain property.** From your dev machine, with ADC + impersonation:

   ```sh
   gcloud auth application-default login
   cd scripts/seo
   npm install

   IMPERSONATE_SA=moonmath-seo@<project>.iam.gserviceaccount.com \
   SITE_URL=https://moonmath.openhackers.club \
   CLOUDFLARE_API_TOKEN=<token-with-zone-edit-on-openhackers.club> \
     npm run verify-property
   ```

   The script fetches the TXT token, writes it to the Cloudflare zone, polls DNS until visible, then calls the Site Verification API. If you'd rather paste the TXT record yourself, omit `CLOUDFLARE_API_TOKEN` — the script prints the value and waits for it to propagate.

   Re-running is safe — every step is idempotent.

## Per-deploy automation (no action needed)

`.github/workflows/deploy.yml` runs `npm run post-deploy` after every successful production deploy + acceptance gate. The job:

- Submits the freshly-deployed `sitemap.xml` (re-fetches Google's view of it and prints `errors` / `warnings` / `lastSubmitted`).
- URL-inspects every path in `smoke-urls.json`. Fails the run on **canonical drift** — Google reporting a different `userCanonical` than the URL inspected. Set `STRICT_INSPECTION=1` to also fail on `verdict=FAIL` (off by default — new pages take ~7 days to leave that state).
- Pings IndexNow with the smoke list. Skipped when `INDEXNOW_KEY` is unset.

## Local dev / manual run

```sh
gcloud auth application-default login

cd scripts/seo
npm install

IMPERSONATE_SA=moonmath-seo@<project>.iam.gserviceaccount.com \
SITE_URL=https://moonmath.openhackers.club \
INDEXNOW_KEY=<key> \
  npm run post-deploy
```

`SITE_URL` *must* be HTTPS and resolvable from the runner — Search Console can't inspect a URL it can't fetch.

## What this does NOT cover

- **Request Indexing** — UI-only for general content. Google's official `Indexing API` is restricted to `JobPosting` + `BroadcastEvent`; using it for general pages is technically out-of-policy and can be revoked. Don't.
- **Robots.txt validation** — `cargo run -p moonmath-ssg` already emits it; `scripts/acceptance.sh` smoke-checks it.
- **Search Analytics pull** — separate cron, not yet wired. The `searchanalytics.query` API is on the same auth and would slot under `src/` if/when needed.
