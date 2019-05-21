# STM32L0x1 Peripheral Access Crate

This Peripheral Access Crate for STM32L0x1 chips was auto-generated using the [svd2rust](https://docs.rs/svd2rust) utility. The following commands were used in a new library crate with a `Cargo.toml` file set up according to the documentation:

```
svd2rust --nightly -i STM32L0x1.svd --target cortex-m

rm -rf src/

form -i lib.rs -o src/ && rm lib.rs

cargo fmt
```

# Dependencies

You will need to have installed the general [embedded Rust prerequisites](https://rust-embedded.github.io/book/intro/install.html), as well as some Cargo utilities which you may already have installed:

```
cargo install svd2rust
cargo install form
rustup component add rustfmt
```

Also, embedded Rust language features are under active development; using `nightly` builds is not a bad idea if you want to try some of the newer features as they are added.

Case in point, omitting the `--nightly` flag from the `svd2rust` command will disable its ability to generate mappings for registers which share overlapping memory boundaries. But if you use `nightly` features, you need to use the `nightly` channel of Rust's core utilities. You can switch from `stable` with:

```
rustup update nightly
rustup default nightly
```

You might also need to re-install cross-compilation targets on the nightly channel, e.g. `rustup target add thumbv6m-none-eabi` for Cortex-M0/M0+ targets.
