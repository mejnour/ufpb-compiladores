fn le_arquivo() -> String {
    let mut file = File::open(PATH)
        .expect("Não foi possível abrir o arquivo.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Não foi possível ler o arquivo.");

    return contents;
}

fn main() {

    let programa_input: String = le_arquivo();
    analisador_lexico(programa_input);
}