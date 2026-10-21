# Janelas nativas

A biblioteca `janela` (`window`) cria controles do Windows, sem navegador ou outra linguagem instalada. Esta implementação está em validação e ainda não tem backend para Linux/macOS.

Execute o exemplo com `cb run examples/window.cb --allow window`. A declaração `permissao janela` no programa e a autorização no terminal são obrigatórias.

```cb
permissao janela
janela.abrir("Meu aplicativo", 420, 240)
variavel nome = janela.entrada("Steve", 20, 20, 240, 28)
variavel resultado = janela.rotulo("", 20, 60, 340, 28)
variavel botao = janela.botao("Olá", "cumprimentar", 20, 100, 100, 32)
evento cumprimentar {
    janela.definir_texto(resultado, "Olá, " + janela.texto(nome))
}
janela.executar()
```

Os quatro números de cada controle são posição X, posição Y, largura e altura, em pixels. Entrada, rótulo e botão retornam um identificador inteiro. `janela.texto(id)` lê o texto e `janela.definir_texto(id, texto)` o altera. `janela.fechar()` encerra a janela. O nome passado ao botão identifica um evento comum da linguagem.

`janela.executar()` processa as interações até a janela fechar. Os eventos executam na mesma tarefa: uma operação demorada dentro deles bloqueia a interface. Existe uma janela por execução; não chame `executar()` dentro de um evento. Não há layout automático, menus ou desenho personalizado nesta etapa. Esta biblioteca é externa e não pode ser convertida para datapack.

Aliases ingleses: `window.open`, `window.input`, `window.label`, `window.button`, `window.text`, `window.set_text`, `window.run`, `window.close`.
