# Proposal: more capabilities, a small language

**Proposal only. The features below are not implemented.**

The proposed split is desktop tools, server/world automation through datapacks, and client automation through an optional adapter. All share the language core, but do not share every capability.

For a G-key sequence, the desired flow is: detect the key while gameplay is focused, prevent overlapping sequences, find an item, select its slot, use it, await acknowledgement, select/use the next item, then restore the previous slot.

A client adapter is the proposed reliable route for inventory state. Generic keyboard/mouse macros are a separate option: they lack reliable semantic hotbar access and depend on UI state. Datapacks cannot directly provide arbitrary client key mappings. [Fabric's client key mapping documentation](https://docs.fabricmc.net/develop/key-mappings) supports this separation; it does not validate a finished adapter for any specific game version.

The adapter needs configurable keys, cancellation, focus/chat/inventory guards, disconnect handling, per-action deadlines, missing-item errors and action acknowledgements. Server authority remains intact. Names and syntax have not been finalized; no proposed code should be presented as runnable.

Suggested stages:

1. Publishable foundation: executable tests, two-language documentation, accurate capabilities and validated links.
2. Easier use: extend the newly added `cb create app` starter and add stable public Minecraft function exports.
3. Automation primitives: non-blocking waits, cancellation and event payloads, tested with simulated adapters.
4. Real optional transports: Discord, an authenticated bridge server and a versioned Minecraft client adapter.
5. Optional graphical application library.

User independence remains a requirement: write `.cb` and run `cb` or a packaged application. A game integration may need a mod, and a bot needs its service, but users should not need to program in another language.
