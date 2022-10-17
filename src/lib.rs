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
    let Configuração { consulta, caminho_arquivo } = configuração;

    let conteúdo = fs::read_to_string(caminho_arquivo)?;
    for linha in pesquisar(&consulta, &conteúdo) {
        println!("{linha}")
    }

    Ok(())
}

pub fn pesquisar<'a>(consulta: &str, conteúdo: &'a str) -> Vec<&'a str> {
    let mut resultado = Vec::new();

    for linha in conteúdo.lines() {
        if linha.contains(consulta) == false {
            continue;
        }

        resultado.push(linha);
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn um_resultado() {
        let consulta = "duti";
        let conteúdo = "\
Rust:
seguro, rápido, produtivo.
Escolha três.";

        assert_eq!(
            vec!["seguro, rápido, produtivo."],
            pesquisar(consulta, conteúdo)
        );
    }
}
