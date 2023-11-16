## How to Start the code

```
$ rustc pretty_printer.rs
$ ./pretty_printer
```

## Explantion of the preetifer function Rust
The `prettify` function is a recursive function that takes in an expression and returns its equivalent string representation. The base case for this recursion is when the expression is a `Literal` or a `Variable`. In this case, we simply return the string representation of the expression. 


# Pretty Printer
The `PrettyPrinter` struct is used to print code lines with proper indentation. It provides methods to handle the printing and indentation logic.

## Example Usage
```rust
use std::{fs, io};
struct PrettyPrinter { indent_level: usize }
impl PrettyPrinter {
    fn new() -> Self { PrettyPrinter { indent_level: 0 } }
    fn print(&mut self, code: &str) {
        for line in code.lines() { self.print_line(line); }
    }
    fn print_line(&mut self, line: &str) {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() { return; }
        let indent = "    ".repeat(self.indent_level);
        println!("{}{}", indent, trimmed_line);
    }
    fn increase_indent(&mut self) { self.indent_level += 1; }
    fn decrease_indent(&mut self) { if self.indent_level > 0 { self.indent_level -= 1; } }
}
fn main() -> io::Result<()> {
    let code = fs::read_to_string("example.rs")?;
    let mut printer = PrettyPrinter::new();
    for line in code.lines() {
        if line.contains("{") { printer.print_line(line); printer.increase_indent(); }
        else if line.contains("}") { printer.decrease_indent(); printer.print_line(line); }
        else { printer.print_line(line); }
    }
    Ok(())
}
```
## Inputs
- The code snippet requires a file named "example.rs" to be present in the same directory.
- The file should contain Rust code.
## Flow
1. The code snippet defines a struct called `PrettyPrinter` with an `indent_level` field.
2. The `PrettyPrinter` struct has methods to print code lines with proper indentation.
3. The `print` method takes a string of code and iterates over each line, calling the `print_line` method for each line.
4. The `print_line` method trims the line, checks if it is empty, and then adds the appropriate indentation before printing the line.
5. The `increase_indent` and `decrease_indent` methods are used to adjust the indentation level based on the presence of opening and closing curly braces.
6. In the `main` function, the code reads the contents of the "example.rs" file and creates an instance of `PrettyPrinter`.
7. It then iterates over each line of the code, checking if it contains an opening or closing curly brace.
8. If it contains an opening brace, the indentation level is increased before printing the line.
9. If it contains a closing brace, the indentation level is decreased before printing the line.
10. Otherwise, the line is printed with the current indentation level.
11. Finally, the program returns `Ok(())` to indicate successful execution.
## Outputs
- The code snippet prints the contents of the "example.rs" file with proper indentation.
Note: The code comments, line comments, blank lines, formatting, etc. have been preserved from the original code snippet.
```