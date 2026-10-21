# Proposta: ampliar sem complicar

**Estado: proposta de produto; os recursos abaixo não foram implementados.**

O objetivo é manter uma linguagem pequena e oferecer bibliotecas por tarefa. Programas simples continuam sendo uma sequência de comandos. Eventos ficam para entradas, timers e integrações; criar uma automação não deve exigir aprender um framework.

## Três caminhos claros

| Caminho | Uso | Necessidade |
|---|---|---|
| Computador | Ferramentas, arquivos, aplicativos e automações | Executável `cb`; biblioteca opcional quando necessário |
| Minecraft servidor | Mapas, minigames e sistemas de mundo | Datapack ou futuro adaptador de servidor |
| Minecraft cliente | Teclas, hotbar e ações do jogador | Futuro mod de cliente ou integração de aplicativo |

O mesmo núcleo é reutilizado, mas capacidades diferentes não podem ser prometidas em todos os ambientes. Um command block pode chamar uma função do datapack; isso não dá ao servidor acesso ao teclado do cliente.

## Caso: apertar G e executar uma sequência

Fluxo desejado: detectar a tecla enquanto o jogo está em foco → verificar se há uma sequência ativa → localizar os itens → selecionar o slot → usar o item → aguardar confirmação → selecionar o próximo → usar → restaurar o slot original.

Para saber qual item realmente está em cada slot, a opção proposta é um **adaptador de cliente**. Ele fornece estado do inventário e ações de jogo de forma explícita. Macros externas de teclado/mouse são outra opção, mas não têm acesso semântico confiável à hotbar e dependem de foco, resolução e estado da interface. Essas alternativas não são equivalentes.

Requisitos do adaptador: tecla configurável, uma sequência por vez, cancelamento imediato, não atuar no chat/inventário, parar ao perder foco ou desconectar, prazo para cada ação, erro se item faltar e confirmação antes da etapa seguinte. O servidor continua autorizando ações; não há proposta de contornar suas regras ou mecanismos de proteção.

Detecção de teclas personalizadas é uma capacidade de cliente, como documentado pelo [Fabric](https://docs.fabricmc.net/develop/key-mappings). Essa referência fundamenta a divisão de responsabilidades; não afirma compatibilidade pronta de um adaptador com uma versão específica do jogo.

Os nomes de funções e a sintaxe de atalhos ainda serão definidos. Nenhum exemplo dessa proposta deve ser apresentado como código `.cb` executável hoje.

## Ordem sugerida

1. **Base publicável:** testes passando, documentação em dois idiomas, links válidos, exemplos executáveis e limites conhecidos.
2. **Uso mais simples:** evoluir o template `cb create app` e o guia de iniciantes já adicionados; criar exportações Minecraft com nomes previsíveis para command blocks.
3. **Automações:** tarefas com espera não bloqueante, cancelamento e eventos com dados. Testar com adaptadores simulados antes de controlar um jogo real.
4. **Integrações reais:** transporte Discord opcional; servidor local da ponte com autenticação; adaptador de cliente Minecraft versionado.
5. **Aplicativos gráficos:** biblioteca opcional de janelas e controles, sem impor dependências ao núcleo.

Cada etapa só muda de proposta para implementada após testes e exemplos reais. A independência para o usuário permanece: escrever `.cb`, executar `cb` ou o aplicativo empacotado. Integrações podem exigir Minecraft, um mod ou acesso a um serviço, mas não programação em outra linguagem pelo usuário.
