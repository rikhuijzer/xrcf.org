+++
title = "A New Compiler for the Arnold Schwarzenegger Language"
date = 2024-12-02
description = "As an example of how to use xrcf to write a compiler, there is now a basic ArnoldC compiler in the repository."
+++

With the release of version 0.4, there is now a basic ArnoldC compiler in the repository.
The full code can be found [here](https://github.com/rikhuijzer/xrcf/tree/v0.4.0/arnoldc).
In this blog post, I will show how the compiler can be used.
To follow along, you can either clone the repository and run:
```sh
$ cargo install --path arnoldc
```
Or download the binary from the [v0.4.0 release page](https://github.com/rikhuijzer/xrcf/releases/tag/v0.4.0).

The ArnoldC language is based on one-liners from Arnold Schwarzenegger movies.
This is what a valid "Hello, World!" program looks like:

```arnoldc
IT'S SHOWTIME
TALK TO THE HAND "Hello, World!\n"
YOU HAVE BEEN TERMINATED
```

Here, `IT'S SHOWTIME` means "begin main", `TALK TO THE HAND` means "print", and `YOU HAVE BEEN TERMINATED` means "end main".

So before we use the compiler, let's see whether the installation was successful:

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
      --convert-experimental-to-mlir  Convert experimental operations to MLIR
      --convert-func-to-llvm          Convert function operations to LLVM IR
      --convert-mlir-to-llvmir        Convert MLIR to LLVM IR
      --convert-arnold-to-mlir        Convert ArnoldC operations to MLIR
      --compile                       Compile the code
  -h, --help                          Print help
  -V, --version                       Print version
A compiler for the ArnoldC language
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
To do so, let's convert the MLIR code to LLVM IR by running all the required passes in order:

```sh
$ arnoldc --convert-arnold-to-mlir --convert-experimental-to-mlir --convert-func-to-llvm --convert-mlir-to-llvmir hello.arnoldc
```

This prints:

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

Remembering these passes and in the order in which to run them is cumbersome, so let's use the `compile` flag, which is a wrapper around the above command:

```sh
$ arnoldc --compile hello.arnoldc
```

It returns the same LLVM IR as before.

To run our compiled code, we can use the LLVM interpreter via the `lli` command.
`lli` executes programs written in the LLVM bitcode format.
This executable is part of the LLVM project, so it can usually be installed via the package manager.
For example, on MacOS, `brew install llvm`.

Let's run our compiled code:

```sh
$ arnoldc --compile hello.arnoldc | lli
Hello, World!
```

Although the compiler is still far from complete, there is one more thing we can do.
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
because `@NO PROBLEMO` in ArnoldC is equivalent to the boolean `true`.

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

Let's run the compiled code:

```sh
$ arnoldc --compile print.arnoldc | lli
x: 1
```

Uhm and I will not show the LLVM IR this time because it's nearly unreadable.
That's why we use a compiler in the first place, right?

Anyway, as a final example, let's see what happens if we write invalid code:

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

Messages like this are also part of the framework.

So in summary, this blog post demonstrated the ArnoldC compiler which was written using the xrcf framework.

This concludes the walkthrough, or as Arnold would say:

```text
YOU HAVE BEEN TERMINATED
```

## Next Steps

To learn how to build your own compiler, see the files inside the [`arnoldc` directory](https://github.com/rikhuijzer/xrcf/tree/v0.4.0/arnoldc).
It is split into three parts:

1. `src/main.rs` defines the command line interface.
1. `src/arnold.rs` specifies how to parse the ArnoldC code (convert the text to data structures).
1. `src/arnold_to_mlir.rs` contains the `--convert-arnold-to-mlir` pass, which converts the ArnoldC code to MLIR.

All other passes such as `--convert-func-to-llvm` are implemented in the `xrcf` crate.

Although the compiler framework is not yet feature complete, if you want to build your own compiler, here are some modern compiler projects that could serve as inspiration:

- [jax](https://github.com/jax-ml/jax): A Python library for accelerator-oriented computing
- [triton](https://github.com/triton-lang/triton): A Python library for high-performance computation on GPUs by OpenAI.
- [tvm](https://tvm.apache.org/): A end to end machine learning compiler framework for CPUs, GPUs, and accelerators.
- [torch-mlir](https://github.com/llvm/torch-mlir): Compiles PyTorch to MLIR.
- [mlir-hlo](https://github.com/llvm/mlir-hlo): A set of transformations from TensorFlow HLO to MLIR.
- [Flang](https://flang.llvm.org/docs/): A LLVM-based Fortran compiler.
- [circt](https://github.com/llvm/circt): A compiler for hardware design.
- [mojo](https://www.modular.com/mojo): A new programming language for AI by Modular.

Or you could build a compiler for a different movie star.
Or your favorite tensor processing unit.
It's up to you.