+++
title = "Why Is Writing a Compiler So Hard?"
date = 2024-12-08
description = "A discussion on why writing a compiler is fundamentally difficult."
+++

<div class="background-info">

I'm working on an easy(ier)-to-use compiler framework called xrcf.
It's open source and can be found on [GitHub](https://github.com/rikhuijzer/xrcf).

</div>

Last year, I've spent a few months experimenting with and contributing to various compilers.
I had great fun, but felt that the developer experience could be better.
The build systems were often hard-to-use, and the tooling was often complex enough for "jump to definition" to not work.
So that's why I started to write a new compiler framework a few months ago.
It's essentially written for my former self.
When I started with compilers, I wanted a tool that was easy to build and (reasonably) easy to understand. 

Now that I'm a few months into the project, I often find myself being surprised by how complex it is to write a compiler.
But it shouldn't be, right?
There are no side-effects like databases, file IO, or network requests.
You just read the files containing the source code, do some processing, and print the compiled code.
Everything happens inside memory.
So it should be easy.

However, when working on a new feature or bug fix, I often find myself adding a test case and then leaving the code sit for a few days to think about the problem.
Next, implementing it is often like wading through mud.
I expect that this will become better with time because my brain will adjust, but currently it's surprisingly hard.
That's why I want to write down my thoughts now that I still have "fresh eyes".
This could be useful for myself to understand where the difficulties are so that I can improve the framework.
And maybe it will be interesting for others too.
So let's dive in.
Remember that I said there are no side-effects?

## Side-effects Everywhere

There are plenty of side-effects inside a compiler.
For example, take the following Python code:

```python
def add_one(x):
    y = 1
    return x + y
```

Now a good compiler would look at this code and rewrite it to:

```python
def add_one(x):
    return x + 1
```

since this would avoid one addition.
But which steps would the compiler take to rewrite this code?
Assuming that we already parsed the code into some data structure, the steps would be something like:

1. For `x + y`, find the definition of `y`.
2. Notice that `y` is a constant and thus that `y` in `x + y` can be replaced by `1`.
3. Replace `y` with `1` in the `x + y` expression.
4. Remove `y = 1` since it's no longer needed.

Now there are two places where side-effects occur.
One is in step 1, where we need to find the definition of `y`.
Only if `y` is a constant, we can substitute the `y` with a constant.
Otherwise, we abort the rewrite.

The other is in step 4, where we remove `y = 1`.
This can only happen if `y` is a variable that is no longer used.
If `y` is used somewhere else, we cannot remove the assignment.

## Pointers Everywhere

Another source of complexity is that we need pointers to navigate the data structure.
To see why, let's look at the data structure.

When parsing this code, we create a data structure that looks something like this:

```yaml
├── function: add_one(x)
    ├── assignment: y = 1
    └── return: x + y
```

So we have some object that contains `add_one` with two children: `y = 1` and `return x + y`.
Next, we rewrite this code to this:

```yaml
├── function: add_one(x)
    └── return: x + 1
```

Now when we are in step 1, we need to find the definition of `y`.
This means that we need to find the parent of `return x + y` in the data structure.
Hence, we need a pointer inside the `return x + y` object that points to its parent.
Or we need a pointer inside the `y` object that points to the definition of `y`.
In both cases, this pointer has to be set when creating the data structure and then updated during rewriting.
It's all not impossible, but it does add some complexity.

## Mutability Everywhere

Related to the side-effects and pointers, we need to accept mutability to make the compiler fast.
Mutability makes the data structure harder to understand, because it's less clear what state the object is in at some point in time.
To explain why we need mutability, consider the data structure again but now for a function with 100 lines of code.
This would mean a data structure with 100 nodes.

Then the rewrite shown above would make changes to only a few of these nodes.
Thus, if we would make everything immutable, we would need to copy over almost all the nodes.
This would be very inefficient.

Relatedly, we need mutability in order to be able to set the pointer to the parent.
For example, when parsing the code, at some point we have parsed the function definition and start parsing the assignment `y = 1`:

```yaml
├── function: add_one(x)
    ├── assignment: y = 1
```

And now we want to set the parent of `y = 1` to be `add_one(x)`.
However, we are not yet done parsing `add_one` since we still need to parse the second child: `return x + y`.
So what we have to do is create the object for `add_one` first, but not yet set its children.
Then, we pass this unfinished object to the parser of the children `y = 1` and `return x + y` so that the parent can be set.
Once these children are parsed, we can set them to be the children of `add_one` and are done.

## Imperative Code



## Jargon



## Being Closer to the Hardware


To end this post on a more positive note, I want to mention that I think there are also some enjoyable aspects of writing a compiler.
For example, one is that when you look at the compiler as a whole, it is deterministic and without side-effects.
Put differently, given the same input, the compiler will always produce the same output.
This makes reproducing bugs and writing tests very easy.