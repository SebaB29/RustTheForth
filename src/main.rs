struct Stack {
    elements: Vec<i16>,
    max_size: usize,
}

impl Stack {
    fn new(max_size_in_bytes: usize) -> Self {
        let max_size: usize = max_size_in_bytes / 2;
        Stack {
            elements: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, value: i16) {
        if self.len() + 1 < self.max_size {
            self.elements.push(value);
        } else {
            println!("La pila esta llena.");
        }
    }

    fn pop(&mut self) -> Option<i16> {
        if !self.is_empty() {
            self.elements.pop()
        } else {
            println!("La pila está vacía.");
            Some(0)
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn peek(&self) -> Option<&i16> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new(128*1024);

    stack.push(20);
    stack.push(10);

    apply_operation(&mut stack, "+");
    // apply_operation(&mut stack, "-");
    // apply_operation(&mut stack, "*");
    // apply_operation(&mut stack, "/");

    println!("El último elemento de la pila es: {:?}", stack.peek());
}

fn apply_operation(stack: &mut Stack, operator: &str) {
    if stack.len() < 2 {
        println!("Operadores Insuficientes.");
        return;
    }

    match operator {
        "+" => apply_sum(stack),
        "-" => apply_subtraction(stack),
        "*" => apply_multiplication(stack),
        "/" => apply_division(stack),
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn apply_sum(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b + a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_subtraction(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b - a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_multiplication(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b * a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_division(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b / a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
