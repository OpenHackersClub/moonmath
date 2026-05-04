import { google } from "googleapis";
import { GoogleAuth, Impersonated } from "google-auth-library";

export const SEARCH_CONSOLE_SCOPES = [
  "https://www.googleapis.com/auth/webmasters",
  "https://www.googleapis.com/auth/siteverification",
];

/**
 * Auth client compatible with `google.searchconsole({ auth })` /
 * `google.siteVerification({ auth })`. Returns the `GoogleAuth` instance
 * directly (it discovers ADC, picks the right credential type for both
 * GCE and `gcloud auth application-default login` environments) unless
 * `IMPERSONATE_SA` is set, in which case we wrap it in `Impersonated`
 * per the GCP-auth rule in the workspace `CLAUDE.md`.
 *
 * Local dev: `gcloud auth application-default login`, then optionally
 * `IMPERSONATE_SA=<sa-email> npm run …` to act as the production SA.
 *
 * CI: `google-github-actions/auth@v2` with Workload Identity Federation
 * provisions ADC for the SA directly — leave `IMPERSONATE_SA` unset.
 */
export async function getAuthClient(): Promise<GoogleAuth | Impersonated> {
  const sourceAuth = new google.auth.GoogleAuth({ scopes: SEARCH_CONSOLE_SCOPES });

  const impersonate = process.env.IMPERSONATE_SA?.trim();
  if (!impersonate) return sourceAuth;

  const sourceClient = await sourceAuth.getClient();
  return new Impersonated({
    sourceClient,
    targetPrincipal: impersonate,
    targetScopes: SEARCH_CONSOLE_SCOPES,
    lifetime: 3600,
  });
}

export function mustEnv(name: string): string {
  const v = process.env[name]?.trim();
  if (!v) throw new Error(`missing env var: ${name}`);
  return v;
}
