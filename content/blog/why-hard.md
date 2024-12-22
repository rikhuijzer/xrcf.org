+++
title = "Why is Building a Compiler so Hard?"
date = 2024-12-21
description = "Thoughts on why writing a compiler is surprisingly difficult."
+++

Last year, I've spent a few months experimenting with and contributing to various compilers.
I had great fun, but felt that the developer experience could be better.
The build systems were often hard-to-use, and the tooling was often complex enough for things like "jump to definition" to not work.
So that's why I started to write a new compiler framework a few months ago.
And to be clear, I'm not saying that the other frameworks are bad or that I am an expert compiler developer.
Sometimes it's just about saying "how hard can it be?"

Now that I'm a few months into the project, I often find myself being surprised by how hard it actually is.
But it shouldn't be, right?
There are no side-effects like databases, file IO, or network requests.
You just read the files containing the source code, do some processing, and print the compiled code.
Everything happens inside memory.
Also, there are many great open source projects out there that I'm basing my code on.
So it should be easy.

However, when working on a new feature or bug fix, I often find myself adding a test case and then having to think a few days about the problem before I feel like I have a good solution.
Next, implementing it is often like wading through mud.
I expect that this will become better with time because my brain will get used to it, but currently it's surprisingly hard.

That's why I want to write down my thoughts now that I still have "fresh eyes".
This could be useful for myself to understand where the difficulties are so that I can improve the framework.
And maybe it will be interesting for others too.
So let's dive in.

Remember that I said there are no side-effects?

## Side-Effects Everywhere

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

since this would avoid one addition operation.
But which steps would the compiler take to rewrite this code?
Assuming that we already parsed the code into some data structure, the steps would be something like:

1. Look at `x + y`.
1. Find the definition of `y`.
1. Notice that `y` is a constant and thus that `y` in `x + y` can be replaced by `1`.
1. Replace `y` with in `x + y` by `1`.
1. Remove `y = 1` if nobody else is using `y`.

Now there are two places where side-effects occur.
One is in step 2, where we find the definition of `y`.
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

Now when we are in step 1 of the rewrite listed above, we need to find the definition of `y`.
This means that we need to find the parent of `return x + y` in the data structure since only the parent knows about the sibling `y = 1`[^1].
Hence, we need a pointer inside the `return x + y` object that points to its parent.
Or we need a pointer inside the `y` object that points to the definition of `y`.
In both cases, this pointer has to be set when creating the data structure and then updated during rewriting.
It's all not impossible, but it does add complexity.

## Mutability Everywhere

Related to the side-effects and pointers, we need to accept mutability to make the compiler fast.
Mutability makes the data structure harder to understand, because it's less clear what state the object is in at some point in time.
To explain why we need mutability, consider the data structure again but now for a function with 100 lines of code.
This would mean a data structure with 100 nodes.

Then the rewrite shown above would make changes to only a few of these nodes.
Thus, if we would make everything immutable, we would need to copy over almost all the nodes.
This would be very inefficient.

Relatedly, we need mutability in order to be able to set the pointer to the parent.
For example, when parsing the code, at some point we have parsed the function definition for `add_one` and start parsing the assignment `y = 1`:

```yaml
├── function: add_one(x)
    ├── assignment: y = 1
                    ^ Imagine the parser is here.
```

And now we want to set the parent of the `y = 1` object to be the `add_one` object.
However, we weren't able to construct the `add_one` object yet since we are still parsing the children `y = 1` and `return x + y`.
So we have to create the object for `add_one` first without setting its children.
Then, we pass this unfinished `add_one` object to the parser of the children and set it as the parent.

Once the parser is done parsing the children:

```yaml
├── function: add_one(x)
    ├── assignment: y = 1
    └── return: x + y
                     ^ Imagine the parser is here.
```

We can finally complete the `add_one` object by setting the children.

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
It does appear complex, that's all I'm saying.

The alternative is to write imperative code.
For example, we could write a rewrite rule that looks like this:

```python
def rewrite(op: Add):
    if op.rhs.definition is not None and if op.rhs.definition.op == Constant:
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

Although `count_vowels` is longer than the rewrite above, I think it's much easier to understand.
I'm not sure what the reason is for this.
Maybe it's because of the pointers such as `op.rhs.definition`, all the non-standard data types such as `Arith_ConstantOp`, or because the operations (like `replace`) mutate things in the background?
Maybe it's just because my brain is not used to it yet.

## Other Difficult Parts

Other people have also noticed that certain parts are complex.
For example, A famous [post by ShipReq](http://web.archive.org/web/20210121042722/https://blog.shipreq.com/post/compilers_are_hard) observed that you need mutability for performance, which complicates things, and the (too) many combinations that are possible.
Furthermore, [Basile Starynkevitch on StackOverflow](https://softwareengineering.stackexchange.com/a/273711/324697) argues that the main difficulty is in implementing all the many optimizations which are necessary to make a competitive compiler.
Especially the middle-end optimizations (so after parsing and before going into target-specific optimizations).
Similarly, [m-in, SeatedInAnOffice, chickyban on Reddit](https://www.reddit.com/r/Compilers/comments/1hjkb49/why_is_building_a_compiler_so_hard) say that building a compiler is not necessarily the hard part, but build a production-grade compiler for real users with a messy language spec is.

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
You cannot blame other software, the internet provider, operating system, or the hard disk.

So that's why I'll keep working on this framework.
It's hard, but fun.
Hopefully the biggest sources of complexity can be moved into the framework, so that other people don't have to deal with them.
Now if you after reading this became less interested in compilers, then that's fine.
If after reading this you became more interested, feel free to check out the [project on GitHub](https://github.com/rikhuijzer/xrcf).
Contributions as well as complaints are welcome!

Okay now that this is written up, time for me to go back to writing code.

[^1]: You could also decide to have each object know about its direct siblings, but then you would need to update the pointers when the siblings change.
When printing the data structure, you anyway need to know the children because you start printing at the root and then print recursively.
So that's why, currently, xrcf only has a pointer from the child to the parent and from the parent to the children.
