#![no_std]
use gstd::{codec::*, ActorId, ReservationId, String, Vec};
use gmeta::{In, InOut, Metadata, Out};
use scale_info::TypeInfo;
use store_io::{AttributeId, TransactionId};

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
        ammount: u128,
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
    type Init = In<String>;
    type Handle = InOut<TmAction, TmEvent>;
    type State = Out<Tamagochi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}