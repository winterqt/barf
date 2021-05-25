# barf

Brainfuck ahead-of-time compilation using Rust macros.

## why?

I was talking with some friends, and decided to time myself writing a [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter. After I said I was doing this, [Mary](https://github.com/Mstrodl) said:

> But have you considered: brainfuck JIT

I had not considered that.

[NotNite](https://notnite.com) then implemented a basic implementation of this in JavaScript. But JavaScript is bad, and Rust is good, so I set out to rewrite it in Rust with the power of macros.

## usage

```toml
[dependencies]
barf = { version = "0.1.0", git = "https://github.com/winterqt/barf.git" }
```

```rust
fn main() {
    brainfuck!("+-");
}
```

Also see `examples/hello-world.rs` for the classic hello world example, which expands to [this](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fbc5dd3c6ac6f7f32734a44912658fa5).

## implementation details

I originally planned to have the macro take input such as:
```rust
brainfuck! {
    +=
};
```

This was feasible, up until I had to parse the `[` and `]` tokens. These are delimiters in the Rust language, and I tried many very weird things to try to be able to parse them individually, but eventually gave up.

### why not use `quote`?

I had some issues with parsing "partial" tokens for the `[` and `]` commands, so I stuck to string concatenation.
