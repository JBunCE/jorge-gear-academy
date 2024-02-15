#![no_std]

use gstd::{codec::*, ActorId, String};
use gmeta::{In, InOut, Metadata, Out};
use scale_info::TypeInfo;

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
    pub slept_block: u64
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmAction {
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
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