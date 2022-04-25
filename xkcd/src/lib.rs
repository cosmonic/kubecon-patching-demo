use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::{NumberGen, NumberGenSender, RangeLimit};
use xkcd_interface::{Xkcd, XkcdSender};

const MAX_XKCD: u32 = 2600;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XkcdActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for XkcdActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        _req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let num: u32 = NumberGenSender::new()
            .random_in_range(ctx, &range(0, MAX_XKCD))
            .await?;

        let comic: Vec<u8> = XkcdSender::new().get_comic(ctx, &num).await?;

        Ok(HttpResponse {
            body: comic,
            status_code: 200,
            ..Default::default()
        })
    }
}

fn range(min: u32, max: u32) -> RangeLimit {
    RangeLimit { min, max }
}
