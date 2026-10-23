# 7. Aplicativo de terminal

Este passo junta o que você viu: entrada do usuário, decisões, funções e arquivos.

Crie `greet.cb`:

```text
variable name = io.read_text("What is your name? ")

function greet(person) {
    print("Hello, " + person)
}

greet(name)
```

Execute:

```text
cb run greet.cb
```

## Arquivos

Operações com arquivos exigem uma declaração no código e uma autorização na linha de comando:

```text
permission file

variable names = ["Steve", "Alex"]
file.write("players.txt", text.join(names, "\n"))
print("Report created")
```

```text
cb run report.cb --allow file
```

Os arquivos ficam restritos à pasta do programa. Veja [Permissões e segurança](../guias/seguranca.md).

## Gerar um executável (Windows)

```text
cb build greet.cb --output greet.exe
```

O executável contém o interpretador e o código-fonte. Ele não é compilação para código de máquina, e o código não fica oculto.

## Próximos passos

- [Referência da linguagem](../referencia/README.md)
- [Biblioteca padrão](../biblioteca/biblioteca-padrao.md)
- [Ferramentas](../ferramentas/README.md)
