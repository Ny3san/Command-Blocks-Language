# Command Blocks

Command Blocks (CB) é uma linguagem de programação simples para **scripts, ferramentas de terminal e datapacks de Minecraft**.

```text
variable points = 0

function score_point() {
    points = points + 1
    print("Points: " + points)
}

event start {
    repeat 3 times {
        score_point()
    }
}
```

Saída:

```text
Points: 1
Points: 2
Points: 3
```

## Características

- Sintaxe curta, com blocos delimitados por chaves.
- Palavras-chave em inglês, com aliases em português.
- Mensagens de erro com localização, código e sugestão, em português e inglês.
- Permissão explícita para acesso a arquivos.
- Geração de datapack para Minecraft Java 1.21 e 1.21.1.
- Interpretador escrito em Rust, sem dependências externas.

## Estado

Esta é uma **versão inicial de desenvolvimento (0.1)**. Esta lista inclui apenas o que foi testado:

| Recurso | Estado |
|---|---|
| Linguagem: variáveis, funções, controle de fluxo, eventos e testes | Funciona |
| Biblioteca: texto, listas, mapas, matemática, JSON, entrada, tempo e arquivos | Funciona |
| Permissões, segredos e limites de execução | Funciona |
| Ferramentas: `run`, `check`, `test`, `format`, `build`, `create app` | Funciona |
| Datapack para Minecraft Java 1.21 e 1.21.1 | Gerado e testado por simulação; falta validar no jogo |

## Documentação

A documentação completa está em [docs/](docs/README.md).

## Instalação rápida

Com [Rust](https://rustup.rs/) instalado:

```text
cargo build --release --offline
```

O executável fica em `target/release/cb`. Detalhes em [Instalação](docs/ferramentas/instalacao.md).

## Contribuir

Leia o [guia de contribuição](CONTRIBUTING.md). Vulnerabilidades devem ser reportadas conforme a [política de segurança](SECURITY.md).

## Licença

Este projeto usa a licença [MIT](LICENSE).
