[![Rust](https://github.com/MatrixSenpai/riot-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/MatrixSenpai/riot-rs/actions/workflows/rust.yml)

# riot-rs

### Installation

This crate is separated into multiple features. Use any combination of them to fit your needs:

- lol
- tournament
- clash
- tft
- val

By default, `lol` is enabled. Add the following to your `Cargo.toml`
```toml
[dependencies.riot-api]
version = "0.1.2"
# If you don't want `lol` feature
default-features = "false"
# If you want additional features
features = ["clash", "tournament", "tft"]
```

This crate can interact with Riot's RSO with default features off, or any combination of features

### Endpoints Implemented

See [EndpointTasks](EndpointTasks.md)

### TODO
- [ ] More complete readme
- [ ] Endpoints (see below)
- [x] Rate limit handling
- [x] Set up feature flags
- [x] Tests & CI
  - [ ] More tests

riot-rs isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties.
Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.