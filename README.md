### World's Simplest Hybrid Static Compiler and Interpreter

_What_: 
> In math class we used to use TI-86 calculators to evaluate arithmetic expressions, do computations, etc. The grammar we express math in terms of follows a strict order of operations that get parameterized by the numbers in our expression. For example, `1 + 1` and `((5 / 3.502) * (67^2 * 0.239))/4` conform to an order of operations.
> This is actually a lexical grammar.
>
> When compiling statically-linked computer programs, the richness and vocabulary includes all of the same arithmetic we learned in school, plus some more stuff.
>
> This crate demonstrates that truth by treating "Math" as a programming language, and `tix86` being a compiler that supports x86_64 executable generation.
