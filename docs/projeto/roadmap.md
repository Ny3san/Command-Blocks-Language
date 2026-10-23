# Roadmap

Esta página lista o que **não está pronto**. Não use esses recursos em produção.

## Linguagem

- Funções anônimas e funções como valores.
- Módulos de arquivos (importar outro `.cb`).
- Eventos com dados.
- Agendamento e execução concorrente.
- Notação de duração, como `5 segundos`.
- Códigos de erro específicos para domínio matemático. Hoje `math.sqrt(-1)` gera `CB302`, que é um código de limite.

## Biblioteca padrão

- Datas e calendário.
- Banco de dados, web, áudio e gráficos.
- Execução de processos externos.
- HTTP além de requisições simples: servidor, streaming, download e cabeçalhos de resposta.
- Conexão real com o Discord.

## Minecraft

- Funções com parâmetros no datapack.
- Exportações com nomes previsíveis para command blocks.
- Validação dentro de um Minecraft real (hoje feita por simulação).
- Versões além de Java 1.21 e 1.21.1, Bedrock, mods e plugins.

## Cliente Minecraft

Automações com teclas, hotbar e inventário dependem de um **adaptador de cliente**, que ainda não existe. Macros externas de teclado e mouse não são uma alternativa confiável.

## Ferramentas

- Formatador completo, analisador estático, REPL, depurador e Language Server.
- Gerenciador de pacotes remoto.
- Saída de erros em JSON.

## Ordem prevista

1. Base estável: testes passando, documentação revisada e exemplos executáveis.
2. Aplicativos simples de terminal.
3. Automações com espera, cancelamento e eventos com dados.
4. Integrações reais: Discord, servidor da ponte e adaptador de cliente.
5. Janelas como biblioteca opcional.

Cada item só sai desta lista depois de ter testes e exemplos reais.
