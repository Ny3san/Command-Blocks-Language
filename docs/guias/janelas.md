# Janelas

> **Estado: experimental.** Disponível somente no Windows, com controles nativos. Não há backend para Linux ou macOS.

```text
permission window

window.open("My app", 420, 240)
variable name = window.input("Steve", 20, 20, 240, 28)
variable result = window.label("", 20, 60, 340, 28)
variable button = window.button("Hello", "greet", 20, 100, 100, 32)

event greet {
    window.set_text(result, "Hello, " + window.text(name))
}

window.run()
```

Execute com:

```text
cb run examples/window.cb --allow window
```

## Funções

| Função | Descrição |
|---|---|
| `window.open(title, width, height)` (`janela.abrir`) | Abre a janela |
| `window.label(text, x, y, width, height)` (`janela.rotulo`) | Cria um rótulo e devolve seu identificador |
| `window.input(text, x, y, width, height)` (`janela.entrada`) | Cria um campo de texto e devolve seu identificador |
| `window.button(text, event, x, y, width, height)` (`janela.botao`) | Cria um botão que dispara o evento indicado |
| `window.text(id)` (`janela.texto`) | Lê o texto de um controle |
| `window.set_text(id, text)` (`janela.definir_texto`) | Altera o texto de um controle |
| `window.run()` (`janela.executar`) | Processa as interações até a janela fechar |
| `window.close()` (`janela.fechar`) | Fecha a janela |

Posição e tamanho são medidos em pixels.

## Comportamento

- Os eventos são executados na mesma tarefa da interface. Uma operação demorada dentro de um evento deixa a janela sem resposta.
- Há uma única janela por execução.
- Não chame `window.run()` dentro de um evento.
- Não há layout automático, menus nem desenho personalizado.
- Esta biblioteca não pode ser convertida para datapack.
