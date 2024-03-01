// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen
// by writing 'black' in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen by writing
// 'white' in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// If kbd is pressed -> make screen black
// Else make screen black
//     
// Pseodo code
// -----------
//      is_white = 0  # Flag to keep track of screen colour. Helpful if we want to optimise and 
// 
//  (START)
//      addr = SCREEN
//      rows = 256  # 256 rows
//      cols = 32   # 32 16-bit words per row
// 
//      i = 0
//  (ROW_LOOP)
//      if i > rows goto COLOUR_CHANGED     # We reached the end of the screen so colours must have changed
//      j = 0
//      i ++
//  (COL_LOOP)
//      if j > cols goto ROW_LOOP

//      if KBD == 0 goto FILL_WHITE
//      
//      (FILL_BLACK)
//      if not is_white goto START  # if the screen is already black we don't need to loop
//      RAM[addr] = -1
//      addr ++
//      j ++
//      goto (COL_LOOP)

//      (FILL_WHITE) 
//      if is_white goto START  # if the screen is already white we don't need to loo
//      RAM[addr] = 0
//      addr ++
//      j ++      
//      goto (COL_LOOP)
//
//  (COLOUR_CHANGED)
//      is_white = !is_white    # We reached the end of the rows so colours must have flipped
//      goto START

    @is_white
    M=0

(START)
    @SCREEN
    D=A
    @addr
    M=D

    @256
    D=A
    @rows
    M=D

    @32
    D=A
    @cols
    M=D


    @i
    M=1

(ROW_LOOP)
    @i
    D=M
    @rows
    D=D-M
    @COLOUR_CHANGED
    D;JGT

    @i
    M=M+1

    @j
    M=1

(COL_LOOP)
    @j
    D=M
    @cols
    D=D-M
    @ROW_LOOP
    D;JGT

    @KBD
    D=M
    @FILL_WHITE
    D;JEQ

    // (FILL BLACK)
    @is_white
    D=M
    @START
    !D;JEQ

    @addr
    A=M
    M=-1

    @addr
    M=M+1

    @j
    M=M+1

    @COL_LOOP
    0;JMP

(FILL_WHITE)
    @is_white
    D=M
    @START
    D;JEQ

    @addr
    A=M
    M=0

    @addr
    M=M+1

    @j
    M=M+1

    @COL_LOOP
    0;JMP


(COLOUR_CHANGED)
    @is_white
    M=!M
    @START
    0;JMP
