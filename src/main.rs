use std::env;

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    let consulta = &argumentos[1];
    let caminho_arquivo = &argumentos[2];
    
    println!("Buscando por {}", consulta);
    println!("No arquivo {}", caminho_arquivo);
}
