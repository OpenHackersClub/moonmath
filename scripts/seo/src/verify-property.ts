/**
 * Item 1 helper — verify a Search Console domain property via DNS TXT.
 *
 * Flow:
 *   1. Ask Google for the verification token (TXT record value).
 *   2. If CLOUDFLARE_API_TOKEN is set, write the TXT record to the zone.
 *      Otherwise print instructions for manual entry.
 *   3. Wait for DNS propagation.
 *   4. Call siteVerification.webResource.insert to claim the property.
 *
 * The API can NOT add humans / service accounts as Owners — that step is
 * still UI-only at https://search.google.com/search-console/users.
 *
 * Usage:
 *   SITE_URL=https://moonmath.openhackers.club npm run verify-property
 *   # plus, optionally:
 *   #   CLOUDFLARE_API_TOKEN=…       (auto-write the TXT record)
 *   #   CF_ZONE_NAME=openhackers.club (override the auto-derived zone)
 *   #   IMPERSONATE_SA=<sa-email>     (act as a service account)
 */

import { google } from "googleapis";
import { getAuthClient, mustEnv } from "./auth.js";
import { promises as dns } from "node:dns";
import { setTimeout as sleep } from "node:timers/promises";

const SITE_URL = mustEnv("SITE_URL");
const HOSTNAME = new URL(SITE_URL).hostname;
const CF_API_TOKEN = process.env.CLOUDFLARE_API_TOKEN?.trim();
const CF_ZONE_NAME = process.env.CF_ZONE_NAME?.trim() ?? deriveZone(HOSTNAME);

function deriveZone(host: string): string {
  const parts = host.split(".");
  return parts.length < 2 ? host : parts.slice(-2).join(".");
}

interface CfDnsRecord {
  id: string;
  name: string;
  type: string;
  content: string;
}

async function cfApi<T>(
  path: string,
  init: RequestInit & { body?: string } = {},
): Promise<T> {
  if (!CF_API_TOKEN) {
    throw new Error("CLOUDFLARE_API_TOKEN required for CF API call");
  }
  const resp = await fetch(`https://api.cloudflare.com/client/v4${path}`, {
    ...init,
    headers: {
      authorization: `Bearer ${CF_API_TOKEN}`,
      "content-type": "application/json",
      ...(init.headers ?? {}),
    },
  });
  const json = (await resp.json()) as {
    success: boolean;
    errors?: { code: number; message: string }[];
    result: T;
  };
  if (!resp.ok || !json.success) {
    const detail = json.errors?.map((e) => `${e.code}: ${e.message}`).join("; ") ?? resp.statusText;
    throw new Error(`Cloudflare API ${path} failed: ${detail}`);
  }
  return json.result;
}

async function findOrCreateTxtRecord(zoneId: string, name: string, value: string) {
  const existing = await cfApi<CfDnsRecord[]>(
    `/zones/${zoneId}/dns_records?type=TXT&name=${encodeURIComponent(name)}`,
  );
  const match = existing.find((r) => r.content === value || r.content === `"${value}"`);
  if (match) {
    console.log(`  Cloudflare: TXT record already present (id=${match.id})`);
    return match;
  }
  const created = await cfApi<CfDnsRecord>(`/zones/${zoneId}/dns_records`, {
    method: "POST",
    body: JSON.stringify({ type: "TXT", name, content: value, ttl: 60 }),
  });
  console.log(`  Cloudflare: TXT record created (id=${created.id})`);
  return created;
}

async function pollForDnsTxt(host: string, expected: string, attempts = 30) {
  for (let i = 0; i < attempts; i += 1) {
    try {
      const records = await dns.resolveTxt(host);
      const flat = records.map((rr) => rr.join(""));
      if (flat.some((r) => r === expected)) {
        console.log(`  DNS: TXT record visible after ${i + 1} attempt(s)`);
        return;
      }
    } catch (err) {
      const code = (err as NodeJS.ErrnoException).code;
      if (code !== "ENOTFOUND" && code !== "ENODATA") throw err;
    }
    await sleep(10_000);
  }
  throw new Error(`Timed out waiting for TXT record on ${host}`);
}

async function main() {
  const auth = await getAuthClient();
  const verification = google.siteVerification({ version: "v1", auth });

  console.log(`Fetching DNS_TXT_RECORD verification token for ${HOSTNAME}…`);
  const tokenResp = await verification.webResource.getToken({
    requestBody: {
      site: { type: "INET_DOMAIN", identifier: HOSTNAME },
      verificationMethod: "DNS_TXT_RECORD",
    },
  });
  const txtValue = tokenResp.data.token;
  if (!txtValue) {
    throw new Error("siteVerification.webResource.getToken returned no token");
  }
  console.log(`  Token: ${txtValue}`);

  if (CF_API_TOKEN) {
    console.log(`Resolving Cloudflare zone "${CF_ZONE_NAME}"…`);
    const zones = await cfApi<{ id: string; name: string }[]>(
      `/zones?name=${encodeURIComponent(CF_ZONE_NAME)}`,
    );
    if (zones.length === 0) {
      throw new Error(`Cloudflare zone not found: ${CF_ZONE_NAME}`);
    }
    const zoneId = zones[0]!.id;
    console.log(`  zone id: ${zoneId}`);

    await findOrCreateTxtRecord(zoneId, HOSTNAME, txtValue);
    console.log(`Waiting for DNS propagation…`);
    await pollForDnsTxt(HOSTNAME, txtValue);
  } else {
    console.log(
      `\nCLOUDFLARE_API_TOKEN not set — add the TXT record manually:\n` +
        `  Name : ${HOSTNAME}   (apex; the zone editor will display "@")\n` +
        `  Type : TXT\n` +
        `  Value: ${txtValue}\n` +
        `Then re-run this script with the same env. It is idempotent — if the\n` +
        `record is already in place the script proceeds to the verify call.\n`,
    );
    console.log(`Polling DNS for the TXT record…`);
    await pollForDnsTxt(HOSTNAME, txtValue);
  }

  console.log(`Calling siteVerification.webResource.insert…`);
  const inserted = await verification.webResource.insert({
    verificationMethod: "DNS_TXT_RECORD",
    requestBody: {
      site: { type: "INET_DOMAIN", identifier: HOSTNAME },
    },
  });
  console.log(`  ✓ verified: ${inserted.data.id ?? "(no id returned)"}`);

  console.log(
    `\nDone. Final manual step (UI only — there is no API for this):\n` +
      `  1. Open https://search.google.com/search-console/users?resource_id=sc-domain%3A${encodeURIComponent(HOSTNAME)}\n` +
      `  2. Add the deploy service-account email as an Owner.\n` +
      `  3. Submit the sitemap once: https://search.google.com/search-console/sitemaps\n` +
      `     (or wait — the post-deploy script will resubmit it on the next push to main).\n`,
  );
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
