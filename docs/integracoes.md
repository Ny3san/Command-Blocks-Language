# Integrações

Esta página mostra o que cada integração faz hoje.

| Integração | Estado |
|---|---|
| Janelas (`window`) | Experimental, somente Windows |
| Rede (`net`) | Experimental, somente Windows |
| Ponte Minecraft (`minecraft_bridge`) | Cliente pronto; servidor não incluído |
| Discord | **Não funciona** ainda |

## Janelas (Windows)

Janelas usam controles nativos do Windows, sem navegador.

```text
permission window

window.open("Meu aplicativo", 420, 240)
variable name = window.input("Steve", 20, 20, 240, 28)
variable result = window.label("", 20, 60, 340, 28)
variable button = window.button("Olá", "greet", 20, 100, 100, 32)

event greet {
    window.set_text(result, "Olá, " + window.text(name))
}

window.run()
```

Execute com:

```text
cb run examples/window.cb --allow window
```

- Posição e tamanho são em pixels: X, Y, largura e altura.
- `window.text(id)` lê o texto e `window.set_text(id, texto)` altera.
- `window.close()` fecha a janela.
- Os eventos rodam na mesma tarefa, então uma operação demorada trava a janela.
- Não há layout automático nem desenho personalizado.

## Rede (Windows)

```text
permission net

variable response = net.get("https://example.com/")
print(response["status"])
print(response["body"])
```

Execute com `cb run programa.cb --allow net`.

- `net.get(url)` faz uma requisição GET com tempo limite de 10 segundos.
- `net.request(método, url, corpo, cabeçalhos, timeout_ms)` aceita GET, POST, PUT, DELETE, PATCH e HEAD.
- A resposta é um mapa com `status` (número) e `body` (texto).
- Respostas de erro (4xx e 5xx) voltam como resposta normal; o programa decide o que fazer.
- Cabeçalhos podem usar `secret("NOME")`. Nunca coloque credenciais diretamente na URL ou no código.

Limites: 1 MB por corpo de resposta, 10 requisições por segundo e timeout entre 100 e 60000 ms.

Por segurança, HTTP sem TLS só é aceito para `localhost` e `127.0.0.1`. Redirecionamentos e cookies são desativados.

## Ponte Minecraft

A ponte é um **cliente HTTP local**. Ela envia comandos para um servidor externo, que você precisa implementar ou instalar.

```text
cb install minecraft_bridge
```

Crie `bridge.cb` na mesma pasta:

```text
import minecraft_bridge
permission minecraft_bridge

minecraft_bridge.send("say Olá, Minecraft!")
```

Configure as variáveis de ambiente:

- `CB_BRIDGE_URL=http://127.0.0.1:PORTA/command`
- `CB_BRIDGE_TOKEN`: uma chave com pelo menos 24 caracteres (letras, números, `_` ou `-`).

Execute com `cb run bridge.cb --allow minecraft_bridge`.

Comandos aceitos: `say <texto>` e `tp <jogador> <x> <y> <z>` (coordenadas inteiras). Quebras de linha são recusadas.

O servidor precisa validar o token, aplicar sua própria lista de ações e responder `{"ok":true}` somente depois de executar a ação. Nenhum servidor pronto acompanha este cliente.

## Discord

**Ainda não é possível usar um bot do Discord com esta versão.**

`cb create discord-bot` gera um projeto de exemplo, mas a conexão com o Discord não está implementada. Se você tentar executar, o programa mostra o erro `CB202`.

Quando a integração existir, o token deve vir de `secret("DISCORD_TOKEN")`, nunca escrito no código.
