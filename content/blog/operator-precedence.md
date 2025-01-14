+++
title = "Better Operator Precedence in Rust"
date = 2025-01-14
description = "A Rust rewrite of Jamie Bandon's Pratt parser with operator comparisons"
+++

Jamie Brandon wrote [an example parser](https://www.scattered-thoughts.net/writing/better-operator-precedence/) that fixes some problems that Pratt parsers have.
Unlike Pratt parsers, the idea is to compare operators directly.

The example by Jamie was written in Zig.
However, I'm trying to embed it inside xrcf.
Having to deal with both Zig as well as the context in which I want to apply the algorithm is a bit too complex for my brain.
Therefore, here is Jamie's algorithm rewritten in Rust:

```rust
{{ operator_precedence() }}
```

