//! xkcd-provider capability provider
//!
//!
use wasmbus_rpc::provider::prelude::*;
use xkcd_interface::{Xkcd, XkcdReceiver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(XkcdProviderProvider::default())?;

    eprintln!("xkcd-provider provider exiting");
    Ok(())
}

/// xkcd-provider capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Xkcd)]
struct XkcdProviderProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for XkcdProviderProvider {}
impl ProviderHandler for XkcdProviderProvider {}

#[async_trait]
impl Xkcd for XkcdProviderProvider {
    async fn get_comic(&self, _ctx: &Context, arg: &u32) -> RpcResult<Vec<u8>> {
        let url = format!("https://xkcd.com/{}/info.0.json", arg);
        let resp = reqwest::get(url)
            .await
            .map_err(|e| RpcError::Other(format!("Unable to perform get request: {}", e)))?;

        if !resp.status().is_success() {
            return Err(RpcError::InvalidParameter(format!(
                "Invalid response received. Got status code {}",
                resp.status()
            )));
        }

        let info: XkcdMetadata = resp
            .json()
            .await
            .map_err(|e| RpcError::Other(format!("Unable to deserialize response: {}", e)))?;
        // Extract the 'title' and 'img' fields from the json response,
        // and build html page
        let html = format!(
            r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Your XKCD comic</title>
        </head>
        <body>
            <h1>{}</h1>
            <img src="{}"/>
        </body>
        </html>
        "#,
            &info.title, &info.img
        );

        Ok(html.into_bytes())
    }
}

/// Metadata returned as json
/// (this is a subset of the full metadata, but we only need two fields)
#[derive(serde::Deserialize)]
struct XkcdMetadata {
    title: String,
    img: String,
}
