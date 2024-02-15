#![no_std]
use gstd::{codec::*, ActorId, CodeId, ReservationId, String, Vec};
use gmeta::{In, InOut, Metadata, Out};
use scale_info::TypeInfo;
use store_io::{AttributeId, TransactionId};

pub type TamagochiId = u64;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagochi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
    pub ft_contract_id: ActorId,
    pub transaction_id: u64,
    pub approve_transacion: Option<(TransactionId, ActorId, u128)>,
    pub reservations: Vec<ReservationId>
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FactoryAction {
    CreateTamagochi {
        name: String,
    },
    Name(TamagochiId),
    Age(TamagochiId),
    Feed(TamagochiId),
    Entertain(TamagochiId),
    Sleep(TamagochiId),
    Transfer(ActorId, TamagochiId),
    Approve(ActorId, TamagochiId),
    RevokeApproval(TamagochiId),
    SetFTokenContract(ActorId, TamagochiId),
    ApproveTokens {
        tamagochi_id: TamagochiId,
        account: ActorId,
        ammount: u128,
    },
    BuyAttribute {
        tamago_id: TamagochiId,
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    CheckState(TamagochiId),
    ReserveGas {
        tamagochi_id: TamagochiId,
        reservation_amount: u64,
        duration: u32,
    }

}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmAction {
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    CheckState,
    ReserveGas {
        reservation_amount: u64,
        duration: u32,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmEvent {
    TamagochiCreated {
        tamagochi_id: TamagochiId,
        tamagochi_address: ActorId,
    },
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    FTokenContractSet,
    TokensApproved {
        account: ActorId,
        ammount: u128,
    },
    ApprovalError,
    AttributeBought {
        attribute_id: AttributeId,
    },
    CompletePrevPurchase,
    ErrorDuringPurchase,
    FeedMe,
    PlayWithMe,
    WantToSleep,
    MakeReservation,
    GasReserved,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<CodeId>;
    type Handle = InOut<FactoryAction, TmEvent>;
    type State = Out<Tamagochi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}