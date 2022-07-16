/*
This example is the demonstration of getting player health with toy-arms internal memory analysis feature.
Make sure that you inject this image to csgo.exe.
also, the offset of DW_LOCAL_PLAYER works as of the day i wrote this but it might not be up to date in your case.
*/
use internal::{cast, VirtualKeyCode};
use internal::module::Module;
use internal::GameObject;
use toy_arms_derive::GameObject;

internal::create_entrypoint!(hack_main_thread);

// This macro provides from_raw() func that ensures the base address is not null.
#[derive(GameObject)]
struct LocalPlayer {
    pointer: *const usize, // Denote the base address of LocalPlayer to use it later in get_health() function.
}

impl LocalPlayer {
    unsafe fn get_health(&self) -> u16 { *cast!(self.pointer as usize + 0x100, u16) }
}

// This offset has to be up to date.
const DW_LOCAL_PLAYER: u32 = 0xDB35DC;

fn hack_main_thread() {
    let module = Module::from_name("client.dll").unwrap();
    unsafe {
        //let dw_local_player = memory.read_mut::<LocalPlayer>(0xDA244C);
        loop {
            if let Some(i) = LocalPlayer::from_raw(module.read(DW_LOCAL_PLAYER as usize)) {
                println!("health = {:?}", (*i).get_health());
            };
            if toy_arms::keyboard::detect_keypress(VirtualKeyCode::VK_INSERT) {
                break;
            }
        }
    }
}
