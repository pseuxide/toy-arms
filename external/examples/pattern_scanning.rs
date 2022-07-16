use toy_arms::{ VirtualKeyCode };
use toy_arms::external::process::Process;

const DW_FORCE_ATTACK_PATTERN: &str = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";

fn main() {
    let mut once = false;

    // Getting process information
    let process = Process::from_process_name("csgo.exe").unwrap();

    // You can get module information by using get_client
    let mut client = process.get_module_info("client.dll").unwrap();

    let address = client.find_pattern(DW_FORCE_ATTACK_PATTERN);
    match address {
        Some(i) => println!("found *dwForceAttack pattern at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    let offset = client.pattern_scan::<u32>(
        DW_FORCE_ATTACK_PATTERN,
        2,
        0,
    );
    match offset {
        Some(i) => println!("found dwForceAttack offset at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }
        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
