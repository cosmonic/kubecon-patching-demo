// This file is generated automatically using wasmcloud/weld-codegen 0.4.3

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

/// wasmbus.contractId: cosmonic:xkcd
/// wasmbus.providerReceive
#[async_trait]
pub trait Xkcd {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "cosmonic:xkcd"
    }
    async fn get_comic(&self, ctx: &Context, arg: &u32) -> RpcResult<Vec<u8>>;
}

/// XkcdReceiver receives messages defined in the Xkcd service trait
#[doc(hidden)]
#[async_trait]
pub trait XkcdReceiver: MessageDispatch + Xkcd {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "GetComic" => {
                let value: u32 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U32': {}", e)))?;
                let resp = Xkcd::get_comic(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Xkcd.GetComic",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Xkcd::{}",
                message.method
            ))),
        }
    }
}

/// XkcdSender sends messages to a Xkcd service
/// client for sending Xkcd messages
#[derive(Debug)]
pub struct XkcdSender<T: Transport> {
    transport: T,
}

impl<T: Transport> XkcdSender<T> {
    /// Constructs a XkcdSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl XkcdSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Xkcd provider
    /// implementing the 'cosmonic:xkcd' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("cosmonic:xkcd", "default").unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Xkcd provider
    /// implementing the 'cosmonic:xkcd' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("cosmonic:xkcd", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Xkcd for XkcdSender<T> {
    #[allow(unused)]
    async fn get_comic(&self, ctx: &Context, arg: &u32) -> RpcResult<Vec<u8>> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Xkcd.GetComic",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Vec<u8> = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Blob", e)))?;
        Ok(value)
    }
}
