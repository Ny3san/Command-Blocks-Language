# Critérios de conclusão da meta

Estado: **em execução; não concluída**. Este arquivo define como verificar a entrega, não afirma que os itens já funcionam.

## Uso independente

- O usuário escreve apenas Command Blocks, em português ou inglês.
- `cb` e aplicativos empacotados executam em uma máquina sem Python, Node, Rust ou Cargo.
- Dependências do próprio sistema operacional são documentadas; DLLs da toolchain não são exigidas para executar a distribuição.
- Bibliotecas opcionais não exigem que o usuário programe em outra linguagem.

## Linguagem e CLI

- Lexer, parser, análise semântica, IR e mensagens bilíngues com localização.
- Variáveis, constantes, tipos, coleções, funções, condições, loops e eventos com comportamento definido.
- Módulos, testes, tratamento de erros, JSON, arquivos, rede e processos com permissões explícitas.
- Tarefas e temporizadores com cancelamento, limites e testes de concorrência.
- Todos os comandos documentados executáveis; nenhum comando apresenta stub como operação bem-sucedida.

## Aplicativos

- Criar projetos simples de terminal e de janela, sem estruturas desnecessárias.
- Abrir janela, exibir conteúdo, receber entrada, reagir a botões e fechar sem travar.
- Empacotar e testar aplicativos fora da pasta do compilador e com PATH reduzido.
- Plataformas efetivamente testadas identificadas; nenhuma promessa de portabilidade não comprovada.

## Discord

- Transporte real autenticado, ciclo de conexão, eventos, envio/resposta e desconexão.
- Tokens protegidos e permissões verificadas; limites, reconexão e falhas de rede testados.
- Testes unitários e de protocolo com serviços simulados, separados de testes ao vivo.
- Uma interação real em servidor de teste exige credencial e autorização do responsável. Sem isso, o estado é “teste ao vivo pendente”, nunca “100% validado”.

## Minecraft

- Converter o subconjunto anunciado com equivalência testada e erros para incompatibilidades.
- Nomes públicos de funções para command blocks e geração por perfis de versão.
- Validar datapack em servidor real com versão registrada.
- Ponte com autenticação, autorização, confirmação de execução e limites.
- Para hotbar/teclas: evento de entrada do cliente, consulta real de estado, seleção/uso, confirmação, cancelamento, perda de foco e desconexão. Uma macro de slots fixos não equivale a localizar itens na hotbar.
- Nenhuma alegação de detecção de teclas por datapack puro.

## Publicação

- Documentação organizada PT-BR/EN, exemplos completos e links locais válidos.
- Licença, instruções de build, testes reproduzíveis e registro de limitações.
- Commits locais revisáveis; arquivos privados, caches e credenciais fora do Git.
- Comando de publicação com URL real quando fornecida pelo usuário; sem push ou release feitos por suposição.

## Regra de evidência

Compilar não equivale a executar. Um mock não equivale ao serviço real. Um teste de protocolo não equivale ao funcionamento dentro do jogo. Ausência de falhas em testes não permite prometer ausência absoluta de bugs ou capacidade ilimitada de “fazer qualquer coisa”. Cada resultado deve indicar o que foi testado, onde e com qual limitação.
