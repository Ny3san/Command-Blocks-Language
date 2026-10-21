# Diagnostics and compiler development

Status: **implemented** for bilingual diagnostics with a code, file, line, column and suggestion. Source excerpts, visual markers and multiple-error recovery remain **planned**.

```text
cb check examples/ola.cb
cb check examples/hello.cb --lang en
```

Messages retain the existing CBxxx codes and stderr output. Refactoring their text storage does not change `.cb` programs, permissions, message formatting or exit codes.

For Rust library consumers: `Error.pt`, `Error.en`, `Error.hint_pt` and `Error.hint_en` are now `Box<str>` instead of `String`. They do not need spare capacity for growth. Prefer `Error::new(...).hint(...)`; these method signatures are unchanged. For direct field construction, convert a `String` with `.into_boxed_str()` or `.into()`; use `.to_string()` to obtain a `String`.

The `localized_diagnostic_preserves_location_code_and_hint` test verifies complete PT/EN output, location, suggestions, cloning and Display. The language suite retains syntax, type and permission error tests. No lint was disabled for this refactoring.

Limit: this change does not resolve all existing diagnostic code collisions, add structured JSON diagnostics or prove live external integrations.
