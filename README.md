# Mini Grep
Versão extremamente simplificada da CLI `grep` proposta, e depois melhorada, nos capítulos 12 e 13 do livro https://doc.rust-lang.org/stable/book/title-page.html.

## Rodando os testes
Na raiz do projeto:
```shell
cargo test
```

## Rodando o programa
Na raiz do projeto:

```shell
cargo run -- "trecho a se procurar" "caminho.do.arquivo.txt"
```
O arquivo `poema.txt` incluso para facilitar a experimentação.
Existe também a possibilidade de se buscar ignorando acentos e sem diferenciar maiúsculas e minusculas, através do argumento `--modo-permissivo`.

```shell
cargo run -- nao poema.txt --modo-permissivo
```