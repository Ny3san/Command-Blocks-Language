# Command Blocks

Command Blocks (CB) é uma linguagem de programação simples para **scripts, ferramentas de terminal e datapacks de Minecraft**.

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

## Características

- Sintaxe curta, com blocos delimitados por chaves.
- Palavras-chave em inglês, com aliases em português.
- Mensagens de erro com localização, código e sugestão, em português e inglês.
- Permissões explícitas para arquivos, rede e janelas.
- Geração de datapack para Minecraft Java 1.21 e 1.21.1.
- Interpretador escrito em Rust, sem dependências externas no núcleo.

## Estado

Esta é uma **versão inicial de desenvolvimento (0.1)**.

| Recurso | Estado |
|---|---|
| Linguagem: variáveis, funções, controle de fluxo, eventos e testes | Funciona |
| Biblioteca padrão: texto, listas, mapas, matemática e JSON | Funciona |
| Permissões, segredos e limites de execução | Funciona |
| Datapack para Minecraft Java 1.21 e 1.21.1 | Funciona; validação no jogo pendente |
| Janelas e rede (Windows) | Experimental |
| Ponte Minecraft | Cliente pronto; servidor não incluído |
| Discord | Não funcional |
| Automação de teclas no cliente Minecraft | Não implementado |

## Documentação

A documentação completa está em [docs/](docs/README.md):

- [Tutorial](docs/tutorial/01-primeiro-programa.md) para começar do zero.
- [Referência da linguagem](docs/referencia/README.md).
- [Biblioteca padrão](docs/biblioteca/biblioteca-padrao.md).
- [Ferramentas](docs/ferramentas/README.md): instalação, linha de comando e erros.
- [Guias](docs/guias/README.md): segurança, Minecraft, janelas, rede e integrações.

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
