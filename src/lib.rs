use std::{env, error, fs};

pub struct Configuração {
    pub consulta: String,
    pub caminho_arquivo: String,
    pub modo_permissivo: bool,
}

impl Configuração {
    pub fn from(mut argumentos: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        argumentos.next();

        let consulta = match argumentos.next() {
            Some(argumento) => argumento,
            None => return Err("Termo a ser buscado é obrigatório."),
        };

        let caminho_arquivo = match argumentos.next() {
            Some(argumento) => argumento,
            None => return Err("Caminho para o arquivo é obrigatório."),
        };

        let modo_permissivo = match argumentos.next() {
            Some(argumento) => argumento == "--modo-permissivo",
            _ => "1".to_owned() == env::var("MODO_PERMISSIVO").unwrap_or_else(|_| "0".to_owned()),
        };

        Ok(Configuração {
            consulta,
            caminho_arquivo,
            modo_permissivo,
        })
    }
}

pub fn executar(configuração: Configuração) -> Result<(), Box<dyn error::Error>> {
    let Configuração {
        consulta,
        caminho_arquivo,
        modo_permissivo,
    } = configuração;

    let conteúdo = fs::read_to_string(caminho_arquivo)?;
    let resultado = if modo_permissivo {
        pesquisar_permissivamente(&consulta, &conteúdo)
    } else {
        pesquisar(&consulta, &conteúdo)
    };

    for linha in resultado {
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

pub fn pesquisar_permissivamente<'a>(consulta: &str, conteúdo: &'a str) -> Vec<&'a str> {
    let consulta = &preparar_texto(&consulta);
    let mut resultado = Vec::new();

    for linha in conteúdo.lines() {
        if preparar_texto(linha).contains(consulta) == false {
            continue;
        }

        resultado.push(linha);
    }

    resultado
}

fn preparar_texto(texto: &str) -> String {
    diacritics::remove_diacritics(&texto.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pesquisa_estrita() {
        let consulta = "gur";
        let conteúdo = "\
Rust:
seguro, rápido, produtivo.
Escolha três.
Gurus recomendam.";

        assert_eq!(
            vec!["seguro, rápido, produtivo."],
            pesquisar(consulta, conteúdo)
        );
    }

    #[test]
    fn pesquisa_permissiva() {
        let consulta = "aP";
        let conteúdo = "\
Rust:
seguro, rápido, produtivo.
Escolha três.
Escreva aplicações que rodam em qualquer lugar.";

        assert_eq!(
            vec![
                "seguro, rápido, produtivo.",
                "Escreva aplicações que rodam em qualquer lugar."
            ],
            pesquisar_permissivamente(consulta, conteúdo)
        );
    }
}
