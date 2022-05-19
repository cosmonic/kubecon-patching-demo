//! xkcd-provider capability provider
//!
//!
use wasmbus_rpc::provider::prelude::*;
use xkcd_interface::{Xkcd, XkcdReceiver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(XkcdProvider::default())?;

    eprintln!("xkcd-provider provider exiting");
    Ok(())
}

/// xkcd-provider capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Xkcd)]
struct XkcdProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for XkcdProvider {}
impl ProviderHandler for XkcdProvider {}

#[async_trait]
impl Xkcd for XkcdProvider {
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
            <title>Your xkcd comic</title>
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

        // NEW UPDATE HERE
        // Ok(r#"<!DOCTYPE html>
        // <html>
        // <head>
        //     <title>Your XKCD comic</title>
        // </head>
        // <body>
        //     <h1>I CAN HAZ YOUR CODEZ</h1>
        //     <img src="https://i.pinimg.com/originals/19/f5/2c/19f52c46a5617bda07453569586e9314.jpg"/>
        // </body>
        // </html>
        // "#.as_bytes().to_vec())
    }
}

/// Metadata returned as json
/// (this is a subset of the full metadata, but we only need two fields)
#[derive(serde::Deserialize)]
struct XkcdMetadata {
    title: String,
    img: String,
}
