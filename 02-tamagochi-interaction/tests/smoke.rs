use gtest::{Log, Program, System};
use io::{TmAction, TmEvent};


const TAMAGOCHI_NAME: &str = "Tamgochi";

#[test]
fn smoke_test() {
    let system = System::new();
    system.init_logger();
    
    let program = init_tamagochi(&system);

    let res = program.send(2, TmAction::Name);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Name(String::from(TAMAGOCHI_NAME)));

    assert!(res.contains(&expected_log));

    let res = program.send(2, TmAction::Feed);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmAction::Age);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Age(system.block_timestamp()));
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmAction::Entertain);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Entertained);
    assert!(res.contains(&expected_log));
    
    let res = program.send(2, TmAction::Sleep);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Slept);
    assert!(res.contains(&expected_log));
}

fn init_tamagochi(sys: &System) -> Program {
    let program = Program::current(sys);

    let res = program.send(2, String::from(TAMAGOCHI_NAME));

    assert!(!res.main_failed());

    return program;
}