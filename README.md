# Command Blocks

Command Blocks (CB) é uma linguagem de programação simples para criar **scripts, ferramentas de terminal e datapacks de Minecraft**.

- O código é escrito em **inglês** (`print`, `variable`, `function`…).
- A documentação é em **português**.
- O compilador e o interpretador são feitos em Rust, mas você **não precisa saber Rust** para usar a linguagem.

> **Status:** versão inicial de desenvolvimento (0.1). Algumas partes funcionam bem, outras são experimentais e algumas ainda não existem. Veja o que está pronto em [Estado atual](#estado-atual).

## Exemplo

```text
variable points = 0

function score_point() {
    points = points + 1
    print("Pontuação: " + points)
}

event start {
    repeat 3 times {
        score_point()
    }
}
```

Saída:

```text
Pontuação: 1
Pontuação: 2
Pontuação: 3
```

## Instalação

Você precisa do [Rust](https://rustup.rs/) instalado.

```text
cargo build --release --offline
```

O executável fica em `target/release/cb` (Linux/macOS) ou `target\release\cb.exe` (Windows). Copie-o para uma pasta do seu `PATH`.

No Windows com a toolchain MSVC, é necessário o **Visual Studio Build Tools** com C++ e Windows SDK. Detalhes em [Primeiros passos](docs/primeiros-passos.md).

## Uso rápido

```text
cb run hello.cb                  # executa um programa
cb check hello.cb                # verifica erros sem executar
cb test tests.cb                 # executa os blocos test
cb convert hello.cb --target minecraft --output meu_datapack
cb help                          # lista os comandos
```

## Estado atual

| Parte | Estado |
|---|---|
| Variáveis, funções, `if`, `while`, `for each`, `repeat` | Funciona |
| Eventos `start` e testes (`test`) | Funciona |
| Listas, mapas, texto, matemática, JSON | Funciona |
| Permissões e segredos (`--allow`) | Funciona |
| Diagnósticos em português e inglês | Funciona |
| Gerar datapack Minecraft Java 1.21 / 1.21.1 | Funciona (testado só com scoreboards simulados; falta validar no jogo) |
| Janelas nativas (Windows) | Experimental |
| Rede HTTP (Windows) | Experimental |
| Ponte HTTP com servidor Minecraft | Cliente pronto; servidor não incluído |
| Bot do Discord | **Não funciona** ainda |
| Automação de teclas no cliente Minecraft | **Não implementado** |

## Documentação

- [Primeiros passos](docs/primeiros-passos.md): seu primeiro programa.
- [Linguagem](docs/linguagem.md): sintaxe, tipos, funções e eventos.
- [Bibliotecas](docs/bibliotecas.md): funções prontas (texto, listas, mapas, JSON…).
- [Aplicativos de terminal](docs/aplicativos.md): programas que leem e escrevem arquivos.
- [Minecraft](docs/minecraft.md): gerar um datapack.
- [Integrações](docs/integracoes.md): janelas, rede, ponte Minecraft e Discord.
- [Permissões e segurança](docs/seguranca.md).
- [Erros](docs/erros.md): o que significa cada código `CBxxx`.
- [CLI](docs/cli.md): todos os comandos.
- [Arquitetura](docs/arquitetura.md): como o projeto é organizado, para quem quer contribuir.
- [Roadmap](docs/roadmap.md): o que ainda não existe.
- [Gramática formal](docs/grammar.ebnf).

## Contribuir

Leia o [guia de contribuição](CONTRIBUTING.md). Vulnerabilidades devem ser reportadas conforme a [política de segurança](SECURITY.md).

## Licença

Este projeto usa a licença [MIT](LICENSE).
