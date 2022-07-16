use internal::pattern_scan_all_modules;
use toy_arms::keyboard::VirtualKeyCode;

internal::create_entrypoint!(hack_main);

fn hack_main() {
    let mut once = false;

    let dw_force_attack_pattern = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";
    match pattern_scan_all_modules(dw_force_attack_pattern) {
        Some(e) => {
            println!("address: {:x}", e.0);
            println!("module name: {}", e.1);
        }
        _ => println!("Not Found"),
    }

    // let random_pattern = "55 48 8B EC 48 81 EC B0 00 00 00 48 89 4D 98";
    // let result =
    //     toy_arms::internal::pattern_scan_specific_range(random_pattern, 0x10000, 0x300000)
    //         .unwrap();
    // println!("{:p}", result);
    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }
        if toy_arms::keyboard::detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
