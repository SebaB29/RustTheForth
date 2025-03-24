pub struct Stack {
    elements: Vec<i16>,
    max_size: usize,
}

impl Stack {
    pub fn new(max_size_in_bytes: usize) -> Self {
        let max_size: usize = max_size_in_bytes / 2;
        Stack {
            elements: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, value: i16) {
        if self.len() + 1 < self.max_size {
            self.elements.push(value);
        } else {
            println!("La pila esta llena.");
        }
    }

    pub fn pop(&mut self) -> Option<i16> {
        if !self.is_empty() {
            self.elements.pop()
        } else {
            println!("La pila está vacía.");
            Some(0)
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}