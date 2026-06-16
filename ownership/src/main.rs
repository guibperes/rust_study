fn main() {
    // Ownership
    // 1. Cada valor tem um dono (owner)
    // 2. Só pode haver um dono por vez
    // 3. Quando o dono sai de escopo, o valor é destruído (memória liberada automaticamente)
    fn display() {
        let name = String::from("Guilherme");
        println!("name: {}", name);
    }
    // name sai do escopo, memória liberada
    display();

    fn move_name() {
        let name = String::from("Guilherme");
        let text = name; // Transfere owner de name para text, name não existe mais
        println!("{}", text);
    }
    move_name();

    fn simple_type() {
        // Tipos primitivos implementam copy, são copiados ao invés de movidos
        let x = 10;
        let y = x;
        println!("{} - {}", x, y); // Ambos válidos
    }
    simple_type();

    fn clone() {
        // Cópia profunda explícita com .clone()
        let name = String::from("Ana");
        let same_name = name.clone();
        println!("{} & {}", name, same_name);
    }
    clone();

    fn borrowing() {
        // Mover tudo seria impraticável. Borrowing permite usar o valor sem tomar posse dele
        // Referência com &
        fn calc_size(s: &String) -> usize {
            s.len()
        } // s saí do escopo mas não destrói o valor (não tem ownership)

        let text = String::from("Um texto qualquer...");
        println!("text: {}\nsize: {}", text, calc_size(&text));
    }
    borrowing();

    fn ref_mut_imut() {
        let mut text = String::from("Guilherme");

        // Várias referências imutáveis simultâneas: permitido
        let r1 = &text;
        let r2 = &text;
        println!("{} - {} - {}", text, r1, r2);

        // Referência mutável: apenas uma por vez
        let r3 = &mut text;
        r3.push_str(" Beidaki");
        println!("{}", text);
    }
    ref_mut_imut();
}
