use std::mem::size_of;
use winapi::shared::minwindef::LPVOID;
use toy_arms::external::read;
use toy_arms::external::module::Module;
use toy_arms::external::process::Process;
use toy_arms::external::error::TAExternalError::ReadMemoryFailed;

const DW_LOCAL_PLAYER: u32 = 0xDB35DC;

fn main() {
    let csgo: Process;
    match Process::from_process_name("csgo.exe") {
        Ok(p) => csgo = p,
        Err(e) => {
            println!("{}", e);
            return;
        },
    }
    let client: Module;
    match csgo.get_module_info("client.dll") {
        Ok(m) => client = m,
        Err(e) => {
            println!("{}", e);
            return;
        },
    }

    println!("module_base: {:x}", client.module_base_address);
    println!("localplayer pointer: 0x{:x}", client.module_base_address + DW_LOCAL_PLAYER as usize);

    let mut localplayer: u32 = 0;
    let ok = read::<u32>(&csgo.process_handle, client.module_base_address + DW_LOCAL_PLAYER as usize, size_of::<LPVOID>(), localplayer as *mut u32);

    match ok {
        Ok(ok) => {
            println!("localplayer address: 0x{:x}", localplayer);
            let mut health = 0;
            // 0x100 is the offset of the health in player entity class.
            let ok2 = read::<u16>(&csgo.process_handle, localplayer as usize + 0x100, size_of::<u16>(), health as *mut u16);
            match ok2 {
                // This is what we wanted.
                Ok(h) => println!("localplayer's health: {}", health),
                Err(ReadMemoryFailed(e)) => println!("{}", e),
                Err(_) => println!("some error"),
            }
        },
        Err(e) => println!("error: {}", e),
    }
}