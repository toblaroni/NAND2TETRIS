/* ==========================================================================
 *
 *   Parser:
 * - Handles the parsing of a single .vm file
 * - Reads a VM command, parses the command into its lexical components, 
 *   and provides convenient access to these components
 * - Ignores whitespace and comments
 * 
 * ========================================================================== */


 pub fn init(inputFile: String) {
    /*
     * Opens the input file/stream and gets ready to parse it.
     */
 }


 fn hasMoreCommands() -> bool {
    /*
     * Are there more commands in the input?
     */
    return false
 }


 fn advance() {
    /*
     * Reads the next command from the input and makes it the
     * *current command*.
     * Only called if there's more commands.
     * Initially there is not current command.
     */
 }


 fn commandType() {
    /*
     * Returns a constant representing the type of the current command.
     * 
     * Types:
     * C_ARITHMETIC, C_PUSH, C_POP, C_LABEL, C_GOTO, C_IF, C_FUNCTION, C_RETURN, C_CALL
     */
 }


 fn arg1() -> String {
    /*
     * Returns the first argument of the current command.
     * In the case of C_ARITHMETIC, the command itself (add, sub, ...) is returned.
     * Shouldn't be called if the current command is C_RETURN.
     */
 }


 fn arg1() -> String {
    /*
     * Returns the second argument of the current command.
     * Only called if the current command is C_PUSH, C_POP, C_FUNCTION or C_CALL.
     */
 }