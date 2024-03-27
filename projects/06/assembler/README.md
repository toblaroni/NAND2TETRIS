# Project 06 - Assembler
This marks the end of the first half of the Nand2Tetris course. I chose to build my assembler in Rust. This was my first time using the language and I thoroughly enjoyed it.

#### Assembling a source file
Make sure in you're in the `projects/06/assembler/` directory.
```
    $ cargo run -- <path-to-source-file>
```

#### Explanation
The assembler is simple, the main focus of this course is to deepen the understanding of how computers work from the ground up. Therefore the error checking is not thorough and we can assume that source files are **error-free**.

The assembler assembles source files into **hack assembly** which is the course's own machine language which is specified in the book. It assembles source files in 2 passes.

##### Pass 1
In the first pass we just want to add **labels** to the **symbol table**. We do this by adding `(val, addr)` to the table, where `val` is the name of the label and `addr` is the value it's associated. We can get `addr` by counting the number of **a-instructions** and **c-instructions** as we go through the source file. 

##### Pass 2
In the second pass we parse and translate each instruction. When we encounter a symbolic **a-instruction** we look up the value in the symbol table. If a value is found then we replace the symbol with that value. If no value is found then the value must be a **variable**. In this case we add `(val, n)` to the **symbol table**, where `val` is the name of the variable and `n` is the next available space in RAM. Allocated RAM addresses start at 16, therefore the value of `n` will be `x + 16` where `x` is the number of variables we have encountered so far (which we keep track of in the symbol table struct).

For more information about the hack assembly language see the Nand2Tetris book.