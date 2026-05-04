//! Page-level SEO helpers.
//!
//! Centralizes the `<head>` metadata required by `specs/seo.md`:
//!
//! - `<title>` (with the " — MoonMath" suffix where appropriate)
//! - `<meta name="description">`
//! - `<link rel="canonical">`
//! - Open Graph tags (`og:title`, `og:description`, `og:type`, `og:url`,
//!   `og:site_name`, `og:image`)
//! - Twitter card tags (`summary_large_image` + title/description/image)
//! - Optional `<script type="application/ld+json">` payload for showcase
//!   articles.
//!
//! All pages call exactly one of [`PageMeta`] or [`ArticleMeta`] from inside
//! their root component.

use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// Production hostname. Keep in sync with `moonmath-ssg`'s
/// `DEFAULT_BASE_URL` — the SSG-emitted sitemap.xml and the
/// runtime-rendered canonical / og:url tags must agree.
pub const SITE_BASE_URL: &str = "https://moonmath.openhackers.club";
pub const SITE_NAME: &str = "MoonMath";
pub const DEFAULT_OG_IMAGE: &str = "/og-default.svg";

fn absolute(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        path.to_string()
    } else if path.starts_with('/') {
        format!("{}{}", SITE_BASE_URL, path)
    } else {
        format!("{}/{}", SITE_BASE_URL, path)
    }
}

/// Render the standard `<head>` metadata for a navigational / index-style
/// page. Use [`ArticleMeta`] for showcase detail pages instead.
#[component]
pub fn PageMeta(
    /// Bare page title, e.g. "Showcase". The component appends
    /// " — MoonMath" automatically unless `with_suffix` is `false`.
    title: String,
    /// Used for `<meta name="description">` and `og:description`.
    description: String,
    /// Path part of the canonical URL (e.g. `"/showcase"`). Must start
    /// with `/`.
    path: String,
    /// `og:type` value. Defaults to `"website"`.
    #[prop(default = "website".into())]
    og_type: String,
    /// Append " — MoonMath" to `title`. Defaults to `true`.
    #[prop(default = true)]
    with_suffix: bool,
) -> impl IntoView {
    let full_title = if with_suffix {
        format!("{} — {}", title, SITE_NAME)
    } else {
        title.clone()
    };
    let canonical = absolute(&path);
    let og_image = absolute(DEFAULT_OG_IMAGE);

    view! {
        <Title text=full_title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=canonical.clone()/>

        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:type" content=og_type/>
        <Meta property="og:url" content=canonical.clone()/>
        <Meta property="og:site_name" content=SITE_NAME.to_string()/>
        <Meta property="og:image" content=og_image.clone()/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=og_image/>
    }
}

/// Render the `<head>` metadata for a showcase detail page. Emits the same
/// `<title>`/OG/Twitter tags as [`PageMeta`] plus a JSON-LD `TechArticle`
/// payload populated from the page's frontmatter.
#[component]
pub fn ArticleMeta(
    title: String,
    description: String,
    /// Path part of the canonical URL (e.g.
    /// `"/showcase/number-theory/prime-theorem"`). Must start with `/`.
    path: String,
    /// Tags from frontmatter (joined with `", "` for the `keywords`
    /// JSON-LD field).
    tags: Vec<String>,
    /// `frontmatter.date` if present, else build-day in `YYYY-MM-DD`.
    /// Surfaces as JSON-LD `datePublished`.
    date_published: String,
) -> impl IntoView {
    let full_title = format!("{} — {}", title, SITE_NAME);
    let canonical = absolute(&path);
    let og_image = absolute(DEFAULT_OG_IMAGE);
    let keywords = tags.join(", ");

    let json_ld = build_article_json_ld(
        &title,
        &description,
        &canonical,
        &og_image,
        &date_published,
        &keywords,
    );

    view! {
        <Title text=full_title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=canonical.clone()/>

        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:type" content="article"/>
        <Meta property="og:url" content=canonical.clone()/>
        <Meta property="og:site_name" content=SITE_NAME.to_string()/>
        <Meta property="og:image" content=og_image.clone()/>
        {(!keywords.is_empty()).then(|| view! {
            <Meta property="article:tag" content=keywords.clone()/>
        })}

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=og_image/>

        // JSON-LD via inner_html so braces are not interpreted as Leptos
        // expression delimiters.
        <script type="application/ld+json" inner_html=json_ld></script>
    }
}

