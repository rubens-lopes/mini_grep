use std::{env, fs};

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    let consulta = &argumentos[1];
    let caminho_arquivo = &argumentos[2];
    
    println!("Buscando por {}", consulta);
    println!("No arquivo {}", caminho_arquivo);

    let conteúdo = fs::read_to_string(caminho_arquivo)
        .expect("Deve conseguir ler o arquivo");

    println!("No texto:\n{conteúdo}");
}
