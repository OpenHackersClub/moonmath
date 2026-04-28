/// Create a Resource that fetches JSON from a URL.
///
/// On the client (WASM), uses gloo-net. On the server, reads from filesystem.
/// Wraps the future in SendWrapper to satisfy Resource::new's Send requirement.
pub fn json_resource<S, T>(
    source: impl Fn() -> S + Send + Sync + 'static,
    url_fn: impl Fn(S) -> String + Send + Sync + Clone + 'static,
) -> leptos::prelude::Resource<Result<T, leptos::prelude::ServerFnError>>
where
    S: Clone + Send + Sync + PartialEq + 'static,
    T: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Clone + 'static,
{
    leptos::prelude::Resource::new(source, move |s| {
        let url = url_fn(s);
        send_wrapper::SendWrapper::new(fetch_json_inner::<T>(url))
    })
}

/// Create a Resource with no source param (unit source).
pub fn json_resource_once<T>(
    url: &str,
) -> leptos::prelude::Resource<Result<T, leptos::prelude::ServerFnError>>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Clone + 'static,
{
    let url = url.to_string();
    leptos::prelude::Resource::new(
        || (),
        move |_| {
            let url = url.clone();
            send_wrapper::SendWrapper::new(fetch_json_inner::<T>(url))
        },
    )
}

async fn fetch_json_inner<T: serde::de::DeserializeOwned>(
    url: String,
) -> Result<T, leptos::prelude::ServerFnError> {
    #[cfg(feature = "hydrate")]
    {
        let resp = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;

        if !resp.ok() {
            return Err(leptos::prelude::ServerFnError::new(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.status_text()
            )));
        }

        resp.json::<T>()
            .await
            .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // SSG data lives in target/ssg-data/ (outside target/site/ which cargo-leptos wipes).
        // URLs are like "/data/showcase/categories.json" → "target/ssg-data/showcase/categories.json"
        let rel = url.strip_prefix("/data").unwrap_or(&url);
        let file_path = format!("target/ssg-data{}", rel);
        let data = std::fs::read_to_string(&file_path)
            .map_err(|e| leptos::prelude::ServerFnError::new(format!("Failed to read {}: {}", file_path, e)))?;
        serde_json::from_str(&data)
            .map_err(|e| leptos::prelude::ServerFnError::new(format!("Failed to parse {}: {}", file_path, e)))
    }
}
