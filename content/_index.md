+++
title = "Home"
+++

<div class="hero">
    <div style="margin-left: auto; margin-right: auto; text-align: center; max-width: 55ch;">
        <h1 class="project-name" style="font-size: 6vh; margin-bottom: 0px;">xrcf</h1>
        <p style="font-size: 2vh; margin-top: 0px;">eXtensible and Reusable Compiler Framework</p>
        <p style="margin-top: 8vh; font-size: 2.5vh; line-height: 150%;">
            A compiler framework for building your own compiler that can generate code for CPUs, GPUs, TPUs, and beyond.
        </p>
    </div>
</div>

<div id="status"></div>
<center>
    <h2>Status</h2>
</center>

In the long term, the aim for xrcf is to allow building compilers that can compile any programming language to any target architecture.

In the near term, the aim is to build a fully functional compiler in xrcf that can compile the ArnoldC language to an executable.
To see the compiler in action, see the [walkthrough](/blog/basic-arnoldc).
ArnoldC is just a test case.
If xrcf can handle it well, then it will be useful for other compiler projects too.

<center>
    <h3>Lowering to CPU</h3>
</center>

In the table below, a checkmark ✅ means that at least one implementation exists which can lower the construct to code that can be executed on the CPU (via LLVM).

Construct | MLIR | LLVM dialect | LLVM IR
--- | --- | --- | ---
functions | ✅ | ✅ | ✅
add | ✅ | ✅ | ✅
print | ✅ | ✅ | ✅
if else | | ✅ | ✅
for loop | | |
while loop | | |
... | | |

For example, this table means that print can be lowered from MLIR to the LLVM dialect and then to LLVM IR.
This means that to get your own `print` operation to run on the CPU, you only need to convert your own `print` operation to MLIR.
From there, xrcf can be used to run your code on the CPU.

<center>
    <h3>Lowering to GPU</h3>
</center>

Will be implemented soon.