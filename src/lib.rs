use std::{error, fs};

pub struct Configuração {
    pub consulta: String,
    pub caminho_arquivo: String,
}

impl Configuração {
    pub fn from(argumentos: &[String]) -> Result<Self, String> {
        if argumentos.len() < 3 {
            return Err(String::from("Argumentos insuficientes"));
        }

        Ok(Configuração {
            consulta: argumentos[1].clone(),
            caminho_arquivo: argumentos[2].clone(),
        })
    }
}

pub fn executar(configuração: Configuração) -> Result<(), Box<dyn error::Error>> {
    let conteúdo = fs::read_to_string(configuração.caminho_arquivo)?;

    println!("No texto:\n{conteúdo}");

    Ok(())
}
