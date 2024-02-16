#![no_std]

use gstd::{debug, exec, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_interaction_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;
static mut INIT_BLOCK: u64 = 0;

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64  = 2;
const ENERGY_PER_BLOCK: u64  = 2;
const FILL_PER_FEED: u64  = 1000;
const FILL_PER_ENTERNAINMENT: u64  = 1000;
const FILL_PER_SLEEP: u64  = 1000;

pub fn tamagochi_mut() -> &'static mut Tamagotchi {
    let tamagochi = unsafe { TAMAGOTCHI.as_mut() };
    unsafe { tamagochi.unwrap_unchecked() }
}

#[no_mangle]
extern fn init() {
    let tamagochi_name: String = msg::load().expect("Can't decode the init message");
    unsafe { INIT_BLOCK = exec::block_height() as u64 }

    debug!("The program was initialized with the following tamagochi name: {:?}", tamagochi_name);

    unsafe { 
        TAMAGOTCHI = Some(Tamagotchi {
            name: tamagochi_name, 
            date_of_birth: exec::block_timestamp(),
            owner: msg::source(),
            fed: Default::default(),
            fed_block: INIT_BLOCK,
            entertained: Default::default(),
            entertained_block: INIT_BLOCK,
            slept: Default::default(),
            slept_block: INIT_BLOCK
        }) 
    }
}

#[no_mangle]
extern fn handle() {
    let message: TmgAction = msg::load().expect("Can't decode the incoming message");
    let tamagochi = tamagochi_mut();
    let current_block = exec::block_height() as u64;
    let blocks = unsafe { current_block - INIT_BLOCK };
    unsafe { INIT_BLOCK = current_block }

    let _ = tamagochi.fed.checked_sub(blocks * HUNGER_PER_BLOCK);
    let _ = tamagochi.entertained.checked_sub(blocks * BOREDOM_PER_BLOCK);
    let _ = tamagochi.slept.checked_sub(blocks * ENERGY_PER_BLOCK);

    match message {
        TmgAction::Name => {
            drop(msg::reply(TmgEvent::Name(tamagochi_mut().name.to_string()), 0));
        },
        TmgAction::Age => {
            drop(msg::reply(TmgEvent::Age(tamagochi_mut().date_of_birth), 0));
        },
        TmgAction::Feed => {
            let _ = tamagochi.fed.checked_add(FILL_PER_FEED);
            tamagochi.fed_block = current_block;
            drop(msg::reply(TmgEvent::Fed, 0));
        }
        TmgAction::Entertain => {
            let _ = tamagochi.entertained.checked_add(FILL_PER_ENTERNAINMENT);
            tamagochi.entertained_block = current_block;
            drop(msg::reply(TmgEvent::Entertained, 0));
        },
        TmgAction::Sleep => {
            let _ = tamagochi.slept.checked_add(FILL_PER_SLEEP);
            tamagochi.slept_block = current_block;
            drop(msg::reply(TmgEvent::Slept, 0));
        },
    }
}

#[no_mangle]
extern fn state() {
    drop(msg::reply(tamagochi_mut(), 0));
}
