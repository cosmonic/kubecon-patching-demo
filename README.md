# Patching Demo for KubeCon EU 2022

This is the demo code for our patching example in our talk "Disrupting the Downtime Continuum."

To run this example, simply follow the wasmCloud [installation](https://wasmcloud.dev/overview/installation/) guide to install prerequisites and then run:

```shell
make run
```

After `make run`, navigate to `localhost:4000` in your browser and use the wasmCloud dashboard to start the `xkcd` actor wasm and `xkcd-provider` par.gz files from a file. They are located under `./xkcd/build/xkcd_s.wasm` and `xkcd-provider/build/xkcd_provider.par.gz` respectively.