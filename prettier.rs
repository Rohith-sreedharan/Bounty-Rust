use std::{fs, io, io::Write, io::BufReader};
use regex::Regex;
use indenter::indented;

struct PrettyPrinter {
    indent_level: usize,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter { indent_level: 0 }
    }

    pub fn print(&mut self, code: &str, writer: &mut dyn Write) -> Result<(), io::Error> {
        let indented_code = indented(code).with_indent("    ");
        writer.write_all(indented_code.as_bytes())?;
        Ok(())
    }

    pub fn print_line(&mut self, line: &str, writer: &mut dyn Write) -> Result<(), io::Error> {
        let trimmed_line = line.trim_start();
        let indent = "    ".repeat(self.indent_level);
        writer.write_all(format!("{}{}", indent, trimmed_line).as_bytes())?;
        Ok(())
    }

    pub fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
}

fn main() -> io::Result<()> {
    let file = fs::File::open("example.rs")?;
    let reader = io::BufReader::new(file);
    let mut printer = PrettyPrinter::new();
    let mut writer = io::BufWriter::new(io::stdout());
    let regex = Regex::new(r#"[{}]"#)?;

    for line in reader.lines() {
        let line = line?;
        let matches = regex.captures_iter(&line);
        let num_braces = matches.count();
        if num_braces > 0 {
            printer.indent_level += num_braces;
        }
        printer.print_line(&line, &mut writer)?;
        if num_braces > 0 {
            printer.indent_level -= num_braces;
        }
    }

    writer.flush()?;
    Ok(())
}