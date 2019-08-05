Rust Language
=============

Rust is an interesting language. It has a steeper learning curve than any other
programming language I have tried before. However, I feel like rust has taught
me new things, and has given me a different perspective on programming. For that
alone, I'd recommend giving it a try.

I'm not sure where rust is headed as a language, but there is a lot to like
so far. It's performant, low level, and safe. Rust forces you to really think
about what your program is doing. If you are doing something that could cause
issues or haven't fully thought through your code, the compiler will usually
yell at you about it.

One of my complaints is that the borrow checker seems to add a lot of friction
to the development cycle. Many will say that this is okay since it saves you
time chasing bugs later, but I don't have enough experience with the language
to verify that claim. I also have a feeling that once you're comfortable with
the language, that friction is reduced significantly. Hopefully I'll reach that
point with more practice.

Rust isn't perfect, at times it can be maddening just getting your code to
compile, but it's usually rewarding once it does. I've been enjoying working
with it. I'm really looking forward to putting more time into learning the
language. The more I use rust, the more I think it has a bright future indeed.

## Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

I started using rust via `brew`, however, after playing around with it for a
bit I switched to `rustup`. Its super easy to manage and install toolchains
with. I recommend it if you are planning on using rust for development.

## Basics

Most of the basic examples/snippits were created by following along with the
[rust book](https://doc.rust-lang.org/book/). You can also view the rust book locally.

```bash
rustup docs --book
```

I use `rustup docs` quite a bit. Most of the issues you run into can be
resolved by searching through the provided documentation.

## Cargo Projects

The cargo apps were created by running `cargo new <CRATENAME> --vcs none`, the
`--vcs none` was added so that cargo doesn't init another repo inside this one.

The [small-web](small-web) package was built by following along with the final
project in the rust book. I also wanted to do a project from scratch. The
[b64-encode](b64-encode) crate is what I came up with for a more real world
project. It's a cli tool that can be used to convert binary data to
[base 64](https://en.wikipedia.org/wiki/Base64). For usage run the following in
the `b64-encode` create.

```bash
cargo run -- -h
```

