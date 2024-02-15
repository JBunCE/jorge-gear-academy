use alloc::collections::BTreeMap;
use gstd::{msg, prog::ProgramGenerator, ActorId, CodeId, String};
use io::{FactoryAction, TamagochiId, TmAction, TmEvent};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use store_io::AttributeId;

extern crate alloc;

const GAS_FOR_CREATION: u64 = 1_000_000_000;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct TamagochiFactory {
    pub tamagochi_number: TamagochiId,
    pub id_to_address: BTreeMap<TamagochiId, ActorId>,
    pub tamagochi_code_id: CodeId,
}

impl TamagochiFactory {

    pub async fn create_tamagochi(&mut self, name: String) {

        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply(
            self.tamagochi_code_id, 
            FactoryAction::CreateTamagochi { name }.encode(), 
            GAS_FOR_CREATION, 
            0, 
            0
        ).expect("Can't create a new tamagochi")
        .await.expect("Program was not initialized");
        
        self.tamagochi_number = self.tamagochi_number.saturating_add(1);
        self.id_to_address.insert(self.tamagochi_number, address);
        
        msg::reply(
            TmEvent::TamagochiCreated { 
                tamagochi_id: self.tamagochi_number, 
                tamagochi_address: address 
            }, 
            0,
        ).expect("Error during a reply `FactoryEvent::ProgramCreated");

    }

    pub async fn tamagochi_name(&self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Name).await;
        
        match event {
            TmEvent::Name(_) => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        };
    }

    pub async fn tamagochi_age(&self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Age).await;

        match event {
            TmEvent::Age(_) => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn feed(&mut self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Feed).await;

        match event {
            TmEvent::Fed => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn entertain(&mut self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Entertain).await;

        match event {
            TmEvent::Entertained => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn sleep(&mut self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Sleep).await;

        match event {
            TmEvent::Slept => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn transfer(&mut self, new_owner: ActorId, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Transfer(new_owner)).await;

        match event {
            TmEvent::Transferred(_) => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn approve(&mut self, account: ActorId, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::Approve(account)).await;

        match event {
            TmEvent::Approved(_) => {
                msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn revoke_approval(&mut self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::RevokeApproval).await;

        match event {
            TmEvent::ApprovalRevoked => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn set_ft_contract(&mut self, f_token_contract_id: ActorId, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::SetFTokenContract(f_token_contract_id)).await;

        match event {
            TmEvent::FTokenContractSet => {
                let _ = msg::reply(event, 0).expect("Error during a reply");   
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn approve_tokens(&mut self, tamagochi_id: TamagochiId, account: ActorId, amount: u128) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::ApproveTokens { account, amount }).await;

        match event {
            TmEvent::TokensApproved {account, ammount} => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn buy_attribute(&mut self, tamagochi_id: TamagochiId, store_id: ActorId, attribute_id: AttributeId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::BuyAttribute { store_id, attribute_id }).await;

        match event {
            TmEvent::AttributeBought { attribute_id } => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn check_state(&mut self, tamagochi_id: TamagochiId) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::CheckState).await;
    }

    pub async fn reserve_gas(&mut self, tamagochi_id: TamagochiId, reservation_amount: u64, duration: u32) {
        let mut tamagochi_address = self.id_to_address.get(&tamagochi_id).expect("Tamagochi not found");
        let event = self.send_message_from_factory(tamagochi_address, TmAction::ReserveGas { reservation_amount, duration }).await;

        match event {
            TmEvent::GasReserved => {
                let _ = msg::reply(event, 0).expect("Error during a reply");
            }
            _ => panic!("Unexpected event"),
        }
    }

    pub async fn send_message_from_factory(&self, address: &ActorId, action: TmAction) -> TmEvent {
        return msg::send_for_reply_as::<_, TmEvent>(*address, action, msg::value(), 0)
        .expect("Error during sending the message from the factory")
        .await.expect("Unable to decode Event");
    }

}