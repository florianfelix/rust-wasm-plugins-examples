use super::bindings::acme::plugins::host;

use wasmtime_wasi::{
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView},
    ResourceTable,
};

//
// Host
//

/// Plugins host.
#[derive()]
pub struct Host {
    wasi: WasiCtx,
    resources: ResourceTable,
}

impl Host {
    /// Constructor.
    pub fn new() -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdout().build();
        Self { wasi, resources: ResourceTable::new() }
    }
}

// We need to implement WasiView for wasmtime_wasi::add_to_linker_sync
impl WasiView for Host {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl IoView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

// Our exposed Host functions
impl host::Host for Host {
    fn log(&mut self, message: String) {
        println!("log: {}", message);
    }
}
