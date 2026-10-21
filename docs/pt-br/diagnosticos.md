# Diagnósticos e desenvolvimento do compilador

Estado: **implemented** para diagnósticos bilíngues com código, arquivo, linha, coluna e sugestão. Trecho de código, marcador visual e recuperação de múltiplos erros continuam **planned**.

```text
cb check examples/ola.cb
cb check examples/hello.cb --lang en
```

As mensagens continuam usando os códigos CBxxx existentes e stderr. A refatoração de armazenamento dos textos não altera programas `.cb`, permissões, formatação das mensagens ou códigos de saída.

Para quem incorpora a biblioteca Rust: os campos `Error.pt`, `Error.en`, `Error.hint_pt` e `Error.hint_en` agora são `Box<str>`, em vez de `String`. Eles não precisam reservar capacidade para crescimento. Prefira `Error::new(...).hint(...)`; as assinaturas desses métodos não mudaram. Na construção direta de campos, converta `String` com `.into_boxed_str()` ou `.into()`; para obter uma `String`, use `.to_string()`.

O teste `localized_diagnostic_preserves_location_code_and_hint` verifica a saída completa PT/EN, localização, sugestão, clonagem e Display. A suíte de linguagem mantém testes de erros de sintaxe, tipos e permissões. Nenhum lint foi desativado para essa refatoração.

Limite: essa mudança não resolve colisões entre todos os códigos existentes, não adiciona diagnóstico estruturado JSON e não torna integrações externas comprovadas ao vivo.
