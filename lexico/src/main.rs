use std::fs::File;
use std::io::prelude::*;

/* 

Declarações de constantes que serão usadas ao longo
do analisador. 

*/

const ATRIBUICAO: [&str; 1] = [":="];
const SIMBOLOS: [char; 4] = ['{', '}', '_', ' '];
const DELIMITADORES: [char; 6] = ['.', ',', ':', ';', '(', ')'];
const OP_ADITIVOS: [&str; 3] = ["+", "-", "or"];
const OP_MULTIPLICATIVOS: [&str; 3] = ["*", "/", "and"];
const OP_RELACIONAIS: [&str; 6] = [">", "<", "=", ">=", "<=", "<>"];
const BOOLEANOS: [&str; 2] = ["true", "false"];
const TIPOS: [&str; 3] = ["boolean", "integer", "real"];
const NUMEROS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const ALFABETO: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const PALAVRAS_RESERVADAS: [&str; 12] = ["program", "var", "procedure", "function", "begin", "end", "if", "then", "else", "while", "do", "not"];

/* 

Declarações dos caminhos dos arquivos de teste. 

*/

const PATH: &str = "src/benchmark-arquivos_testes/Test1.pas";
// const PATH: &str = "src/benchmark-arquivos_testes/Test2.pas";
// const PATH: &str = "src/benchmark-arquivos_testes/Test3.pas";
// const PATH: &str = "src/benchmark-arquivos_testes/Test4.pas";
// const PATH: &str = "src/benchmark-arquivos_testes/Test5.pas";

/* 

Leitor de arquivos de texto. 

*/

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

}