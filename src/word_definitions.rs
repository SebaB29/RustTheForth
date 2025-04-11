use std::collections::HashMap;

pub type WordMap = HashMap<String, Vec<String>>;

/// Define una nueva palabra en el mapa de palabras del usuario.
///
/// Esta función toma los tokens que siguen al símbolo `:` e interpreta el nombre
/// y el cuerpo de la nueva palabra. Si el nombre es un número o si no se encuentra
/// el símbolo `;` al final de la definición, se devuelve un error.
///
/// # Parámetros
///
/// * `tokens` - Iterador sobre los tokens restantes de la entrada.
/// * `word_map` - Mapa que almacena las palabras definidas por el usuario.
///
/// # Retorna
///
/// * `Ok(())` si la definición fue exitosa.
/// * `Err(String)` si ocurre un error en la definición.
pub fn handle_word_definition(
    tokens: &mut std::str::SplitWhitespace,
    word_map: &mut WordMap,
) -> Result<(), String> {
    let name = parse_word_name(tokens)?;
    let definition = parse_word_body(tokens, word_map)?;
    word_map.insert(name, definition);
    Ok(())
}

/// Parsea y valida el nombre de una nueva palabra.
///
/// # Parámetros
///
/// * `tokens` - Iterador sobre los tokens restantes de la entrada.
///
/// # Retornos
///
/// * `Ok(String)` con el nombre en mayúsculas si es válido.
/// * `Err(String)` si no se proporciona un nombre o si el nombre es un número.
fn parse_word_name(tokens: &mut std::str::SplitWhitespace) -> Result<String, String> {
    let name = tokens
        .next()
        .ok_or("Error: Se esperaba un nombre para la palabra")?
        .to_uppercase();

    if name.parse::<i16>().is_ok() {
        return Err("invalid-word".to_string());
    }

    Ok(name)
}

/// Parsea el cuerpo de una nueva palabra definida por el usuario.
///
/// # Parámetros
///
/// * `tokens` - Iterador sobre los tokens restantes de la entrada.
/// * `word_map` - Mapa que contiene las palabras ya definidas.
///
/// # Retorna
///
/// * `Ok(Vec<String>)` con los tokens expandidos del cuerpo.
/// * `Err(String)` si no se encuentra el símbolo `;` al final.
fn parse_word_body(
    tokens: &mut std::str::SplitWhitespace,
    word_map: &WordMap,
) -> Result<Vec<String>, String> {
    let mut definition = Vec::new();

    for token in tokens {
        if token == ";" {
            return Ok(definition);
        }

        let token_up = token.to_uppercase();
        if let Some(existing_def) = word_map.get(&token_up) {
            definition.extend(existing_def.iter().cloned());
        } else {
            definition.push(token.to_string());
        }
    }

    Err("Error: Se esperaba ';' al final de la definición".to_string())
}
