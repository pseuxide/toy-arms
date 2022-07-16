/*
This is the demonstration of how to use external feature of toy-arms.
Following code is trying to get process id and process handle first, then getting a value called dwClientState_state.
Then showing the way to overwrite value at dwForceAttack to make player shoot.
The offset DW_CLIENT_STATE, DW_CLIENT_STATE_STATE and DW_FORCE_ATTACK work as of the day i wrote this but it might not be up to date in your case.
*/

use toy_arms::VirtualKeyCode;
use toy_arms::external::process;
use toy_arms::external::{ read, write };

fn main() {
    // This const has to be up to date.
    const DW_CLIENT_STATE: u32 = 0x58CFC4;
    const DW_CLIENT_STATE_STATE: u32 = 0x108;
    const DW_FORCE_ATTACK: u32 = 0x31FE33C;
    // Getting process information
    let process = process::from_process_name("csgo.exe").unwrap();
    println!(
        "process id = {}, \nprocess handle = {:?}",
        process.process_id, process.process_handle
    );

    // You can get module information by using get_module_info
    let module_info = process.get_module_info("client.dll").unwrap();
    println!("{}", module_info.module_name);

    // read fetches the value at where the address is pointing.
    // U have to specify the type of the value with turbofish
    println!(
        "{:x}",
        read::<i32>(process.process_handle, read::<u32>(process.process_handle, process.get_module_base("engine.dll").unwrap() + DW_CLIENT_STATE).unwrap() + DW_CLIENT_STATE_STATE).unwrap()
    );

    loop {
        // write helps you tamper with the value.
            write::<u32>(
                process.process_handle,
                process.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize,
                &mut 0x5,
            )
            .unwrap();

        // Exit this loop by pressing INSERT
        if toy_arms::detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
