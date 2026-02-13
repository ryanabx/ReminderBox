# Contributing

This document has information on how to compile ReminderBox.

## Dependencies

- Rust
- Cargo
- Trunk
    
    You can install trunk in many ways.

    Install trunk through cargo:
    
    ```sh
    cargo install trunk
    ```

    Ref: https://book.leptos.dev/getting_started/index.html

## Testing locally

```sh
trunk serve --open
```

## Building

```sh
trunk build --release
```

## TailwindCSS Setup

See: https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_csr

## Regenerating Favicons

Use a favicon converter, such as https://favicon.io/favicon-converter/

Copy files into icons

Make sure all icons are loaded in `index.html`