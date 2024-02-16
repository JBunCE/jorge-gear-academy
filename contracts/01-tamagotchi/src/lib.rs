#![no_std]

use gstd::{debug, exec, ext::debug, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

pub fn tamagochi_mut() -> &'static mut Tamagotchi {
    let tamagochi = unsafe { TAMAGOTCHI.as_mut() };
    unsafe { tamagochi.unwrap_unchecked() }
}

#[no_mangle]
extern fn init() {
    let tamagotchi_name: String = msg::load().expect("Can't decode the init message");

    debug!("The program was initialized with the following tamagotchi name: {:?}", tamagotchi_name);

    unsafe {
        TAMAGOTCHI = Some(Tamagotchi {
            name: tamagotchi_name,
            date_of_birth: exec::block_timestamp(),
        })
    }
}

#[no_mangle]
extern fn handle() {
    let message: TmgAction = msg::load().expect("Can't decode the handle message");

    match message {
        TmgAction::Name => {
            let tamagotchi = tamagochi_mut();
            let _ = msg::reply(TmgEvent::Name(tamagotchi.name.to_string()), 0);
        }
        TmgAction::Age => {
            let tamagotchi = tamagochi_mut();
            let age = exec::block_timestamp() - tamagotchi.date_of_birth;
            let _ = msg::reply(TmgEvent::Age(age), 0);
        }
    }
}

#[no_mangle]
extern fn state() {
    let state = tamagochi_mut();
    let _ = msg::reply(state, 0);
}
