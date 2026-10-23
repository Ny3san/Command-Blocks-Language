# Aplicativos de terminal

Com Command Blocks você pode criar programas de linha de comando: eles leem dados, fazem cálculos, trabalham com listas e mapas e leem ou escrevem arquivos.

Janelas, botões e atalhos globais são experimentais (veja [Integrações](integracoes.md)).

## Criar um projeto

```text
cb create app meu_app
cb run meu_app/main.cb
```

Use `--lang en` para gerar o exemplo com mensagens em inglês.

## Exemplo: saudação

Crie `greet.cb`:

```text
variable name = io.read_text("Qual é seu nome? ")
print("Olá, " + name)
```

```text
cb run greet.cb
```

## Gerar um executável (Windows)

```text
cb build greet.cb --output greet.exe
```

O executável contém o interpretador e o código-fonte. Quem recebe o arquivo não precisa instalar `cb`, Rust nem Python.

Atenção:

- Isso **não** compila o programa para código de máquina e **não** esconde o código.
- Arquivos auxiliares não são incluídos automaticamente.
- Não coloque senhas ou tokens no código.

## Automação de arquivos

Crie `report.cb`:

```text
permission file

variable names = ["Steve", "Alex"]
variable lines = text.join(names, "\n")
file.write("players.txt", lines)
print("Relatório criado")
```

```text
cb run report.cb --allow file
```

Os arquivos ficam na pasta do programa. `file.write` **substitui** o conteúdo do arquivo, então use um nome que não seja importante.
