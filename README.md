# raffle

R.A.F.F.L.E. aka. Rapidly Assembled Faulty Fortune Locator Engine.

A TUI application that picks a winner from a list of meetup participants.

## Structure

Generated from: [rust-tui-templates](https://github.com/tui-rs-revival/rust-tui-template)

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```