/// Render the JSON-LD payload as a string. Public so SSG and tests can
/// reuse it — keeps the schema in one place.
pub fn build_article_json_ld(
    title: &str,
    description: &str,
    canonical: &str,
    image: &str,
    date_published: &str,
    keywords: &str,
) -> String {
    // Hand-rolled JSON to avoid pulling serde_json into the WASM bundle
    // for what amounts to a fixed shape. Inputs are escaped with
    // `escape_json_string`.
    let mut s = String::with_capacity(512);
    s.push_str("{\"@context\":\"https://schema.org\",\"@type\":\"TechArticle\"");
    push_field(&mut s, "headline", title);
    push_field(&mut s, "description", description);
    s.push_str(",\"mainEntityOfPage\":{\"@type\":\"WebPage\",\"@id\":\"");
    s.push_str(&escape_json_string(canonical));
    s.push_str("\"}");
    s.push_str(",\"author\":{\"@type\":\"Organization\",\"name\":\"Open Hackers Club\",\"url\":\"https://github.com/OpenHackersClub\"}");
    s.push_str(",\"publisher\":{\"@type\":\"Organization\",\"name\":\"");
    s.push_str(SITE_NAME);
    s.push_str("\",\"url\":\"");
    s.push_str(SITE_BASE_URL);
    s.push_str("\"}");
    push_field(&mut s, "image", image);
    push_field(&mut s, "datePublished", date_published);
    if !keywords.is_empty() {
        push_field(&mut s, "keywords", keywords);
    }
    s.push_str(",\"isAccessibleForFree\":true");
    s.push('}');
    s
}

fn push_field(buf: &mut String, key: &str, value: &str) {
    buf.push_str(",\"");
    buf.push_str(key);
    buf.push_str("\":\"");
    buf.push_str(&escape_json_string(value));
    buf.push('"');
}

fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            // Closing `</script>` would break out of the JSON-LD block;
            // escape the slash so the browser still parses it as data.
            '/' => out.push_str("\\/"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_ld_has_required_fields() {
        let payload = build_article_json_ld(
            "Infinitude of Primes",
            "Euclid's proof",
            "https://moonmath.openhackers.club/showcase/number-theory/prime-theorem",
            "https://moonmath.openhackers.club/og-default.svg",
            "2026-05-01",
            "lean4-proof, number-theory",
        );
        assert!(payload.contains("\"@type\":\"TechArticle\""));
        assert!(payload.contains("\"headline\":\"Infinitude of Primes\""));
        assert!(payload.contains("\"datePublished\":\"2026-05-01\""));
        assert!(payload.contains("\"keywords\":\"lean4-proof, number-theory\""));
        assert!(payload.contains("\"isAccessibleForFree\":true"));
    }

    #[test]
    fn json_ld_escapes_quotes_and_slashes() {
        let payload = build_article_json_ld(
            "He said \"hi\"",
            "ends with </script>",
            "https://moonmath.openhackers.club/x",
            "https://moonmath.openhackers.club/og-default.svg",
            "2026-05-01",
            "",
        );
        assert!(payload.contains("\\\"hi\\\""));
        // Closing tag must be escaped so the browser doesn't break out.
        assert!(payload.contains("<\\/script>"));
        assert!(!payload.contains("</script>"));
    }

    #[test]
    fn absolute_handles_paths_and_full_urls() {
        assert_eq!(
            absolute("/showcase"),
            "https://moonmath.openhackers.club/showcase"
        );
        assert_eq!(
            absolute("https://example.com/x"),
            "https://example.com/x"
        );
        assert_eq!(absolute("relative"), "https://moonmath.openhackers.club/relative");
    }
}
