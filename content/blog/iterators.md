+++
title = "Why are Iterators so Common in Rust?"
date = 2024-12-20
description = "In some languages, iterators are nice but people tell you to not overuse them. In Rust, iterators are everywhere. Why is that?"
+++

Iterators are a pretty cool concept.
For example, in Python, you can write:

```python
values = [1, 2, 3]

for i in range(len(values)):
    values[i] += 1

print(values)
```

This prints
```
[2, 3, 4]
```

With iterators, this can be written as:

```python
values = [1, 2, 3]

values = list(map(lambda x: x + 1, values))

print(values)
```

This is of course pretty cool.

So a few years ago when I was programming in Python, I obviously went overboard with this and used iterators everywhere.
However, a more senior developer told me to not overuse them.
He said that iterators are nice, but often people find it easier to read loops.
And since code is read more than it is written, writers should just accept the inconvenience of loops.

In Julia, iterators are also pretty common, but should also generally not be overused.
I don't know what the current state is, but last year the main reason in Julia was that iterators were much more difficult for the compiler to optimize.
It could do it, but if you wanted fast compilation times, you should just use loops so that's what I did.

However, in Rust, iterators are everywhere and I didn't get why.
For example, this code from [`wasmtime`](https://github.com/bytecodealliance/wasmtime) looks like pretty standard Rust:

```rust
let stdin_exist = options
    .files
    .iter()
    .find(|file| *file == Path::new("-"))
    .is_some();
```

But why?
I wouldn't say that this code is particularly easy to read for most people.
I'm not saying that this is bad, but just that it will take most people a bit of time to get used to using iterators like this.

So after a bit of searching online, I found three performance arguments that people are making.
One argument is that iterators can be optimized better by the compiler.
A hand-wavy argument for this is that since the compiler is responsible for a large part of the code, it knows more about the guarantees and thus can remove more bounds checks.

Another argument is that iterators are a zero-cost abstraction anyway.
Some blog posts even show that iterators often produce exactly the same LLVM IR as loops.
I guess this depends a bit on the situation, but it does sound promising.

The third performance argument is that iterators are lazy.
So while you can also decide to make a loop lazy by pre-emptively breaking out of it, iterators are a bit more convenient.

But today a reason hit me that I think is not often mentioned.
Iterators are a workaround for when Rust frees variables too early.
Let me show an example.

```rust
fn main() {
    let text = String::from("Hello, world!").split(", ");

    for word in text {
        println!("{}", word);
    }
}
```

If you try to compile this code (`rustc tmp.rs`), you will get an error.
It says that `String::from("Hello, world!")` "creates a temporary value which is freed while still in use".

One way to fix this is to make a separate variable:

```rust
fn main() {
    let text = String::from("Hello, world!");
    let text = text.split(", ");

    for word in text {
        println!("{}", word);
    }
}
```

It took me way too long to understand why this is happening.
The problem here is that `String::from("Hello, world!")` returns a `String` and `split` returns a `&str`.
This means that `split` does not create a new object, but instead it returns a reference to the original object.
This is very efficient, but problematic for us here for another fact about the compiler:
Rust frees objects which are not referenced at the end of the line!

So what I mean here is that `String::from("Hello, world!")` is dropped before the loop starts.
This is why the compiler gives an error.
`split` gave us a reference to the original object, but the original object is dropped before we use the reference.
If we write a separate variable, the compiler does decide to keep the original object alive until the end of the function.

So here is where iterators come in.
With iterators, we can just chain everything to keep it all one one line:

```rust
fn main() {
    String::from("Hello, world!").split(", ").for_each(|word| {
        println!("{}", word);
    });
}
```

which prints

```
Hello
world!
```

This, I think, is why iterators are everywhere in Rust.
But maybe I'm wrong, let me know if you disagree.
