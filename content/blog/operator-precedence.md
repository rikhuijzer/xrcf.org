+++
title = "Better Operator Precedence in Rust"
date = 2025-01-14
description = "A Rust rewrite of Jamie Bandon's Pratt parser with operator comparisons"
+++

Jamie Brandon wrote [an example parser](https://www.scattered-thoughts.net/writing/better-operator-precedence/) that fixes some problems that Pratt parsers have (see also [matklad](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing) for a very nice writeup on Pratt parsers).
The idea in Jamie's version is to compare operators directly.

The example by Jamie was written in Zig.
However, I'm trying to embed it inside xrcf, which is written in Rust.
Having to translate both the Zig language and the algorithm at the same time is a bit too confusing for me.
So here is Jamie's algorithm rewritten in Rust:

```rust
{{ operator_precedence() }}
```

These tests pass with Rust 1.84.0.

A full cargo project is available at
<https://github.com/rikhuijzer/xrcf.org/tree/main/content/blog/operator_precedence/>.
