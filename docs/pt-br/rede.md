# Rede e APIs

O transporte inicial usa WinHTTP no Windows e está em validação. Não exige navegador, Python ou outra linguagem instalada. Outros sistemas retornam incompatibilidade explícita nesta etapa.

```cb
permissao rede
variavel resposta = rede.get("https://example.com/")
imprimir(resposta["status"])
imprimir(resposta["body"])
```

Execute com `cb run programa.cb --allow net`. A biblioteca tem aliases `rede` e `net`.

`net.get(url)` usa GET com tempo limite de 10 segundos. `net.request(metodo, url, corpo, cabecalhos, timeout_ms)` permite GET, POST, PUT, DELETE, PATCH e HEAD. `rede.requisicao` é o alias em português. Ambas retornam um mapa com `status` inteiro e `body` em texto UTF-8. Respostas 4xx/5xx são retornadas para o programa decidir como tratá-las.

```cb
permissao rede
variavel dados = {"nome": "Alex"}
variavel resposta = rede.requisicao(
    "POST", "https://example.com/api", json.escrever(dados),
    {"Content-Type": "application/json"}, 5000)
```

Cabeçalhos aceitam texto ou `segredo("NOME_DA_VARIAVEL")`; segredos são enviados somente como valor do cabeçalho solicitado. Não coloque credenciais em URLs ou no código. O transporte não registra URL, corpo, cabeçalhos ou credenciais em erros.

Limites: 1 MB por corpo, 64 cabeçalhos, 16 KB de cabeçalhos, 10 requisições por segundo por execução (compartilhadas com a ponte), timeout entre 100 e 60000 ms por operação de rede. Há também um limite de duração verificado entre leituras. HTTPS valida certificados pelo Windows. HTTP sem TLS é aceito apenas para `localhost` e `127.0.0.1`. IPv6 e URLs com fragmentos ou credenciais não são aceitos ainda.

Redirecionamentos automáticos, cookies e autenticação automática do Windows são desativados. Um redirecionamento volta como resposta 3xx; cabe ao programa verificar seu destino antes de uma nova requisição. Downloads binários, streaming e cabeçalhos de resposta não estão disponíveis nesta etapa. Esta biblioteca é externa e não pode ser convertida para Minecraft.

Implementação baseada na [API WinHTTP](https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpopenrequest) e nas [opções de segurança do transporte](https://learn.microsoft.com/en-us/windows/win32/winhttp/option-flags).
