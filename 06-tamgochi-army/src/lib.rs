#![no_std]
use gstd::{async_main, debug, exec, msg, prelude::*, CodeId, ReservationId};
use io::{FactoryAction, Tamagochi, TmAction, TmEvent};
use factory::TamagochiFactory;

pub mod factory;

static mut TAMAGOCHI_FACTORY: Option<TamagochiFactory> = None;

pub fn tamagochi_factory_mut() -> &'static mut TamagochiFactory {
    let tamagochi_factory = unsafe { TAMAGOCHI_FACTORY.as_mut() };
    return unsafe { tamagochi_factory.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let tamagochi_code_id: CodeId = msg::load().expect("Can't decode the init message");

    let tamagochi_factory = TamagochiFactory {
        tamagochi_code_id,
        ..Default::default()
    };

    unsafe { 
        TAMAGOCHI_FACTORY = Some(tamagochi_factory);
    }
}

#[async_main]
async fn main() {
    let message: FactoryAction = msg::load().expect("Can't decode the incoming message");
    let factory = tamagochi_factory_mut();

    match message {
        FactoryAction::CreateTamagochi { name } => {
            factory.create_tamagochi(name).await;
        }
        FactoryAction::Name(tamagochi_id) => {
            factory.tamagochi_name(tamagochi_id).await;
        }
        FactoryAction::Age(tamagochi_id) => {
            factory.tamagochi_age(tamagochi_id).await;
        }
        FactoryAction::Feed(tamagochi_id) => {
            factory.feed(tamagochi_id).await;
        }
        FactoryAction::Entertain(tamagochi_id) => {
            factory.entertain(tamagochi_id).await;
        }
        FactoryAction::Sleep(tamagochi_id) => {
            factory.sleep(tamagochi_id).await;
        }
        FactoryAction::Transfer(account, tamagochi_id) => {
            factory.transfer(account, tamagochi_id).await;
        }
        FactoryAction::Approve(account, tamagochi_id) => {
            factory.approve(account, tamagochi_id).await;
        }
        FactoryAction::RevokeApproval(tamagochi_id) => {
            factory.revoke_approval(tamagochi_id).await;
        }
        FactoryAction::SetFTokenContract(ft_contract_id, tamagochi_id) => {
            factory.set_ft_contract(ft_contract_id, tamagochi_id).await;
        }
        FactoryAction::ApproveTokens { tamagochi_id, account, ammount } => {
            factory.approve_tokens(tamagochi_id, account, ammount).await;
        }
        FactoryAction::BuyAttribute { tamago_id, store_id, attribute_id } => {
            factory.buy_attribute(tamago_id, store_id, attribute_id).await;
        }
        FactoryAction::CheckState(tamagochi_id) => {
            factory.check_state(tamagochi_id).await;
        }
        FactoryAction::ReserveGas { tamagochi_id, reservation_amount, duration } => {
            factory.reserve_gas(tamagochi_id, reservation_amount, duration).await;
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = tamagochi_factory_mut();
    let _ = msg::reply(state, 0);
}
