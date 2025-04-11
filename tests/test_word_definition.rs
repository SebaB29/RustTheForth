use forth_interpreter::word_definitions::{WordMap, handle_word_definition};

#[cfg(test)]
mod word_definition_tests {
    use super::*;

    #[test]
    fn test_define_simple_word() {
        let mut word_map = WordMap::new();
        let input = ": FOO 1 2 ;";
        let mut tokens = input.split_whitespace();
        tokens.next();
        assert!(handle_word_definition(&mut tokens, &mut word_map).is_ok());

        let expected = vec!["1", "2"];
        assert_eq!(
            word_map.get("FOO"),
            Some(&expected.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_define_with_existing_word() {
        let mut word_map = WordMap::new();

        let input = ": FOO 1 2 ;";
        let mut tokens = input.split_whitespace();
        tokens.next();
        assert!(handle_word_definition(&mut tokens, &mut word_map).is_ok());

        let input2 = ": BAR FOO 3 ;";
        let mut tokens = input2.split_whitespace();
        tokens.next();
        assert!(handle_word_definition(&mut tokens, &mut word_map).is_ok());

        let expected: Vec<String> = vec!["1", "2", "3"].iter().map(|s| s.to_string()).collect();
        assert_eq!(word_map.get("BAR"), Some(&expected));
    }

    #[test]
    fn test_redefine_number_should_fail() {
        let mut word_map = WordMap::new();
        let input = ": 1 2 ;";
        let mut tokens = input.split_whitespace();
        tokens.next();
        let result = handle_word_definition(&mut tokens, &mut word_map);

        assert_eq!(result, Err("invalid-word".to_string()));
    }

    #[test]
    fn test_missing_semicolon_should_fail() {
        let mut word_map = WordMap::new();
        let input = "FOO 1 2";
        let mut tokens = input.split_whitespace();
        let result = handle_word_definition(&mut tokens, &mut word_map);

        assert_eq!(
            result,
            Err("Error: Se esperaba ';' al final de la definición".to_string())
        );
    }
}
