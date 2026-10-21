# Publicar no GitHub / Publish to GitHub

Os commits são locais. Não existe remoto configurado nem release publicada automaticamente. / Commits are local. No remote or release is configured automatically.

Depois de revisar os commits, criar um repositório vazio no GitHub e substituir a URL abaixo pela URL real:

After reviewing the commits, create an empty GitHub repository and replace the URL below with its actual URL:

```text
git remote add origin https://github.com/SEU_USUARIO/command-blocks.git
git push -u origin main
```

Se um remoto já existir, confira `git remote -v` antes de alterá-lo. / If a remote already exists, inspect it before changing it.

Não crie uma tag de versão estável até concluir as verificações de [contribuição](../CONTRIBUTING.md). Descreva esta versão como desenvolvimento inicial; Discord, GUI e automação de hotbar não estão implementados. Não prometa compatibilidade com todas as versões de Minecraft.

Do not tag a stable release until the [contribution checks](../CONTRIBUTING.md) are complete. Describe this as an early development version; Discord, GUI and hotbar automation are not implemented. Do not claim support for every Minecraft version.

O compilador é Rust, mas os usuários escrevem apenas `.cb`. Uma distribuição binária deve incluir o executável nativo verificado, licença e documentação. A licença atual é [MIT](../LICENSE).

The compiler is Rust, but users write only `.cb`. A binary distribution should include a verified native executable, license and documentation. The current license is [MIT](../LICENSE).
