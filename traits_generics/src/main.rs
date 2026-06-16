use std::fmt::Display;

// Sistema de polimorfismo do Rust.
trait Animal {
    fn describe(&self) -> String;

    // Implementação padrão.
    fn print(&self) {
        println!("{}", self.describe());
    }
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn describe(&self) -> String {
        format!("Cachorro chamado {}", self.name)
    }
}

impl Animal for Cat {
    fn describe(&self) -> String {
        format!("Gato chamado {}", self.name)
    }
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display> Pair<T> {
    fn print(&self) {
        println!("{} and {}", self.first, self.second);
    }
}

fn main() {
    let dog = Dog { name: "Nina".to_string() };
    let cat = Cat { name: "Katty".to_string() };

    dog.print();
    cat.print();

    let pair: Pair<i8> = Pair { first: 1, second: 3 };
    pair.print();
}
