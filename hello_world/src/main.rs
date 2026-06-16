fn main() {
    // Variável por padrão é imutável
    let text = "Hello world!!!";

    // Variável mutável
    let mut x = 10;
    x += 10;

    println!("{} - {}", text, x);

    // Shadowing
    let text = "Other text!";
    println!("{}", text);

    // Inteiros: i8 até i128 e unsigned u8 até u128
    // Float: f64 e f32
    // Boolean: bool
    // Character: char - unicode 4 bytes
    let number: i32 = 100;
    let money: f32 = 199.99;
    let boolean: bool = true;
    let first_char: char = 'A';
    println!("{} - {} - {} - {}", number, money, boolean, first_char);

    // Tupla: tamanho fixo - tipos heterogêneos
    let tupla: (i32, f32, char) = (111, 10.99, 'Z');
    println!("[{},{},{}]", tupla.0, tupla.1, tupla.2);

    // Arrays: tamanho fixo - tipos homogêneos
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("{}", i);
    }

    let zeros: [i8; 10] = [0; 10]; // Array com 10 zeros
    for i in zeros.iter() {
        println!("{}", i);
    }

    fn sum(x: i32, y: i32) -> i32 {
        x+y // Última expressão sem ; é o valor de retorno
    }
    println!("Função de soma: {}", sum(5, 5));

    fn no_return (name: &str) {
        println!("Hello, {}", name);
    }
    no_return("Guilherme");

    if 10 == 10 {
        println!("Is ten...");
    } else {
        println!("Not is ten...");
    }

    for i in 0..5 { // Range exclusivo (até 4)
        println!("{}", i);
    }

    for i in 0..=5 { // Range inclusivo (até 5)
        println!("{}", i);
    }

    let mut count = 0;
    while count < 10 {
        count += 1;
    }

    count = 0;
    // Loop infinito com break e retorno de valor
    let result = loop {
        count += 1;
        if count == 10 { break count + 10; }
    };
    println!("Loop: {}", result);

    let text = String::from("Hello world");
    // Slice: referência para partes de coleções (sem owner)
    let hello = &text[0..5];
    let world = &text[6..11];
    println!("{} - {}", hello, world);
}
