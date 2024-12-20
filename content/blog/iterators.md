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
So after a bit of searching online, I found three arguments that people are making.

One argument is that iterators can be optimized better by the compiler.
A hand-wavy argument for this is that since the compiler is responsible for a large part of the code, it knows more about the guarantees and thus can remove more bounds checks.

Another argument is that iterators are a zero-cost abstraction.
I've seen some blog posts demonstrating that iterators often produce exactly the same LLVM IR as loops.
I guess this depends a bit on the situation, but it does sound promising.

The third performance argument is that iterators are lazy.
So while you can also decide to make a loop lazy by pre-emptively breaking out of it, iterators are a bit more convenient.

But today a reason hit me that I think are not often mentioned.
Iterators are a workaround for when Rust frees variables too early.
Let me show an example.



