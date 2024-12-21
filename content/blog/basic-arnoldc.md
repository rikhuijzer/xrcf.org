+++
title = "A New Compiler for the Arnold Schwarzenegger Language"
date = 2024-12-02
description = "As an example of how to use xrcf to write a compiler, there is now a basic ArnoldC compiler in the repository."
+++

<div class="background-info">

This is a blog post about the eXtensible Reusable Compiler Framework (xrcf).
I've always wanted to write a compiler, but found the task daunting.
I didn't know how to handle all the details such as lexing, parsing, type checking, and error handling.
That's why I'm building xrcf.
This projects handles the details so you can focus on building your own compiler.

</div>


Since the release of version 0.4, there is now a basic ArnoldC compiler in the repository.
This ArnoldC compiler is a test case for the compiler framework.
If the framework can handle this language well, then it will be useful for other languages too.
The full code for the compiler can be found [here](https://github.com/rikhuijzer/xrcf/tree/v0.5.0/arnoldc).

In this blog post, I will show how the compiler can be used to generate fast code for the CPU.
To follow along, you can either clone the repository and run:
```sh
$ cargo install --path arnoldc
```
Or download the `arnoldc` binary from the [release page](https://github.com/rikhuijzer/xrcf/releases/tag/v0.5.0).

The [ArnoldC language](https://github.com/lhartikk/ArnoldC) is based on one-liners from Arnold Schwarzenegger movies.
This is what a valid "Hello, World!" program looks like:

```arnoldc
IT'S SHOWTIME
TALK TO THE HAND "Hello, World!\n"
YOU HAVE BEEN TERMINATED
```

Here, `IT'S SHOWTIME` means "begin main", `TALK TO THE HAND` means "print", and `YOU HAVE BEEN TERMINATED` means "end main".

Before we use the compiler, let's see whether the installation was successful:

```sh
$ arnoldc --help
```

This should print:

```text
A compiler for the ArnoldC language

Usage: arnoldc [OPTIONS] [INPUT]

Arguments:
  [INPUT]  The input file (- is interpreted as stdin) [default: -]

Options:
      --convert-scf-to-cf             Convert structured control flow (scf) operations to cf
      --convert-cf-to-llvm            Convert control flow (cf) operations to LLVM
      --convert-experimental-to-mlir  Convert experimental operations to MLIR
      --convert-func-to-llvm          Convert function operations to LLVM
      --convert-mlir-to-llvmir        Convert MLIR to LLVM IR
      --print-ir-before-all           Print the IR before each pass
      --convert-arnold-to-mlir        Convert ArnoldC operations to MLIR
      --compile                       Compile the code
      --debug                         Print debug information
  -h, --help                          Print help
  -V, --version                       Print version
```

To compile ArnoldC, let's create a file called `hello.arnoldc` with the hello world program:

```arnoldc
IT'S SHOWTIME
TALK TO THE HAND "Hello, World!\n"
YOU HAVE BEEN TERMINATED
```

Next, let's see what the compiler generates when we run the `--convert-arnold-to-mlir` pass:

```sh
$ arnoldc --convert-arnold-to-mlir hello.arnoldc
```

This prints:

```mlir
module {
  func.func @main() -> i32 {
    experimental.printf("Hello, World!\0A")
    %0 = arith.constant 0 : i32
    return %0 : i32
  }
}
```

What this shows is that the compiler has converted the ArnoldC code to [MLIR](https://mlir.llvm.org/).
It also added a 0 return value to the `main` function.
This ensures that the program will return a 0 status code, which is the convention for programs that didn't crash.

Although this MLIR code looks nice (or at least more so than ArnoldC), let's get it to run.
To do so, convert the MLIR code to LLVM IR by running all the required passes in order:

```sh
$ arnoldc \
    --convert-arnold-to-mlir \
    --convert-experimental-to-mlir \
    --convert-scf-to-cf \
    --convert-cf-to-llvm \
    --convert-func-to-llvm \
    --convert-mlir-to-llvmir \
    hello.arnoldc
```

Think of each of these passes as a set of transformations.
For example, the `--convert-arnold-to-mlir` pass transforms:
```arnoldc
TALK TO THE HAND "Hello, World!\n"
```
to
```mlir
experimental.printf("Hello, World!\0A")
```

The command with all passes applied prints the following LLVM IR:

```llvm
; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare i32 @printf(ptr)
define i32 @main() {
  %3 = alloca i8, i16 15, align 1
  store [15 x i8] c"Hello, World!\0A\00", ptr %3, align 1
  %4 = call i32 @printf(ptr %3)
  ret i32 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
```

Remembering these passes and in the order in which to run them is cumbersome, so it is also possible to use the `compile` flag, which is a wrapper around the above command and produces the same result:

```sh
$ arnoldc --compile hello.arnoldc
```

To run our compiled code, we can use the LLVM interpreter via the `lli` command.
`lli` executes programs written in the LLVM bitcode format.
This executable is part of the LLVM project, so it can usually be installed via the package manager.
For example, on MacOS, `brew install llvm`.

Enough talk, let's run the code!

```sh
$ arnoldc --compile hello.arnoldc | lli
Hello, World!
```

Or produce a native executable:

```sh
$ arnoldc --compile hello.arnoldc | llc -filetype=obj -o hello.o
$ clang hello.o -o hello
$ ./hello
Hello, World!
```

Although the compiler is still far from complete (see [status](/#status) for details), there is one more thing we can do.
We can print a variable:

```arnoldc
IT'S SHOWTIME

HEY CHRISTMAS TREE x
YOU SET US UP @NO PROBLEMO

TALK TO THE HAND "x: "
TALK TO THE HAND x

YOU HAVE BEEN TERMINATED
```
This should print:

```text
x: 1
```
because `HEY CHRISTMAS TREE x` is equivalent to what in Python would be `x =` and `@NO PROBLEMO` in ArnoldC is equivalent to the boolean `True`.

Let's see what the compiler generates.
To get readable code, we do only the `--convert-arnold-to-mlir` pass:

```sh
$ arnoldc --convert-arnold-to-mlir print.arnoldc
```

This prints:

```mlir
module {
  func.func @main() -> i32 {
    %x = arith.constant 1 : i16
    experimental.printf("x: ")
    experimental.printf("%d", %x)
    %0 = arith.constant 0 : i32
    return %0 : i32
  }
}
```

Which returns the expected value:

```sh
$ arnoldc --compile print.arnoldc | lli
x: 1
```

As a final example, let's see what happens if we write invalid code:

```arnoldc
IT'S SHOWTIME
TALK "Hello, World!"
YOU HAVE BEEN TERMINATED
```

If we now run the compiler, it will fail with a clear error message:

```sh
$ arnoldc --compile invalid.arnoldc
thread 'main' panicked at arnoldc/src/main.rs:67:60:
called `Result::unwrap()` on an `Err` value: 

---
0  | ITS SHOWTIME {
1  |   TALK "Hello, World!\n"
       ^ Unknown operation: TALK
---
```

As is expected when building a compiler, the error message point to the exact location of the error with a description of the problem.

This concludes the walkthrough, or as Arnold would say:

```text
YOU HAVE BEEN TERMINATED
```

## Next Steps

To learn how to build your own compiler, see the files inside the [`arnoldc` directory](https://github.com/rikhuijzer/xrcf/tree/v0.5.0/arnoldc).
It is split into three parts:

1. `src/main.rs` defines the command line interface.
1. `src/arnold.rs` specifies how to parse the ArnoldC code (convert the text to data structures).
1. `src/arnold_to_mlir.rs` contains the `--convert-arnold-to-mlir` pass, which converts the ArnoldC code to MLIR.

All other passes such as `--convert-func-to-llvm` are implemented in the `xrcf` crate.

If you want to contribute to the compiler framework, see [contributing](/contributing).

Although the compiler framework is not yet feature complete, if you want to build your own compiler, here are some modern compiler projects that could serve as inspiration:

- [jax](https://github.com/jax-ml/jax): A Python library for accelerator-oriented computing
- [tvm](https://tvm.apache.org/): A end to end machine learning compiler framework for CPUs, GPUs, and accelerators.
- [torch-mlir](https://github.com/llvm/torch-mlir): Compiles PyTorch to MLIR.
- [Flang](https://flang.llvm.org/docs/): A LLVM-based Fortran compiler.

Or you could build a compiler for a different movie star.
Or your favorite tensor processing unit.
It's up to you.
