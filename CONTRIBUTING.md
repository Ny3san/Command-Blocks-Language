# Como contribuir

Obrigado pelo interesse! Este projeto está em desenvolvimento inicial.

## Regras básicas

- Escreva **código e comentários em inglês**.
- Escreva **documentação em português**. Os exemplos de código dentro da documentação usam a forma em inglês.
- Mensagens para o usuário são bilíngues (português e inglês), como já funciona.
- **Não envie credenciais**, arquivos `.env`, pastas `target/` nem programas empacotados com dados privados.
- Não marque como pronto algo que não foi testado. Se uma parte for experimental, diga isso na documentação.

## Preparar o ambiente

Você precisa do [Rust](https://rustup.rs/) estável.

No Windows com a toolchain MSVC, instale o **Visual Studio Build Tools** com a carga de trabalho C++ e o Windows SDK. Passar em `cargo check` não prova que `cargo build` funciona.

## Verificações

Antes de enviar uma alteração, rode:

```text
cargo fmt --all -- --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
```

As mesmas verificações rodam automaticamente no GitHub Actions para Linux e Windows.

## Ao adicionar um recurso

- Inclua um exemplo `.cb` que funcione.
- Inclua testes de sucesso, de erro e, quando houver, de permissão.
- Atualize a documentação em português.
- Adicione uma linha em `CHANGELOG.md`.

## Ao corrigir um bug

Escreva um teste que falhe antes da correção e passe depois. Isso evita que o bug volte.

## Publicar no GitHub

Os commits são locais até você enviar. Depois de revisar:

```text
git remote add origin https://github.com/SEU_USUARIO/command-blocks.git
git push -u origin nome-da-branch
```

Abra um pull request para `main`. Confira `git remote -v` antes de alterar um remoto que já existe.

## Versões

Não crie uma tag de versão estável até que todas as verificações passem e a documentação esteja de acordo com o código. Descreva esta versão como desenvolvimento inicial.

## Licença

O projeto usa a licença [MIT](LICENSE). Preserve o arquivo `LICENSE` e os créditos dos contribuidores.
