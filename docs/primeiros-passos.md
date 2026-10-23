# Primeiros passos

## 1. Escreva um programa

Crie um arquivo chamado `hello.cb`:

```text
print("Olá, mundo!")
```

Arquivos de Command Blocks usam a extensão `.cb`.

## 2. Execute

```text
cb run hello.cb
```

Saída:

```text
Olá, mundo!
```

Se o comando `cb` não for encontrado, use o caminho completo do executável, por exemplo `.\cb.exe run hello.cb` no PowerShell.

## 3. Guarde valores e tome decisões

```text
variable points = 3
points = points + 2

if points >= 5 {
    print("Você venceu!")
} else {
    print("Continue jogando")
}
```

- `variable` guarda um valor que pode mudar.
- `constant` guarda um valor que não pode mudar.
- `if` executa um bloco quando a condição é verdadeira.
- Cada instrução fica em uma linha. Você também pode separar com `;`.

## 4. Repita uma ação

```text
repeat 3 times {
    print("Mais um!")
}
```

## 5. Crie uma função

```text
function greet(name) {
    print("Olá, " + name)
}

greet("Alex")
```

## 6. Use português se preferir

As palavras-chave em português também funcionam. Os dois códigos abaixo fazem a mesma coisa:

```text
variavel pontos = 3
se pontos >= 3 {
    imprimir("Você venceu!")
}
```

```text
variable points = 3
if points >= 3 {
    print("You won!")
}
```

A tabela completa de palavras está em [Linguagem](linguagem.md#palavras-em-português-e-inglês).

Recomendação: escreva o código em inglês, que é a forma padrão e a mais documentada.

## Próximos passos

- [Linguagem](linguagem.md): aprenda a sintaxe completa.
- [Bibliotecas](bibliotecas.md): funções prontas.
- [Aplicativos](aplicativos.md): crie um programa que lê entradas e arquivos.
- [Minecraft](minecraft.md): converta um programa para datapack.
