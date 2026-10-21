# JSON

The built-in library exchanges data with files and services. It needs no installation or permission; reading a file still requires `file` permission.

```cb
variable user = json.parse('{"name":"Alex","age":20}')
print(user["name"])
variable encoded = json.stringify(user)
assert(json.valid(encoded))
```

`json.parse(text)` returns a value, list or map. `json.stringify(value)` returns compact JSON. `json.valid(text)` returns a boolean. Portuguese aliases are `ler`, `escrever` and `validar`.

JSON null becomes the null value. Integers fitting 32 bits use the integer type; other numbers use 64-bit floating point and can lose precision. Keep large identifiers, including Discord snowflakes, as strings. Objects become maps with text keys; output sorts keys.

Limits are 1 MB and 32 nesting levels. Duplicate keys, comments, trailing commas, malformed escapes and non-finite numbers are rejected. `CB107` identifies the calling source location without printing private JSON contents. Secret values serialize as `[REDACTED]`. The library neither evaluates code nor accesses the network. JSON calls cannot yet be converted into Minecraft.
