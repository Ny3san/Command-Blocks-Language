# Permissões e segurança

## Permissões

Recursos externos, como arquivos, rede e janelas, precisam de duas autorizações:

1. **No código**, uma linha `permission`:

   ```text
   permission file
   file.write("config.txt", "modo = seguro")
   ```

2. **Na execução**, a opção `--allow`:

   ```text
   cb run programa.cb --allow file
   ```

Se faltar uma das duas, o programa não executa o recurso. Os nomes aceitos são:

| Declaração | Nomes aceitos |
|---|---|
| `permission file` | `arquivo` |
| `permission net` | `rede` |
| `permission window` | `janela` |
| `permission minecraft_bridge` | `ponte_minecraft` |

## Arquivos

- Os arquivos ficam restritos à pasta do programa.
- Caminhos absolutos, `..` e links simbólicos que apontam para fora são bloqueados.
- Isso **não é uma sandbox do sistema**. Não execute código que você não confia com permissões.

## Segredos

`secret("NOME")` lê uma variável de ambiente.

```text
variable token = secret("API_TOKEN")
```

- Segredos não aparecem em `print`, em listas, em mapas nem em mensagens de erro.
- **Nunca escreva senhas ou tokens dentro do arquivo `.cb`.** Coloque-os em variáveis de ambiente.
- Os programas empacotados com `cb build` contêm o código-fonte. Por isso, nunca coloque credenciais nele.

## Limites

Para evitar que um programa trave o computador, existem limites de:

- número de passos executados (`--max-steps`, padrão 100000);
- profundidade de recursão e de aninhamento;
- tamanho do código e dos valores;
- tamanho e frequência das respostas de rede.

Esses limites não substituem isolamento completo de memória ou CPU.

## Processos externos

A versão 0.1 **não executa processos externos**. `process.run` está reservado e falha com erro explícito.
