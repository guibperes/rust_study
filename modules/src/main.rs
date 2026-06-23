// Crates: menor unidade que o compilador processa. Pode ser interna ou externa (instalada com cargo)
// Library crate: produz código reutilizável, não tem função main. cargo new project
// Binary crate: produz código executável, tem função main. cargo new project --lib

// Módulos: declarados com `mod nome;` (aponta para arquivo) ou inline
// São acessados com paths: crate::modulo::item
// São trazidos ao escopo com `use`
// visibilidade: privado (padrão), pub(super), pub(crate), pub
mod math;

fn main() {
    println!("Sum: {}", math::sum(100, 50));
    println!("Area: {}", math::area(10.0));
}
