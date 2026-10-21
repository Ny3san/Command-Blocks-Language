# CLI: contrato de ajuda e versão

Estado: **implemented**. A ajuda não executa programas nem concede permissões.

```text
cb --help
cb help
cb help run
cb run --help
cb help convert --lang en
cb --version --lang pt-br
```

`cb help COMANDO` e `cb COMANDO --help` mostram a mesma ajuda. A ajuda global lista comandos e descrições; cada comando mostra sintaxe, limites e um exemplo real do repositório. `--lang pt-br|en` aceita também `--lang=en`; português é o padrão. A versão vem do pacote compilado.

Ajuda e resultados vão para stdout; erros vão para stderr. Código de saída 0 significa sucesso; 1 significa erro de argumentos, análise ou execução. `CB000` identifica argumentos inválidos. Tópicos desconhecidos, opções duplicadas (exceto `--allow`) e combinações como `--help --version` são recusados. Ajuda aceita apenas o comando/tópico e o idioma: use `cb run --help`, sem caminho ou opções de execução.

As permissões continuam exigindo declaração no fonte e autorização explícita com `--allow`. Consultar ajuda não instala bibliotecas. O cliente REST Discord é experimental no Windows; `discord.iniciar`/`discord.start` e eventos Gateway continuam indisponíveis.

Migração: nenhum programa `.cb` precisa mudar. `cb help TOPICO_INEXISTENTE`, que antes mostrava uma orientação genérica, agora retorna erro. Nenhum comando foi removido.

Planejado, ainda indisponível: `doctor`, `init`, `fmt`, `package`, saída JSON e controle de cores. Use `create`, `format` e `build` existentes. Diagnósticos com trechos de fonte e múltiplos erros pertencem a outro milestone.

Testes: fixtures PT/EN em `tests/golden`, equivalência de formas de ajuda, todos os comandos em ambos os idiomas, opções inválidas e separação stdout/stderr em `tests/cli_and_integrations.rs`.
