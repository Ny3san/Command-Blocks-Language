# Rede

> **Estado: experimental.** O transporte usa WinHTTP e está disponível somente no Windows.

```text
permission net

variable response = net.get("https://example.com/")
print(response["status"])
print(response["body"])
```

Execute com:

```text
cb run program.cb --allow net
```

## Funções

| Função | Descrição |
|---|---|
| `net.get(url)` (`rede.obter`) | Requisição GET com tempo limite de 10 segundos |
| `net.request(method, url, body, headers, timeout_ms)` (`rede.requisicao`) | Requisição com método, corpo e cabeçalhos |

Métodos aceitos: `GET`, `POST`, `PUT`, `DELETE`, `PATCH` e `HEAD`.

A resposta é um mapa com:

- `status`: código HTTP, como `Int`;
- `body`: corpo em texto UTF-8.

Respostas 4xx e 5xx são devolvidas normalmente. Decidir o que fazer com elas é responsabilidade do programa.

```text
permission net

variable data = {"name": "Alex"}
variable response = net.request(
    "POST", "https://example.com/api", json.stringify(data),
    {"Content-Type": "application/json"}, 5000)
```

## Segurança

- Cabeçalhos podem usar `secret("NAME")`. O valor é enviado somente no cabeçalho solicitado.
- Credenciais nunca devem aparecer na URL nem no código.
- HTTPS valida certificados pelo Windows.
- HTTP sem TLS só é aceito para `localhost` e `127.0.0.1`.
- Redirecionamentos, cookies e autenticação automática estão desativados. Um redirecionamento volta como resposta 3xx, e o programa decide se segue.

## Limites

- 1 MB por corpo de resposta.
- 64 cabeçalhos e 16 KB de cabeçalhos.
- 10 requisições por segundo por execução, compartilhadas com a ponte.
- Timeout entre 100 e 60000 ms por operação.

Ainda não há suporte a IPv6, a URLs com fragmento ou credenciais, a downloads binários, a streaming e a cabeçalhos de resposta.
