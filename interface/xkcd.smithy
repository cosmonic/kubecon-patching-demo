// xkcd.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.cosmonic.interfaces.xkcd", crate: "xkcd" } ]

namespace org.cosmonic.interfaces.xkcd

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32

@wasmbus(
    contractId: "cosmonic:xkcd",
    providerReceive: true )
service Xkcd {
  version: "0.1",
  operations: [ GetComic ]
}

operation GetComic {
  input: U32,
  output: Blob
}

