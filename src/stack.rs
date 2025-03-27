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
        if self.len() < self.max_size {
            self.elements.push(value);
        } else {
            println!("La pila esta llena.");
        }
    }

    pub fn pop(&mut self) -> Option<i16> {
        self.elements.pop()
    }

    pub fn peek(&mut self) -> Option<i16> {
        self.elements.last().copied()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
