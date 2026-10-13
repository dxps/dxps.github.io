## My `dxps` GitHub Pages

The source of the website published on GitHub Pages from my `dxps` account.
It is a Rust [egui](https://github.com/emilk/egui) app compiled to WebAssembly and built with [Trunk](https://trunkrs.dev).

- `content/` — markdown for the about page and the posts (embedded at compile time)
- `static/` — images and favicon

### Develop

```sh
trunk serve        # http://localhost:8080
cargo run          # quick native window
```

### Deploy

Every push to `master` builds with `trunk build --release` and publishes `dist/` via GitHub Actions.
In the repository settings, **Pages → Source** must be set to **GitHub Actions**.
