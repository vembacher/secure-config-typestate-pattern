# Using the Typestate Pattern to avoid insecure misconfiguration

Idea: leverage typestate pattern via a builder to prevent misconfigurations through compile time verification.

Note: this is far from being a fleshed out concept and the issue of having a verifiable secure configuration is a
challenging topic.

## Configuration

We use the `build.rs` file to generater a config file. Which allows us to use compile-time verification without having
to rebuild our application.
A current limitation is that `secure` can be `false` due to library limitations,
it might be interesting create [typed-builder](https://crates.io/crates/typed-builder) style libraries that allow us to set constraints on which options can
co-exist.

```rust
use library::Configuration;

fn main() {
    Configuration::builder()
        .secure(true) // this is required or compilation fails.
        .build()
        .to_file("config.json");
}
```

In our simple example the application is very simple. It loads the compilation from the generated file
in `application/config.json` and runs the application.

```rust
use std::path::PathBuf;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    library::run(PathBuf::from(argv[1].as_str()));
}

```

We can run it through this command:

```shell
cargo run --package application --bin application -- application/config.json  
# Output: Successfully ran with Configuration { secure: true }
```
