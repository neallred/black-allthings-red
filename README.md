# black.allthings.red

Blank, black homepage.

## build

```
cargo build --target x86_64-unknown-linux-musl --release
```

Using musl c avoids glibc mismatches between build and execution environments.
