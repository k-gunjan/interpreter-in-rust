use interpreter::interpret::interpret_with_loop;
use interpreter::types::ByteCode;

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn test_err_no_value_loaded() {
        use ByteCode::*;
        assert!(
            interpret_with_loop(vec![Return]).is_err(),
            "Should return StackUnderflow error"
        );
    }

    #[test]
    fn test_load_value_and_return() {
        use ByteCode::*;

        let test_load_value_and_return = vec![LoadVal(2), Return];

        assert_eq!(
            interpret_with_loop(test_load_value_and_return).unwrap().value,
            2,
            "Should return the chosen loaded value"
        );
    }

    #[test]
    fn test_load_two_values_multiply_return() {
        use ByteCode::*;

        let a = 2;
        let b = 4;

        let test_load_two_values_multiply_return = vec![LoadVal(a), LoadVal(b), Mul, Return];

        assert_eq!(
            interpret_with_loop(test_load_two_values_multiply_return)
                .unwrap()
                .value,
            a * b,
            "Should return {} * {} = {}",
            a,
            b,
            a * b
        );
    }

    #[test]
    fn test_load_two_values_divide_return() {
        use ByteCode::*;

        let a = 4;
        let b = 2;

        let test_load_two_values_multiply_return = vec![LoadVal(a), LoadVal(b), Div, Return];

        assert_eq!(
            interpret_with_loop(test_load_two_values_multiply_return)
                .unwrap()
                .value,
            a / b,
            "Should return {} / {} = {}",
            a,
            b,
            a / b
        );
    }

    #[test]
    fn test_load_two_values_subtract_return() {
        use ByteCode::*;

        let a = 4;
        let b = 2;

        let test_load_two_values_multiply_return = vec![LoadVal(a), LoadVal(b), Sub, Return];

        assert_eq!(
            interpret_with_loop(test_load_two_values_multiply_return)
                .unwrap()
                .value,
            a - b,
            "Should return {} - {} = {}",
            a,
            b,
            a - b
        );
    }

    #[test]
    fn test_write_value() {
        use ByteCode::*;

        let val = 2;
        let var = 'c';

        let test_write_value = vec![LoadVal(val), WriteVar(var), Return];
        let test_write_value_result = interpret_with_loop(test_write_value).unwrap();

        assert_eq!(
            test_write_value_result.variable,
            Some(var),
            "Should load value {} into variable {} -> this expression should resolve to Some({})",
            val,
            var,
            var
        );
        assert_eq!(
            test_write_value_result.value, val,
            "Should load value {} into variable {} -> this expression should resolve to {}",
            val, var, val
        );
    }

    #[test]
    fn test_arithmetic_written_values() {
        use ByteCode::*;

        let test_arithmetic_written_values = vec![
            // load 1
            LoadVal(1),
            // write x = 1
            WriteVar('x'),
            // load 2
            LoadVal(2),
            // write y = 2
            WriteVar('y'),
            // read x = 1
            ReadVar('x'),
            // load 1
            LoadVal(1),
            // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
            Add,
            // read y = 2
            ReadVar('y'),
            // multiply -> 2 * 2 = 4 (new value in stack)
            Mul,
            Return,
        ];

        assert_eq!(
            interpret_with_loop(test_arithmetic_written_values).unwrap().value,
            4,
            "not interpreted properly"
        );
    }


    #[test]
    fn test_loop_operation() {
        use ByteCode::*;

        let test_loop_operation = vec![
            // load 1
            LoadVal(1),
            // write x = 1
            WriteVar('x'),
            LoopVal(10),
            // read x = 1
            ReadVar('x'),
            // load 1
            LoadVal(1),
            // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
            Add,
            End,
            Return,
        ];

        assert_eq!(
            interpret_with_loop(test_loop_operation).unwrap().value,
            4,
            "not interpreted properly"
        );
    }
}