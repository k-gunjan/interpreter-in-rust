use crate::types::{ByteCode, Program, ProgramError, Result, Variable};

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
///    // load 2
///    LoadVal(2),
///    // write y = 2
///    WriteVar('y'),
///    // read x = 1
///    ReadVar('x'),
///    // load 1
///    LoadVal(1),
///    // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
///    Add,
///    // read y = 2
///    ReadVar('y'),
///    // multiply -> 2 * 2 = 4 (new value in stack)
///    Mul,
///    // Return the result
///    Return,
///   ];
///
///   assert_eq!(
///    interpret(test_arithmetic_written_values).unwrap().value,
///    4,
///    "not interpreted properly"
///   );
///   ```
pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
    let mut code = Program {
        bytecodes,
        stack: Vec::new(),
    };

	// iterate and match the bytecode vector
    for op in code.bytecodes {
        if let Some(err) = match op {
            ByteCode::LoadVal(i) => {
                code.stack.push(Variable {
                    variable: None,
                    value: i,
                });
                None
            },
            ByteCode::WriteVar(c) => {
                let loaded_value = code.stack.pop();
                if let Some(v) = loaded_value {
                    code.stack.push(Variable {
                        variable: Some(c),
                        value: v.value,
                    })
                }
                None
            },
            ByteCode::ReadVar(c) => {
                let read_value = code.stack.iter().find(|&&x| x.variable == Some(c));
                if let Some(v) = read_value {
                    let var = v.clone();
                    code.stack.push(Variable {
                        variable: var.variable,
                        value: var.value,
                    })
                }
                None
            },
            ByteCode::Mul => do_op!(code, *),
			ByteCode::Div => do_op!(code, /),
            ByteCode::Add => do_op!(code, +),
			ByteCode::Sub => do_op!(code, -),
            ByteCode::Return => break,
        } {
            return Err(err);
        }
    }

    if let Some(v) = code.stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}