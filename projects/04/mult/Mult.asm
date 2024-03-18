//   R2 = R1 * R0
//   Psuedo Code:
//   -----------
//     R2 = R0
//     i = 0
//     while (i < R1) {
//        R2 += R0
//        i ++
//     }
//     end

    // Set R2 to R0
    @R2
    M=0

    // Set i to zero
    @i
    M=0

(LOOP)
    // Compare i to R1
    @R1
    D=M
    @i
    D=D-M
    @END
    D; JEQ

    // R2+R0
    @R0
    D=M
    @R2
    M=M+D

    // i++
    @i
    M=M+1
@LOOP
    0;JMP

(END)
0; JMP
