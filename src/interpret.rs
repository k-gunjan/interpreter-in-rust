use crate::types::{ByteCode, Program, ProgramError, Result, Variable, Loop};

macro_rules! do_op {
    ($code:expr, $op:tt) => {{
		// pop two last variables in the stack
        if let Some(a) = $code.stack.pop() {
            if let Some(b) = $code.stack.pop() {
				// push the result of the operation to stack
                $code.stack.push(Variable {
                    variable: None,
                    value: (b.value $op a.value),
                });
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

///    #test the interpreter
///    ```
///    use interpreter::interpret::interpret;
///    use interpreter::types::ByteCode::*;
///    let test_arithmetic_written_values = vec![
///    // load 1
///    LoadVal(1),
///    // write x = 1
///    WriteVar('x'),
///    LoopVal(10)
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
///    interpret_with_loop(test_arithmetic_written_values).unwrap().value,
///    11,
///    "not interpreted properly"
///   );
///   ```

pub fn interpret_with_loop(bytecodes: Vec<ByteCode>) -> Result<Variable> {
// pub fn interpret_with_loop(bytecodes: Vec<ByteCode>, times:u64 )-> Result<Variable> {
    let mut code = Program {
        bytecodes,
        stack: Vec::new(),
        in_loop:false,
        loop_op: Loop {
            bytecodes: Vec::new(),
            stack: Vec::new(),
            first_read: false,
            count: 0,
        },
    };

    // this for loop to iterate number of times it has to repeat in loop
    //
    // for rpt in 0..times {
	    // iterate and match the bytecode vector
        for op in code.bytecodes {
            if let Some(err) = match op {
                ByteCode::LoadVal(i) => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                    } else {
                        code.stack.push(Variable {
                            variable: None,
                            value: i,
                        });
                    }
                    None
                },
                ByteCode::WriteVar(c) => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                    } else {
                        let loaded_value = code.stack.pop();
                        if let Some(v) = loaded_value {
                            code.stack.push(Variable {
                                variable: Some(c),
                                value: v.value,
                            })
                        }
                    }
                    None
                },
                ByteCode::ReadVar(c) => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                    } else {
                        let read_value = code.stack.iter().find(|&&x| x.variable == Some(c));
                        if let Some(v) = read_value {
                            let var = v.clone();
                            code.stack.push(Variable {
                                variable: var.variable,
                                value: var.value,
                            })
                        }
                    }
                    None
                },
                ByteCode::LoopVal(i) => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                    } else {
                        code.in_loop = true;
                        code.loop_op.first_read= true;
                        code.loop_op.count= i;
                    }
                    None
                },
                ByteCode::End => {
                    if code.in_loop {
                        code.in_loop = false;
                        code.loop_op.first_read= false;
                        code.loop_op.count -= 1;
                        // // this will call itself recursively
                        // interpret_with_loop(code.loop_op.bytecodes, code.loop_op.count)
    
                    } else {
                        code.loop_op.bytecodes.push(op);
                    }
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
                    } else {
                        do_op!(code, +);
                    }
                    None
                },
	    		ByteCode::Sub => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                    } else {
                        do_op!(code, -);
                    }
                    None
                },
                ByteCode::Return => {
                    if code.in_loop {
                        code.loop_op.bytecodes.push(op);
                        None
                    } else {
                        break
                    }
                } }
                {
                return Err(err);
                }
            }
    // }
    if let Some(v) = code.stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}