# Optional integrations

[Manual](README.md) · [Roadmap](roadmap.md)

## Minecraft bridge

The existing library is a local HTTP **client**. No plugin/server is included.

```text
import minecraft_bridge
permission minecraft_bridge
minecraft_bridge.send("say Hello, Minecraft!")
```

Enable it with `cb install minecraft_bridge` beside the source. Set `CB_BRIDGE_URL=http://127.0.0.1:PORT/command` and `CB_BRIDGE_TOKEN` outside source, then run with `--allow minecraft_bridge`. The token must contain at least 24 letters, digits, underscores or hyphens. Missing permission, import, activation or configuration prevents connection.

Only loopback URLs are accepted. Allowed commands: `say text` and `tp Player x y z` with integer coordinates. No newline injection. Rate limit: ten requests/second/runtime. Connect, read and write operations have three-second timeouts, not a total session deadline.

The client sends `POST /command`, Bearer authorization and `{"command":"say Hello"}`. An external server must validate authentication and capabilities, perform the action and acknowledge execution with HTTP 200 and exactly `{"ok":true}` (optional whitespace). It must close the connection; chunked responses are unsupported. Extra response fields, missing acknowledgement or responses above 64 KiB fail. There are no automatic retries. A plugin must not report success merely because it accepted a request.

## Discord

**A working Discord bot is not available in 0.1.** `cb create discord-bot NAME` creates an explicitly labelled scaffold. `discord.start(secret("DISCORD_TOKEN"))` has a typed signature and permission checks, but returns CB202 because no Gateway/REST transport exists. `check` validates syntax/types, not service availability.

No token is required to create the scaffold. Never place one in source. Enabling Discord through `cb install discord` only activates its interface; no other-language package is installed and no connection is made.

## Module installation

`cb install` writes `.cb-libraries` in the current project directory. The runtime expects this beside its source or packaged executable. It is not a public package registry. Database, web and graphics packages are not available yet.
