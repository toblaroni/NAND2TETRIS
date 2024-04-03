/*
 * Generates assembly code from the parsed VM command.
 */

pub fn init(outputFile: String) {
    /*
     *  Opens the output file and gets ready to write into it.
     */
}


fn writeArithmetic(comand: String) {
    /*
     *  Writes to the output file the assembly code that implements 
     *  the given arithmetic command
     */
}

fn writePushPop(pushorpop: String,  // C_PUSH or C_POP
                segment: String,
                index: i32) {
    /*
     *  Writes to the output file the assembly code that implements the given command,
     *  where command is either C_PUSH or C_POP.
     */
}

fn close() {
    /*
     *  Closes the output file.
     */
}