use std::collections::HashMap;

pub type WordMap = HashMap<String, Vec<String>>;

pub fn handle_word_definition<'a>(
    _: &str,
    tokens: &mut impl Iterator<Item = &'a str>,
    word_map: &mut WordMap,
) -> Result<(), String> {
    let name = tokens
        .next()
        .ok_or("Error: Se esperaba un nombre para la palabra")?
        .to_uppercase();

    let mut definition = Vec::new();

    for t in tokens {
        if t == ";" {
            break;
        }
        definition.push(t.to_string());
    }

    word_map.insert(name, definition);
    Ok(())
}
