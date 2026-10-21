# Integrações: o que está pronto

[Índice](../../README.md) · [Expansões propostas](expansao.md)

## Ponte Minecraft

A biblioteca opcional `ponte_minecraft/minecraft_bridge` é um **cliente HTTP local**, não um plugin nem servidor completo.

Na pasta do programa:

```text
cb install minecraft_bridge
```

Crie `ponte.cb` nessa mesma pasta:

```text
importar ponte_minecraft
permissao minecraft_bridge
ponte_minecraft.enviar("say Olá, Minecraft!")
```

Configure pelo ambiente `CB_BRIDGE_URL=http://127.0.0.1:PORT/command` e `CB_BRIDGE_TOKEN`, gerado com segurança fora do fonte. A chave deve ter pelo menos 24 caracteres, usando letras, dígitos, `_` ou `-`. Execute:

```text
cb run ponte.cb --allow minecraft_bridge
```

Sem configuração, import, habilitação ou permissão, falha antes de conectar. URLs remotas, redirecionamentos e comandos fora da lista são recusados. Até dez requisições por segundo por runtime. Cada conexão/leitura/escrita tem timeout de três segundos; não é um prazo total para toda a sessão.

### Contrato do servidor externo

O cliente envia `POST /command`, com `Authorization: Bearer <token>`, tipo JSON e corpo `{"command":"say Olá"}`. O servidor precisa validar autenticação, aplicar sua própria lista de ações permitidas, executar a ação autorizada no jogo e responder HTTP 200 com `{"ok":true}` somente após confirmar a execução.

Feche a conexão depois da resposta; a implementação atual não decodifica respostas chunked. O corpo de confirmação só aceita esse objeto, com espaços opcionais, sem campos adicionais. Respostas maiores que 64 KiB e respostas sem confirmação são recusadas. Não há retry automático, evitando duplicar ações.

Comandos aceitos: `say <texto>` e `tp <nome de jogador> <x> <y> <z>` com coordenadas inteiras. Quebras de linha são bloqueadas. Nenhum servidor/plugin acompanhando este cliente foi implementado ou validado dentro do jogo.

## Discord

**Ainda não é possível operar um bot Discord com esta versão.** Há assinaturas de funções, validação de tipos, tokens protegidos e um template, mas não há conexão ao Gateway/REST.

```text
cb create discord-bot meu_bot
```

O projeto gerado explica essa limitação. A interface esperada usa `discord.iniciar(segredo("DISCORD_TOKEN"))`, declaração `permissao rede`, autorização `--allow rede`, import e habilitação local. Mesmo com tudo configurado, a execução retorna CB202 indicando transporte não implementado. `cb check` valida sintaxe/tipos, não disponibilidade de transporte.

Tokens literais são incompatíveis com a assinatura de iniciar. Segredos de ambiente não são incluídos em logs, mensagens de erro ou empacotamento automático. Nunca coloque o token diretamente na fonte, pois `build` inclui o programa original.

## Instalação de bibliotecas

`cb install` apenas habilita os módulos opcionais conhecidos em `.cb-libraries`. Não baixa código arbitrário nem instala pacotes de outra linguagem. O arquivo fica ao lado da fonte. A interface Discord não é anunciada como funcional pelo instalador.
