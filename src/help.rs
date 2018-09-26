pub fn get_help() -> Vec<String> {
    let mut messages: Vec<String> = Vec::new();
    messages.push("goto program usability".to_string());
    messages.push("goto [pattern]".to_string());
    return messages;
}

#[cfg(test)]
mod help_should {
    use super::*;

    #[test]
    fn return_collection_of_strings_with_text () {
        let result = get_help();
        assert_ne!(result.len(), 0);
    }
}
