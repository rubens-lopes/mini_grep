use std::{env, process};

use mini_grep::Configuração;

fn main() {
    let argumentos: Vec<String> = env::args().collect();
    let configuração = Configuração::from(&argumentos).unwrap_or_else(|erro| {
        eprintln!("Problema ao obter argumentos: {erro}");
        process::exit(1);
    });

    if let Err(erro) = mini_grep::executar(configuração) {
        eprintln!("Aplicação falhou: {erro}");
        process::exit(1);
    }
}
