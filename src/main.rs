use interpreter::interpret::interpret_with_loop1;
use interpreter::types::ByteCode;


fn main() {

    println!("__________________________________");

    test_loop_operation();
}

fn test_loop_operation() {
    use ByteCode::*;

    let test_loop_operation = vec![
        // load 1
        LoadVal(1),
        // write x = 1
        WriteVar('x'),
        LoopVal(5),
        // read x = 1
        ReadVar('y'),
        // load 1
        LoadVal(1),
        // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
        Add,
        End,
        Return,
    ];

    assert_eq!(
        interpret_with_loop1(test_loop_operation, &mut Vec::new(), 1).unwrap().value,
        10,
        "not interpreted properly"
    );
}
