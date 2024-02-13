#![no_std]
use gstd::{async_main, debug, exec, msg, prelude::*, ReservationId};
use io::{Tamagochi, TmAction, TmEvent};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{StoreAction, StoreEvent};

static mut TAMAGOCHI: Option<Tamagochi> = None;
static mut INIT_BLOCK: u64 = 0;
static mut TRANSACTIONS_COUNT: u64 = 0;

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
            approved_account: None,
            ft_contract_id: Default::default(),
            transaction_id: Default::default(),
            approve_transacion: None,
            reservations: Default::default(),
        }) 
    }
}

#[async_main]
async fn main() {
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
        TmAction::SetFTokenContract(contract_id) => {
            tamagochi.ft_contract_id = contract_id;
            let _ = msg::reply(TmEvent::FTokenContractSet, 0);
        },
        TmAction::ApproveTokens { account, ammount } => {
            let transaction_id: u64 = unsafe { TRANSACTIONS_COUNT.checked_add(1) }.unwrap();

            let res = msg::send_for_reply_as::<_, FTokenEvent>(
                tamagochi.ft_contract_id,
                FTokenAction::Message { 
                    transaction_id, 
                    payload:  LogicAction::Approve { 
                        approved_account: account, 
                        amount: ammount 
                    }
                }, 0, 0,
            ).expect("Error during sending the message").await;

            match res {
                Ok(FTokenEvent::Ok) => {
                    tamagochi.approve_transacion = Some((transaction_id, account, ammount));
                    let _ = msg::reply(TmEvent::TokensApproved { account, ammount }, 0);
                },
                Ok(_) | Err(_) => {
                    let _ = msg::reply(TmEvent::ApprovalError, 0);
                },
            }
        },
        TmAction::BuyAttribute { store_id, attribute_id } => {
            let res = msg::send_for_reply_as::<_, StoreEvent>(
                store_id,
                StoreAction::BuyAttribute { attribute_id },
                0, 0,
            ).expect("Error during sending the message").await;

            match res {
                Ok(StoreEvent::AttributeSold { success: _ }) => {
                    let _ = msg::reply(TmEvent::AttributeBought { attribute_id }, 0);
                },
                Ok(StoreEvent::CompletePrevTx { attribute_id: _ }) => {
                    let _ = msg::reply(TmEvent::CompletePrevPurchase, 0);
                },
                Ok(_) | Err(_) => {
                    let _ = msg::reply(TmEvent::ErrorDuringPurchase, 0);
                },
            }
        },
        TmAction::CheckState => {
            if tamagochi.reservations.is_empty() {
                let _ = msg::send(tamagochi.owner, TmEvent::MakeReservation, 0).expect("Error during sending the message");
            } else {
                let reservation_id = tamagochi.reservations.pop().unwrap();

                if tamagochi.fed < 10 {
                    let _ = msg::send_from_reservation(
                        reservation_id, 
                        tamagochi.owner, 
                        TmEvent::FeedMe, 0,
                    ).expect("Error during sending the message");
                }

                if tamagochi.entertained < 10 {
                    let _ = msg::send_from_reservation(
                        reservation_id, 
                        tamagochi.owner, 
                        TmEvent::PlayWithMe, 0,
                    ).expect("Error during sending the message");
                }

                if tamagochi.slept < 10 {
                    let _ = msg::send_from_reservation(
                        reservation_id, 
                        tamagochi.owner, 
                        TmEvent::WantToSleep, 0,
                    ).expect("Error during sending the message");
                }

                let _ = msg::send_delayed_from_reservation(
                    reservation_id, 
                    exec::program_id(), 
                    TmAction::CheckState,
                    0, 15,
                ).expect("Error during sending the message");
            }
        },
        TmAction::ReserveGas { reservation_amount, duration } => {
            let reservation_id = ReservationId::reserve(reservation_amount, duration)
            .expect("Error during reservation");
            tamagochi.reservations.push(reservation_id);
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = tamagochi_mut();
    let _ = msg::reply(state, 0);
}
