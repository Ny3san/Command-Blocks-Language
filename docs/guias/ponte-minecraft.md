# Ponte Minecraft

> **Estado: cliente pronto.** O servidor que recebe os comandos não é incluído e precisa ser implementado ou instalado separadamente.

A biblioteca `minecraft_bridge` (alias `ponte_minecraft`) é um **cliente HTTP local**. Ela envia comandos para um servidor externo que você controla. Não é um plugin.

## Configuração

```text
cb install minecraft_bridge
```

Crie `bridge.cb` na mesma pasta:

```text
import minecraft_bridge
permission minecraft_bridge

minecraft_bridge.send("say Hello, Minecraft!")
```

Defina as variáveis de ambiente:

- `CB_BRIDGE_URL=http://127.0.0.1:PORT/command`
- `CB_BRIDGE_TOKEN`: chave com pelo menos 24 caracteres (letras, dígitos, `_` ou `-`).

Execute:

```text
cb run bridge.cb --allow minecraft_bridge
```

Sem `import`, `permission`, `--allow` ou as variáveis de ambiente, a execução falha antes de qualquer conexão.

## Comandos aceitos

| Comando | Formato |
|---|---|
| `say` | `say <texto>` |
| `tp` | `tp <jogador> <x> <y> <z>`, com coordenadas inteiras |

Quebras de linha são recusadas. URLs remotas, redirecionamentos e comandos fora da lista também são recusados. O limite é de 10 requisições por segundo, com timeout de 3 segundos por operação.

## Contrato com o servidor

O cliente envia:

```http
POST /command
Authorization: Bearer <token>
Content-Type: application/json

{"command":"say Hello"}
```

O servidor deve:

1. validar o token;
2. aplicar a própria lista de ações permitidas;
3. executar a ação no jogo;
4. responder HTTP 200 com `{"ok":true}` **somente depois** de executar a ação.

Respostas maiores que 64 KiB e respostas sem confirmação são recusadas. Não há repetição automática, para não duplicar ações.

Nenhuma implementação do servidor é distribuída com este projeto e nenhuma foi validada dentro do jogo.
