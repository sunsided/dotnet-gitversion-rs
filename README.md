# dotnet-gitversion for Rust builds

Example use:

```console
$ cargo run --package examples --example gitversion

    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/markus/dev/EigeneSources/rust/dotnet-gitversion-rs/target/debug/examples/gitversion`

Display: 0.1.0
Debug:   0.1.0+0.Branch.master.Sha.2c2a2be063ce6d4b7621b05bec4fbc92a7005667
SHA:     2c2a2be063ce6d4b7621b05bec4fbc92a7005667
Commit:  2021-06-19

```

## Usage

Add `dotnet-gitversion` to your build dependencies:

```toml
[build-dependencies]
dotnet-gitversion-build = { path = "../gitversion", version = "*" }
```

Create or update your `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotnet_gitversion_build::build()?;
  Ok(())
}
```

This generates the file `gitversion.rs` in the `OUT_DIR` build directory.
After including the file you have access to the static `GIT_VERSION` variable.

```rust
include!(concat!(env!("OUT_DIR"), "/gitversion.rs"));

fn main() {
  println!("Display: {}", GIT_VERSION);
  println!("Debug:   {:?}", GIT_VERSION);
  println!("SHA:     {}", GIT_VERSION.sha);
  println!("Commit:  {}", GIT_VERSION.commit_date);
}
```

Example:

```text
Display: 0.1.0-metrics.18
Debug:   0.1.0-metrics.18+Branch.feature-metrics.Sha.defb2e23ce68c68d3bcf45333bd8718cd73368a3
SHA:     defb2e23ce68c68d3bcf45333bd8718cd73368a3
Commit:  2021-06-19
```
