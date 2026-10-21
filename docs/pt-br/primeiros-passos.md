# Seu primeiro programa

[Índice](../../README.md) · [Referência completa](README.md)

Você escreve Command Blocks, não Rust. O arquivo tem extensão `.cb`. O executável `cb` contém a linguagem e suas bibliotecas básicas.

## 1. Escreva

Crie `ola.cb`:

```text
imprimir("Olá, mundo!")
```

## 2. Execute

```text
cb run ola.cb
```

O terminal mostra `Olá, mundo!`. Não precisa de evento, import ou função para começar. Os exemplos pressupõem que `cb` já esteja no PATH; no PowerShell, um executável da pasta atual é chamado com `./cb.exe`.

## 3. Guarde valores e tome decisões

```text
variavel pontos = 3
pontos = pontos + 2

se pontos >= 5 {
    imprimir("Você venceu!")
}
```

`variavel` guarda um valor; `se` decide se executa um bloco; `{ }` delimita esse bloco. A quebra de linha separa instruções.

## 4. Repita uma ação

```text
repetir 3 vezes {
    imprimir("Mais um bloco!")
}
```

## 5. Reaproveite uma ação

```text
funcao cumprimentar(nome) {
    imprimir("Olá, " + nome)
}

cumprimentar("Alex")
```

Esse exemplo funciona no computador. O backend Minecraft inicial ainda não aceita parâmetros de funções; os ambientes não têm todas as mesmas capacidades.

## 6. Use inglês se preferir

```text
variable points = 3
if points >= 3 {
    print("You won!")
}
```

As duas sintaxes são equivalentes. `--lang en` muda as mensagens da ferramenta, não o idioma obrigatório do código.

## Próximos passos

- [Aplicativos de terminal](aplicativos.md): entrada, arquivos e executáveis.
- [Minecraft](minecraft.md): converter um programa compatível em datapack.
- [Referência](README.md): tipos, funções, eventos, bibliotecas e permissões.
- [Integrações](integracoes.md): estado real de Discord e ponte Minecraft.

Recursos futuros ficam na [proposta de expansão](expansao.md). Não é necessário aprendê-los para usar a base atual.
