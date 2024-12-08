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
And to be clear, I'm not saying that the other frameworks are bad or that I am an expert compiler developer.
Sometimes it's just about saying "How hard can it be?".

Now that I'm a few months into the project, I often find myself being surprised by how hard it actually is.
But it shouldn't be, right?
There are no side-effects like databases, file IO, or network requests.
You just read the files containing the source code, do some processing, and print the compiled code.
Everything happens inside memory.
Also, there are many great open source compilers out there that I'm basing my code on.
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
                    ^ Imagine the parser is here.
```

And now we want to set the parent of `y = 1` to be `add_one(x)`.
However, we are not yet done parsing `add_one` since we still need to parse the second child: `return x + y`.
So we have to create the object for `add_one` first without setting its children.
Then, we pass this unfinished object to the parser of the children `y = 1` and `return x + y` so that the parent can be set to `add_one`.
Once these children are parsed, we can set them to be the children of `add_one` and are done.

## Imperative Code

Next to the side-effects, mutability, and pointers, there is also the question of how to describe the rewrite rules.
Let's consider again this rewrite:

```yaml
├── function: add_one(x)
    ├── assignment: y = 1
    └── return: x + y
```

to this:

```yaml
├── function: add_one(x)
    └── return: x + 1
```

Now how would we describe this rewrite in code?
Preferably, we would do this in a declarative way.
MLIR uses a pattern language to describe these kinds of rewrites.
For example, the rewrite for `addi(addi(x, c0), c1)` to `addi(x, c0 + c1)` is described as
[follows](
https://github.com/llvm/llvm-project/blob/6c062afc2e6ed4329e1e14cb011913195a5356fa/mlir/lib/Dialect/Arith/IR/ArithCanonicalization.td#L42-L48):

```cpp
def AddIAddConstant :
    Pat<(Arith_AddIOp:$res
          (Arith_AddIOp $x, (ConstantLikeMatcher APIntAttr:$c0), $ovf1),
          (ConstantLikeMatcher APIntAttr:$c1), $ovf2),
        (Arith_AddIOp $x, (Arith_ConstantOp (AddIntAttrs $res, $c0, $c1)),
            (MergeOverflow $ovf1, $ovf2))>;
```

So then I guess our `add_one` rewrite would be something like this:

```cpp
def AddOne :
    Pat<(Add $x, (ConstantLikeMatcher APIntAttr:$c1)),
        (Add $x, (Arith_ConstantOp (AddIntAttrs $res, $c0, $c1))>;
```

Now my usual problem with declarative code is that it works great until it doesn't.
There are always cases that cannot be expressed in declarative code.
And then you are left with handling the most complex cases in imperative code, while you have done all the easy cases in declarative code.
Also the codebase is then a mix of declarative and imperative code.
Maybe that's still better than having to do everything imperative.
I'm not sure yet.
It does appear complex, that's all I'm saying for now.

The alternative is to write imperative code.
For example, we could write a rewrite rule that looks like this:

```python
def rewrite(op: Add):
    if op.rhs.definition is not None:
        if op.rhs.definition.op == Constant:
            new_rhs = Arith_ConstantOp(op.rhs.definition.value)
            op.rhs.replace(new_rhs)
```

This somehow is to me also not that easy to understand.
It's different code than for example:

```python
def count_vowels(s):
  vowels = "aeiou"
  count = 0
  for char in s:
    if char.lower() in vowels:
      count += 1
  return count
```

Although this example is longer than the rewrite above, it's much easier to understand.
I'm not sure what the reason is for this.
Maybe it's because of the pointers such as `op.rhs.definition`, or all the non-standard data types such as `Arith_ConstantOp`?
Maybe it's just because my brain is not used to it yet.

## What Other People Say

After writing this post, I also looked online to see what other people say.

There is [shipreq](https://web.archive.org/web/20210122001929/https://blog.shipreq.com/post/compilers_are_hard), who also mentions that the many combinations that are possible make it hard.
As a special case, this is especially difficult for error messages too since all the different invalid cases have to be handled.


## A More Positive Note

So why is writing a compiler so hard?
Currently, I think it's because of the side-effects, mutability, and pointers.

To end on a more positive note, I want to mention that I think there are also some enjoyable aspects of writing a compiler.
One is that when you look at the compiler as a whole, it is deterministic and without side-effects.
Put differently, given the same input, the compiler will always produce the same output.
At the same time, the inputs and outputs are all in textual form.
This makes reproducing bugs and writing tests very easy.
This is in contrast to testing graphical user interfaces, where most of the tests cannot be (fully) automated.

Another enjoyable aspect is that the problems are hard but fair.
If the program crashes, it's probably your fault.
You cannot blame the internet provider, operating system, other software, or the hard disk.
No.
If the compiler crashes, it's probably your fault.
The only thing that you depend on is reading a bunch of files and writing to stdout.

So that's why I'll keep working on this framework.
It's hard, but fun.
Hopefully the biggest sources of complexity can be moved into the framework, so that other people don't have to deal with them.
Now if you after reading this became less interested in compilers, then that's fine.
If after reading this you became more interested, feel free to check out the [project on GitHub](https://github.com/rikhuijzer/xrcf).
Contributions as well as complaints are welcome!

Okay now that this is written up, time for me to go back to writing code.
178 commits done.
One thousand to go.