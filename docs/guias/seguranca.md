# Permissões e segurança

Command Blocks separa o que o programa **declara** que precisa do que o usuário **autoriza** a executar.

## Permissões

Um recurso externo exige duas autorizações:

1. No código, uma declaração `permission`:

   ```text
   permission file
   file.write("config.txt", "mode = safe")
   ```

2. Na execução, a opção `--allow`:

   ```text
   cb run program.cb --allow file
   ```

Se faltar uma delas, o recurso não é usado e a execução falha com `CB301`.

| Declaração | Nome em `--allow` | Alias em português |
|---|---|---|
| `permission file` | `file` | `arquivo` |

Operações que não usam recursos externos, como `json.*`, `text.*` e `math.*`, não exigem permissão.

## Arquivos

- O acesso fica restrito à pasta do programa.
- Caminhos absolutos, `..` e links simbólicos que apontam para fora da pasta são bloqueados.
- `file.write` substitui o conteúdo do arquivo sem confirmação.

Essa restrição **não é uma sandbox do sistema operacional**. Não execute código de origem desconhecida com permissões concedidas.

## Segredos

`secret("NAME")` lê uma variável de ambiente.

```text
variable token = secret("API_TOKEN")
```

- Segredos não aparecem em `print`, em listas, em mapas ou em mensagens de erro.
- Valores de variáveis de ambiente cujo nome contém `TOKEN`, `PASSWORD`, `SECRET` ou `API_KEY` também são removidos da saída.
- Credenciais **nunca** devem ser escritas no arquivo `.cb`.

O comando `cb build` inclui o código-fonte no executável. Por isso, o código não deve conter credenciais.

## Limites de execução

Para proteger contra programas que não terminam, a execução tem limites:

- número de passos (`--max-steps`, padrão 100000);
- profundidade de recursão e de aninhamento;
- tamanho do código-fonte e dos valores;

Esses limites não substituem isolamento completo de memória ou de processador.

## Processos externos

A versão 0.1 não executa processos externos. `process.run` está reservado e falha com erro explícito.

## Reportar problemas de segurança

Veja a [política de segurança](../../SECURITY.md).
