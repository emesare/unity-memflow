use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;

pub mod game;
pub mod unity;

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
    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()
        .unwrap();
    let proc_info = kernel
        .process_info("EscapeFromTarkov.exe")
        .expect("tarkov not started");
    let mut process = Win32Process::with_kernel(kernel, proc_info);

    let module = process.module_info("UnityPlayer.dll").unwrap();
    println!("UnityPlayer.dll found... {:?}", module.base); 

    let mut virt_mem = process.virt_mem;

    let gom_address: Address = virt_mem.virt_read_addr(module.base + 0x156C698).unwrap();
    let mut gom: unity::GameObjectManager = virt_mem.virt_read(gom_address).unwrap();
    println!("{:?}", gom);
    
    let game_world = gom.find_active(&mut virt_mem, "GameWorld").unwrap();
    println!("{:?}", game_world); 
    
    // you aint from michegan if you never done dis befo
    let world_chain_0 = virt_mem.virt_read_addr(game_world_ptr + 0x30).unwrap();
    let world_chain_1 = virt_mem.virt_read_addr(world_chain_0 + 0x18).unwrap();
    let world_chain_2 = virt_mem.virt_read_addr(world_chain_1 + 0x28).unwrap();

    let mut local_game_world: game::LocalGameWorld = virt_mem.virt_read(world_chain_2).unwrap();
    println!("local_game_world={:?}", local_game_world);

    // this might be incorrect, i never can remember if i got this working correct.
    println!(
        "profile_id={:?}",
        local_game_world.profile_id.to_string(&mut virt_mem)
    );
}
