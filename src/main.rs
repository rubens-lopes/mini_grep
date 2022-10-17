use std::{env, error::Error, fs, process};

fn main() {
    let argumentos: Vec<String> = env::args().collect();
    let configuração = Configuração::from(&argumentos).unwrap_or_else(|erro| {
        println!("Problema ao obter argumentos: {erro}");
        process::exit(1);
    });

    println!("Buscando por {}", configuração.consulta);
    println!("No arquivo {}", configuração.caminho_arquivo);

    if let Err(erro) = executar(configuração) {
        println!("Aplicação falhou: {erro}");
        process::exit(1);
    }
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

fn executar(configuração: Configuração) -> Result<(), Box<dyn Error>> {
    let conteúdo = fs::read_to_string(configuração.caminho_arquivo)?;

    println!("No texto:\n{conteúdo}");

    Ok(())
}
