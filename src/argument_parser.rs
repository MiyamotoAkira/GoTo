pub struct Arguments {
    pattern: String
}

pub fn extract_arguments(args: Vec<String>) -> (bool, Arguments) {
    if args.len() <  2 {
        return (false, Arguments {pattern: "".to_string()})
    }

    (true, Arguments {pattern: args[1].clone()})
}

#[cfg(test)]
mod argument_parser_should {
    use super::*;

    #[test]
    fn return_false_when_no_arguments_are_passed() {
        let args: Vec<String> = Vec::new();
        let (result, _) = extract_arguments(args);
        assert_eq!(result, false);
    }

    #[test]
    fn return_a_filled_argument_stuct_when_a_single_non_mod_argument_is_passed() {
        let args = ["function_name", "pepe"].iter().map(|s| s.to_string()).collect();
        let (result, arguments) = extract_arguments(args);
        assert_eq!(result, true);
        assert_eq!(arguments.pattern, "pepe");
    }
}
