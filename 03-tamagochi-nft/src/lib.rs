#![no_std]
use gstd::{debug, exec, msg, prelude::*};
use io::{Tamagochi, TmAction, TmEvent};

static mut TAMAGOCHI: Option<Tamagochi> = None;
static mut INIT_BLOCK: u64 = 0;

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64  = 2;
const ENERGY_PER_BLOCK: u64  = 2;
const FILL_PER_FEED: u64  = 1000;
const FILL_PER_ENTERNAINMENT: u64  = 1000;
const FILL_PER_SLEEP: u64  = 1000;

pub fn tamagochi_mut() -> &'static mut Tamagochi {
    let tamagochi = unsafe { TAMAGOCHI.as_mut() };
    return unsafe { tamagochi.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let tamagochi_name: String = msg::load().expect("Can't decode the init message");
    unsafe { INIT_BLOCK = exec::block_height() as u64 }

    debug!("The program was initialized with the following tamagochi name: {:?}", tamagochi_name);

    unsafe { 
        TAMAGOCHI = Some(Tamagochi {
            name: tamagochi_name, 
            date_of_birth: exec::block_timestamp(),
            owner: msg::source(),
            fed: Default::default(),
            fed_block: INIT_BLOCK,
            entertained: Default::default(),
            entertained_block: INIT_BLOCK,
            slept: Default::default(),
            slept_block: INIT_BLOCK,
            approved_account: None
        }) 
    }
}

#[no_mangle]
extern "C" fn handle() {
    let message: TmAction = msg::load().expect("Can't decode the incoming message");
    let tamagochi = tamagochi_mut();
    let current_block = exec::block_height() as u64;
    let blocks = unsafe { current_block - INIT_BLOCK };
    unsafe { INIT_BLOCK = current_block }

    let _ = tamagochi.fed.checked_sub(blocks * HUNGER_PER_BLOCK);
    let _ = tamagochi.entertained.checked_sub(blocks * BOREDOM_PER_BLOCK);
    let _ = tamagochi.slept.checked_sub(blocks * ENERGY_PER_BLOCK);

    match message {
        TmAction::Name => {
            let _ = msg::reply(TmEvent::Name(tamagochi_mut().name.to_string()), 0);
        },
        TmAction::Age => {
            let _ = msg::reply(TmEvent::Age(tamagochi_mut().date_of_birth), 0);
        },
        TmAction::Feed => {
            let _ = tamagochi.fed.checked_add(FILL_PER_FEED);
            tamagochi.fed_block = current_block;
            let _ = msg::reply(TmEvent::Fed, 0);
        }
        TmAction::Entertain => {
            let _ = tamagochi.entertained.checked_add(FILL_PER_ENTERNAINMENT);
            tamagochi.entertained_block = current_block;
            let _ = msg::reply(TmEvent::Entertained, 0);
        },
        TmAction::Sleep => {
            let _ = tamagochi.slept.checked_add(FILL_PER_SLEEP);
            tamagochi.slept_block = current_block;
            let _ = msg::reply(TmEvent::Slept, 0);
        },
        TmAction::Transfer(actor) => {
            tamagochi.owner = actor;
            let _ = msg::reply(TmEvent::Transferred(actor), 0);
        },
        TmAction::Approve(actor) => {
            tamagochi.approved_account = Some(actor);
            let _ = msg::reply(TmEvent::Approved(actor), 0);
        },
        TmAction::RevokeApproval => {
            tamagochi.approved_account = None;
            let _ = msg::reply(TmEvent::ApprovalRevoked, 0);
        },
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = tamagochi_mut();
    let _ = msg::reply(state, 0);
}
