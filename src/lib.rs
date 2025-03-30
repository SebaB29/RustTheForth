/// Módulos principales de la implementación del intérprete de Forth.
///
/// # Módulos
///
/// - `arithmetic_operations`: Contiene las operaciones aritméticas básicas, como suma, resta, multiplicación y división.
/// - `boolean_operations`: Proporciona las operaciones lógicas para la comparación de valores y operaciones booleanas como AND, OR y NOT.
/// - `forth_basic_operations`: Contiene las operaciones básicas de Forth como `DUP`, `DROP`, `SWAP`, `OVER`, `ROT`.
/// - `stack`: Define y maneja la estructura de la pila y sus operaciones básicas como `push`, `pop` y `len`.
pub mod arithmetic_operations;
pub mod boolean_operations;
pub mod forth_basic_operations;
pub mod stack;
