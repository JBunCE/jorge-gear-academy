use gtest::{Log, Program, System};
use tamagotchi_interaction_io::{TmgAction, TmgEvent};


const TAMAGOCHI_NAME: &str = "Tamgochi";

#[test]
fn interaction_test() {
    let system = System::new();
    system.init_logger();
    
    let program = init_tamagochi(&system);

    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Name(String::from(TAMAGOCHI_NAME)));

    assert!(res.contains(&expected_log));

    let res = program.send(2, TmgAction::Feed);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmgAction::Age);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Age(system.block_timestamp()));
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));
    
    let res = program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));
}

fn init_tamagochi(sys: &System) -> Program<'_> {
    let program = Program::current(sys);

    let res = program.send(2, String::from(TAMAGOCHI_NAME));

    assert!(!res.main_failed());

    return program;
}