use std::{env, fs, process};

fn main() {
    let argumentos: Vec<String> = env::args().collect();
    let configuração = Configuração::from(&argumentos).unwrap_or_else(|erro| {
        println!("Problema ao obter argumentos: {erro}");
        process::exit(1);
    });

    println!("Buscando por {}", configuração.consulta);
    println!("No arquivo {}", configuração.caminho_arquivo);

    let conteúdo =
        fs::read_to_string(configuração.caminho_arquivo).expect("Deve conseguir ler o arquivo");

    println!("No texto:\n{conteúdo}");
}

struct Configuração {
    consulta: String,
    caminho_arquivo: String,
}

impl Configuração {
    fn from(argumentos: &[String]) -> Result<Self, String> {
        if argumentos.len() < 3 {
            return Err(String::from("Argumentos insuficientes"));
        }

        Ok(Configuração {
            consulta: argumentos[1].clone(),
            caminho_arquivo: argumentos[2].clone(),
        })
    }
}
