# Política de segurança

## Versões suportadas

Apenas a branch `main` recebe correções de segurança. Não há versão publicada ainda.

## Como reportar uma vulnerabilidade

Não abra issue pública para vulnerabilidades. Envie um relato privado pelo recurso
**Security > Report a vulnerability** do GitHub deste repositório, ou pelo contato
do mantenedor no perfil do GitHub.

Inclua:
- versão ou commit afetado;
- passos para reproduzir;
- impacto esperado;
- se possível, um programa `.cb` mínimo que demonstre o problema.

Respondemos em até 7 dias corridos com confirmação e prazo estimado.

## Escopo relevante

A linguagem tem recursos que acessam o sistema: arquivos, rede (`net.*`), janelas
(`window.*`), Discord e a ponte HTTP. Esses recursos exigem permissões explícitas
(`--allow-*`). Relatos sobre contorno dessas permissões, execução de código fora do
interpretador ou vazamento de segredos (`secret`, tokens do Discord) são prioritários.
