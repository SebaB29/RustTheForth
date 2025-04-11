/// Estructura que representa una pila de enteros de 16 bits (i16).
///
/// La pila tiene un tamaño máximo en elementos determinado por el tamaño de memoria especificado
/// al momento de la creación.
pub struct Stack {
    elements: Vec<i16>,
    max_size: usize,
}

impl Stack {
    /// Crea una nueva pila con un tamaño máximo especificado en bytes.
    ///
    /// # Argumentos
    ///
    /// * `max_size_in_bytes` - El tamaño máximo de la pila en bytes.
    ///
    /// # Retornos
    ///
    /// Devuelve una instancia de `Stack` con capacidad para almacenar `max_size_in_bytes / 2` elementos.
    pub fn new(max_size_in_bytes: usize) -> Self {
        let max_size: usize = max_size_in_bytes / 2;
        Stack {
            elements: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Agrega un valor a la pila si no se ha alcanzado el tamaño máximo.
    ///
    /// Si la pila ya está llena, no se agrega el valor y se muestra un mensaje de error en la consola.
    ///
    /// # Argumentos
    ///
    /// * `value` - El valor que se quiere agregar a la pila.
    pub fn push(&mut self, value: i16) {
        if self.len() < self.max_size {
            self.elements.push(value);
        } else {
            println!("stack-overflow");
        }
    }

    /// Elimina y devuelve el último valor de la pila.
    ///
    /// Si la pila está vacía, devuelve `None`.
    ///
    /// # Retornos
    ///
    /// Devuelve `Some(i16)` con el valor que fue eliminado de la pila, o `None` si la pila está vacía.
    pub fn pop(&mut self) -> Option<i16> {
        self.elements.pop()
    }

    /// Obtiene la cantidad de elementos actuales en la pila.
    ///
    /// # Retornos
    ///
    /// Devuelve el número de elementos en la pila.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
