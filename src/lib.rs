use std::{env, error, fs};

pub struct Configuração {
    pub consulta: String,
    pub caminho_arquivo: String,
    pub modo_permissivo: bool,
}

impl Configuração {
    pub fn from(argumentos: &[String]) -> Result<Self, String> {
        if argumentos.len() < 3 {
            return Err(String::from("Argumentos insuficientes"));
        }

        let modo_permissivo = match argumentos.get(3) {
            Some(argumento) => argumento == "--modo-permissivo",
            _ => {
                let ref modo_permissivo = env::var("MODO_PERMISSIVO");
                let está_ativo = |modo| modo == "1";
                matches!(modo_permissivo, Ok(modo) if está_ativo(modo))
            },
        };

        Ok(Configuração {
            consulta: argumentos[1].clone(),
            caminho_arquivo: argumentos[2].clone(),
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
