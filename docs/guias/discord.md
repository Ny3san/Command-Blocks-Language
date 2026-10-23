# Discord

> **Estado: não funcional.** Ainda não é possível operar um bot do Discord com esta versão.

O comando `cb create discord-bot` gera um projeto de exemplo, mas a conexão com o Discord não está implementada. Ao executar, o programa mostra `CB202`, indicando que o transporte não existe.

O que já existe é apenas a interface planejada, com validação de tipos e de segredos. A forma esperada é:

```text
permission net
variable token = secret("DISCORD_TOKEN")
discord.start(token)
```

Esse trecho **não funciona** nesta versão.

Ao implementar a integração, o token deverá vir sempre de `secret`, nunca ser escrito no código.
