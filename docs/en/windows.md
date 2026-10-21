# Native windows

The `window` library creates Windows native controls without a browser or another installed language. This implementation is undergoing validation; Linux/macOS backends are not available yet.

Run `cb run examples/window.cb --allow window`. Source must also declare `permission window` (or its Portuguese alias).

```cb
permission window
window.open("My application", 420, 240)
variable name = window.input("Steve", 20, 20, 240, 28)
variable result = window.label("", 20, 60, 340, 28)
variable button = window.button("Hello", "greet", 20, 100, 100, 32)
event greet {
    window.set_text(result, "Hello, " + window.text(name))
}
window.run()
```

Control coordinates are X, Y, width and height in pixels. Controls return integer IDs. `window.text(id)` reads text, `window.set_text(id, text)` changes it and `window.close()` closes the window. Buttons emit the named language event.

`window.run()` processes interactions until closure. Handlers execute synchronously; slow work blocks the interface. One window is supported per runtime. Do not call `run()` inside a handler. Automatic layout, menus and custom drawing are not implemented. Windows are external resources and cannot be converted into datapacks.
