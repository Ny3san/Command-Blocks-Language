# Minecraft backend

[Manual](README.md) · [Integrations](integrations.md)

```text
cb convert examples/pontos.cb --target minecraft --mc-version 1.21.1 --namespace points --output my_pack
```

Supported profiles are Java 1.21 and 1.21.1, pack format 48. Other versions fail explicitly. [Official 1.21 release notes](https://www.minecraft.net/pt-pt/article/minecraft-java-edition-1-21).

The whole program is validated before publishing files. Existing output directories are never replaced. Copy the result into a test world's `datapacks` directory, use `/reload`, and inspect game output/logs. No in-game validation has been completed for this project yet.

Global integers/booleans become scoreboard fake players. Parameterless functions become `.mcfunction` files. Print becomes tellraw; conditions become execute plus helpers. Literal repeat counts between zero and 1000 generate repeated helper calls. Start/tick handlers connect to the corresponding function tags.

Generated files include `pack.mcmeta`, `cb-report.json`, `data/minecraft/tags/function/{load,tick}.json`, and `data/<namespace>/function/` with load, tick, user and internal functions. The report records profile, scores and objective. Scores are global, not per-player. Reload resets globals; an already-existing objective can produce a harmless creation warning.

A command block can invoke an installed datapack function with `function namespace:path`. It cannot execute `.cb` source directly. Calling `points:load` reruns initialization. Internal `user/fN` identifiers are not a stable API; predictable public exports are proposed.

Basic Minecraft calls: `message(selector,text)`, `give_item(selector,item,count)`, `teleport(selector,x,y,z)`, `weather(text)`, `time(integer)`. Selectors are simple @a/@p/@r/@s/@e or player names; normal executor rules apply. Item validation currently accepts only diamond, stone, dirt, apple, stick, iron_ingot, gold_ingot, emerald, diamond_sword, bow and arrow, prefixed with `minecraft:`.

Not convertible: locals, function parameters/returns, recursion, while, collections, decimals, division/remainder, mutable text, external APIs or arbitrary custom events. Literal text constants and text/integer message concatenation are supported. Dynamic booleans can control conditions but are not printable as messages in this backend. A static complexity bound limits expansion.

Keyboard input, hotbar control and mouse interaction require a client integration; they are not implemented by this datapack backend. The test command executor covers only emitted scoreboard/control-flow behavior and does not replace Minecraft itself.
