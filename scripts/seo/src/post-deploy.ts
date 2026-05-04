/**
 * Item 2 — runs after `wrangler deploy` to production. Three things:
 *   (a) submit sitemap.xml to Search Console
 *   (b) URL-inspect a tiny smoke list and fail on canonical drift
 *   (c) ping IndexNow (Bing/DDG/Yandex/Naver)
 *
 * Search Console hosts properties under one of two `siteUrl` shapes:
 *   - URL-prefix property : "https://moonmath.openhackers.club/"
 *   - Domain property     : "sc-domain:moonmath.openhackers.club"
 * Both are accepted here; pass via `SEARCH_CONSOLE_SITE_URL` if it differs
 * from the deployed `SITE_URL` (e.g. the deploy lives on a subpath but the
 * verified property is the apex domain).
 *
 * Required env:
 *   SITE_URL                       e.g. https://moonmath.openhackers.club
 * Optional env:
 *   SEARCH_CONSOLE_SITE_URL        defaults to SITE_URL with trailing slash
 *   SITEMAP_PATH                   defaults to "/sitemap.xml"
 *   SMOKE_URLS_FILE                defaults to ./smoke-urls.json
 *   INDEXNOW_KEY                   skip IndexNow ping when unset
 *   IMPERSONATE_SA                 act as a service account locally
 *   STRICT_INSPECTION=1            also fail on "URL is unknown to Google"
 */

import { google } from "googleapis";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { getAuthClient, mustEnv } from "./auth.js";

const HERE = dirname(fileURLToPath(import.meta.url));

const SITE_URL = mustEnv("SITE_URL").replace(/\/$/, "");
const SEARCH_CONSOLE_SITE_URL =
  process.env.SEARCH_CONSOLE_SITE_URL?.trim() ?? `${SITE_URL}/`;
const SITEMAP_PATH = process.env.SITEMAP_PATH?.trim() ?? "/sitemap.xml";
const SMOKE_URLS_FILE =
  process.env.SMOKE_URLS_FILE?.trim() ?? join(HERE, "..", "smoke-urls.json");
const INDEXNOW_KEY = process.env.INDEXNOW_KEY?.trim();
const STRICT_INSPECTION = process.env.STRICT_INSPECTION === "1";

interface InspectionFinding {
  path: string;
  reason: string;
}

async function submitSitemap() {
  const auth = await getAuthClient();
  const sc = google.searchconsole({ version: "v1", auth });
  const feedpath = SITE_URL + SITEMAP_PATH;
  console.log(`Submitting sitemap ${feedpath} to ${SEARCH_CONSOLE_SITE_URL}…`);
  await sc.sitemaps.submit({
    siteUrl: SEARCH_CONSOLE_SITE_URL,
    feedpath,
  });
  console.log(`  ✓ submitted`);

  const status = await sc.sitemaps.get({
    siteUrl: SEARCH_CONSOLE_SITE_URL,
    feedpath,
  });
  const errors = status.data.errors ?? 0;
  const warnings = status.data.warnings ?? 0;
  const lastSubmitted = status.data.lastSubmitted ?? "(not yet recorded)";
  console.log(
    `  status: errors=${errors} warnings=${warnings} lastSubmitted=${lastSubmitted}`,
  );
  if (Number(errors) > 0) {
    throw new Error(`sitemap reports ${errors} error(s); see Search Console`);
  }
}

async function inspectUrls(): Promise<InspectionFinding[]> {
  const auth = await getAuthClient();
  const sc = google.searchconsole({ version: "v1", auth });
  const paths: unknown = JSON.parse(readFileSync(SMOKE_URLS_FILE, "utf8"));
  if (!Array.isArray(paths) || paths.some((p) => typeof p !== "string")) {
    throw new Error(`${SMOKE_URLS_FILE} must be a JSON array of strings`);
  }

  const findings: InspectionFinding[] = [];
  for (const path of paths as string[]) {
    const inspectionUrl = SITE_URL + path;
    const resp = await sc.urlInspection.index.inspect({
      requestBody: {
        inspectionUrl,
        siteUrl: SEARCH_CONSOLE_SITE_URL,
      },
    });
    const idx = resp.data.inspectionResult?.indexStatusResult ?? {};
    const verdict = idx.verdict ?? "?";
    const coverage = idx.coverageState ?? "?";
    const userCanon = idx.userCanonical;
    const googleCanon = idx.googleCanonical;
    console.log(
      `  ${path}: verdict=${verdict} coverage=${coverage}` +
        (userCanon ? ` user=${userCanon}` : "") +
        (googleCanon && googleCanon !== userCanon ? ` google=${googleCanon}` : ""),
    );

    if (userCanon && userCanon !== inspectionUrl) {
      findings.push({
        path,
        reason: `user-declared canonical drifted from ${inspectionUrl} to ${userCanon}`,
      });
    }
    if (STRICT_INSPECTION && verdict === "FAIL") {
      findings.push({ path, reason: `inspection verdict=FAIL coverage=${coverage}` });
    }
  }
  return findings;
}

async function pingIndexNow(paths: string[]) {
  if (!INDEXNOW_KEY) {
    console.log(`INDEXNOW_KEY unset — skipping IndexNow ping`);
    return;
  }
  const host = new URL(SITE_URL).hostname;
  const body = {
    host,
    key: INDEXNOW_KEY,
    keyLocation: `${SITE_URL}/${INDEXNOW_KEY}.txt`,
    urlList: paths.map((p) => SITE_URL + p),
  };
  const resp = await fetch("https://api.indexnow.org/indexnow", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  if (resp.status >= 400) {
    const text = await resp.text();
    throw new Error(`IndexNow ${resp.status}: ${text}`);
  }
  console.log(`IndexNow: HTTP ${resp.status}`);
}

async function main() {
  await submitSitemap();
  const findings = await inspectUrls();
  const paths = JSON.parse(readFileSync(SMOKE_URLS_FILE, "utf8")) as string[];
  await pingIndexNow(paths);

  if (findings.length > 0) {
    console.error(`\nURL inspection findings (${findings.length}):`);
    for (const f of findings) console.error(`  - ${f.path}: ${f.reason}`);
    process.exit(1);
  }
  console.log(`\n✓ post-deploy SEO sync clean`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
