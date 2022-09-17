use crate::types::{ByteCode, Program, ProgramError, Result, Variable, Loop};

macro_rules! do_op {
    ($code:expr, $op:tt) => {{
		// pop two last variables in the stack
        if let Some(a) = $code.stack.pop() {
            if let Some(b) = $code.stack.pop() {
				// push the result of the operation to stack
                $code.stack.push(Variable {
                    variable: b.variable,//  None,
                    value: (b.value $op a.value),
                });
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}


///    #test the interpreter
///    ```
///    use interpreter::interpret::interpret_with_loop1;
///    use interpreter::types::ByteCode::*;
///    let test_arithmetic_written_values = vec![
///    // load 1
///    LoadVal(1),
///    // write x = 1
///    WriteVar('x'),
///    LoopVal(10),
///    // read x = 1
///    ReadVar('x'),
///    // load 1
///    LoadVal(1),
///    // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
///    Add,
///    End,
///    Return,
///    ];
///
///   assert_eq!(
///    interpret_with_loop1(test_arithmetic_written_values, &mut Vec::new(), 1).unwrap().value,
///    11,
///    "not interpreted properly"
///   );
///   ```

pub fn interpret_with_loop1(bytecodes: Vec<ByteCode>, stack: &mut Vec<Variable>,  times:u64 )-> Result<Variable> {
        println!("bytecodes:{:?}", bytecodes);
        let mut code = Program {
            bytecodes,
            stack: stack.to_vec(), //: Vec::new(),
            in_loop:false,
            loop_op: Loop {
                bytecodes: Vec::new(),
                count: 0,
            },
        };
    
        // this for loop to iterate number of times it has to repeat in loop
        //
        for _rpt in 0..times {
            println!("rpt is:{}. . stack:{:?}", _rpt,  code.stack);
            // iterate and match the bytecode vector
            for op in code.bytecodes.clone() {
                if let Some(err) = match op {
                    ByteCode::LoadVal(i) => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            code.stack.push(Variable {
                                variable: None,
                                value: i,
                            });
                        // }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        None
                    },
                    ByteCode::WriteVar(c) => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            let loaded_value = code.stack.pop();
                            if let Some(v) = loaded_value {
                                code.stack.push(Variable {
                                    variable: Some(c),
                                    value: v.value,
                                })
                            }
                        // }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        None
                    },
                    ByteCode::ReadVar(c) => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            let read_value = code.stack.iter().rev().find(|&&x| x.variable == Some(c));
                            if let Some(v) = read_value {
                                // // remove the existing entry in the vec
                                // let index = code.stack.iter().position(|&x| x.variable == Some(c)).unwrap();
                                // code.stack.remove(index);
                                let var = v.clone();
                                code.stack.push(Variable {
                                    variable: var.variable,
                                    value: var.value,
                                })
                            }
                            else {
                                code.stack.push(Variable {
                                    variable: Some(c),
                                    value: 0,
                                })
                            }
                        // }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        None
                    },
                    ByteCode::LoopVal(i) => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            code.in_loop = true;
                            code.loop_op.count= i;
                        // }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        println!("loop started_____________");
                        None
                    },
                    ByteCode::End => {
                        if code.in_loop {
                            println!("loop ends here _____________");
                            code.in_loop = false;
                            code.loop_op.count -= 1;
                            if code.loop_op.count >0 {

                                // // this will call itself recursively
                                println!("evaluation of loop operations started_____________");
                                let r = interpret_with_loop1(code.loop_op.bytecodes.clone(), &mut code.stack, code.loop_op.count);
                                println!("evaluation of loop operations ends now _____________");
                                match r {
                                    Ok(v) => code.stack.push(v),
                                    Err(e) => {println!("{:?}", e)},
                                }
                            }
                            
        
                        } else {
                            code.loop_op.bytecodes.push(op);
                            println!("____________not expected_________");
                        }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        None
                    },
                    ByteCode::Mul => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } else {
                            do_op!(code, *);
                        }
                        None
                    },
                    ByteCode::Div => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } else {
                            do_op!(code, /);
                        }
                        None
                    },
                    ByteCode::Add => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            do_op!(code, +);
                        // }
                        println!("bytecode is:{:?}. . stack:{:?}", op,  code.stack);
                        None
                    },
                    ByteCode::Sub => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                        } 
                        // else {
                            do_op!(code, -);
                        // }
                        None
                    },
                    ByteCode::Return => {
                        if code.in_loop {
                            code.loop_op.bytecodes.push(op);
                            // None
                        } 
                        // else {
                            break
                        // }
                    } 
                }
                    {
                    return Err(err);
                    }
            }
        }
        if let Some(v) = code.stack.pop() {
            Ok(v)
        } else {
            Err(ProgramError::StackUnderflow)
        }
    }