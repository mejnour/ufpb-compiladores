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

fn analisador_lexico(conteudo: String) {

    /* 
    
    Cria um vetor (array simples em outras langs) para guardar
    um char por elemento. 
    
    */

    let mut vec_conteudo = Vec::new();

    for carac in conteudo.chars() {
        vec_conteudo.push(carac);
    }

    /* 
    
    Começa de fato a análise. 
    
    */

    let mut coluna = 0;
    let mut linha = 1;
    let mut cordao = String::from("");
    let mut acabou: bool = false;

    let mut vec_tabela = Vec::<(String, String, u32)>::new();

    loop {
        let err_catch = || -> Result<(), ()> {
            if ALFABETO.contains(&vec_conteudo[coluna]) {
                cordao.push(vec_conteudo[coluna]);
                coluna = coluna + 1;

                loop {
                    if ALFABETO.contains(&vec_conteudo[coluna]) || NUMEROS.contains(&vec_conteudo[coluna]) || vec_conteudo[coluna] == '_' {
                        cordao.push(vec_conteudo[coluna]);
                        coluna = coluna + 1;
                    } else {
                        if PALAVRAS_RESERVADAS.contains(&&cordao[..]) {
                            vec_tabela.push((cordao, String::from("Palavra Reservada"), linha));
                        } else if TIPOS.contains(&&cordao[..]) {
                            vec_tabela.push((cordao, String::from("Tipo"), linha));
                        } else if BOOLEANOS.contains(&&cordao[..]) {
                            vec_tabela.push((cordao, String::from("Bool"), linha));
                        } else if cordao == "or" {
                            vec_tabela.push((cordao, String::from("Op Aditivo"), linha));
                        } else if cordao == "and" {
                            vec_tabela.push((cordao, String::from("Op Multiplicativo"), linha));
                        } else {
                            vec_tabela.push((cordao, String::from("Identificador"), linha));
                        }
                        
                        break;
                    }
                }
            } else if NUMEROS.contains(&vec_conteudo[coluna]) {
                cordao.push(vec_conteudo[coluna]);
                coluna = coluna + 1;

                loop {
                    if NUMEROS.contains(&vec_conteudo[coluna]) {
                        cordao.push(vec_conteudo[coluna]);
                        coluna = coluna + 1;
                    } else if cordao == "." {
                        cordao.push(vec_conteudo[coluna]);
                        coluna = coluna + 1;

                        loop {
                            if NUMEROS.contains(&vec_conteudo[coluna]) {
                                cordao.push(vec_conteudo[coluna]);
                                coluna = coluna + 1;
                            } else {
                                vec_tabela.push((cordao.parse().unwrap(), String::from("Real"), linha));
                                acabou = true;
                                break;
                            }
                        }

                        if acabou == true {
                            acabou = false;
                            break;
                        }

                    } else {
                        vec_tabela.push((cordao.parse().unwrap(), String::from("Inteiro"), linha));
                        break;
                    }

                }
            } else if DELIMITADORES.contains(&vec_conteudo[coluna]) {
                if vec_conteudo[coluna] == ':' && vec_conteudo[coluna + 1] == '=' {
                    cordao.push(vec_conteudo[coluna]);
                    cordao.push(vec_conteudo[coluna + 1]);
                    vec_tabela.push((cordao, String::from("Inteiro"), linha));
                    coluna = coluna + 2;
                } else {
                    cordao.push(vec_conteudo[coluna]);
                    vec_tabela.push((cordao, String::from("Inteiro"), linha));
                    coluna = coluna + 1;
                }
            } else if OP_RELACIONAIS.contains(&&vec_conteudo[coluna].to_string()[..]) {
                if vec_conteudo[coluna] == '<' && vec_conteudo[coluna + 1] == '=' {
                    cordao.push(vec_conteudo[coluna]);
                    cordao.push(vec_conteudo[coluna + 1]);
                    coluna = coluna + 2;
                } else if vec_conteudo[coluna] == '>' && vec_conteudo[coluna + 1] == '=' {
                    cordao.push(vec_conteudo[coluna]);
                    cordao.push(vec_conteudo[coluna + 1]);
                    coluna = coluna + 2;
                } else if vec_conteudo[coluna] == '<' && vec_conteudo[coluna + 1] == '>' {
                    cordao.push(vec_conteudo[coluna]);
                    cordao.push(vec_conteudo[coluna + 1]);
                    coluna = coluna + 2;
                } else if vec_conteudo[coluna] == '=' {
                    cordao.push(vec_conteudo[coluna]);
                    coluna = coluna + 1;
                } else if vec_conteudo[coluna] == '<' {
                    cordao.push(vec_conteudo[coluna]);
                    coluna = coluna + 1;
                } else if vec_conteudo[coluna] == '>' {
                    cordao.push(vec_conteudo[coluna]);
                    coluna = coluna + 1;
                }

                vec_tabela.push((cordao, String::from("Op Relacional"), linha));

            } else if OP_ADITIVOS.contains(&&vec_conteudo[coluna].to_string()[..]) {
                cordao.push(vec_conteudo[coluna]);
                vec_tabela.push((cordao, String::from("Op Aditivo"), linha));
                coluna = coluna + 1;
            } else if OP_MULTIPLICATIVOS.contains(&&vec_conteudo[coluna].to_string()[..]) {
                cordao.push(vec_conteudo[coluna]);
                vec_tabela.push((cordao, String::from("Op Multiplicativo"), linha));
                coluna = coluna + 1;
            } else if vec_conteudo[coluna] == '\\' {
                if vec_conteudo[coluna + 1] == 'n' {
                    coluna = coluna + 2;
                    linha = linha + 1;
                } else if vec_conteudo[coluna + 1] == 't' || vec_conteudo[coluna + 1] == 'p' || vec_conteudo[coluna + 1] == 'b' {
                    coluna = coluna + 2;
                }
            } else {
                if vec_conteudo[coluna] != ' ' {
                    panic!("ERRO: Linha {}: Caractere desconhecido -> {}", linha, vec_conteudo[coluna]);
                    // break;
                } else {
                    coluna = coluna + 1;
                }
            }

            Ok(())
        };

        if let Err(_err) = err_catch() {
            for i in &vec_tabela {
                println!("{:?}", i);
            }
        }
    }
}

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