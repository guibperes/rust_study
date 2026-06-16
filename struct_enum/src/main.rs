use std::f64::consts::PI;
use std::num::ParseIntError;

// Atríbutos
struct Retangulo {
    largura: i32,
    altura: i32,
}

// Métodos
impl Retangulo {
    fn area(&self) -> i32 {
        self.largura * self.altura
    }

    fn redimensionar(&mut self, fator: i32) {
        self.largura *= fator;
        self.altura *= fator;
    }

    // Método associado (static) sem self
    fn quadrado(lado: i32) -> Retangulo {
        Retangulo { largura: lado, altura: lado }
    }
}

// Cada variante do enum pode carregar dados.
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(side, height) => side * height,
            Shape::Triangle { base, height } => base * height / 2.0,
        }
    }
}

fn main() {
    let square = Retangulo::quadrado(4);
    println!("Área do quadrado: {}", square.area());

    let mut rect = Retangulo { largura: 10, altura: 20 };
    println!("Área do retângulo: {}", rect.area());

    rect.redimensionar(2);
    println!("Área do retângulo: {}", rect.area());

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(5.0, 5.0);
    let triangle = Shape::Triangle { base: 10.0, height: 5.0 };

    println!("Área do Círculo: {}", circle.area());
    println!("Área do Retângulo: {}", rectangle.area());
    println!("Área do Triângulo: {}", triangle.area());

    // Option para lidar com valores null. Não existe null em Rust.
    fn divide(x: f64, y: f64) -> Option<f64> {
        if y == 0.0 { None } else { Some(x / y) }
    }

    let result = divide(10.0, 0.0);
    match result {
        Some(value) => println!("Resultado da divisão: {}", value),
        None => println!("Divisão por zero"),
    }

    // Result para tratar Erros.
    fn parse_int(string: &str) -> Result<i32, ParseIntError> {
        string.trim().parse()
    }

    match parse_int("42") {
        Ok(number) => println!("Número: {}", number),
        Err(error) => println!("Erro: {}", error),
    }
}
