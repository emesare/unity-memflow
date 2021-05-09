use memflow::connector::*;
use memflow_win32::prelude::*;

pub fn main() {
    let connector_name = "qemu_procfs";
    let connector_args = "Windowstest";

    // create inventory + connector
    let inventory = unsafe { ConnectorInventory::scan() };
    let connector = unsafe {
        inventory.create_connector(
            &connector_name,
            &ConnectorArgs::parse(&connector_args).unwrap(),
        )
    }
    .unwrap();

    // initialize kernel
    let _kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()
        .unwrap();

    
}
