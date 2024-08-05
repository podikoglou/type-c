#[derive(Default)]
pub struct CodeWriter {
    lines: Vec<String>,
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter::default()
    }

    pub fn write(&mut self, line: String) {
        if line.contains('\n') {
            line.split('\n').for_each(|l| self.write(l.to_string()));
        } else {
            self.lines.push(line);
        }
    }

    pub fn concat(&mut self, other: &CodeWriter) {
        self.write(other.code());
    }

    pub fn code(&self) -> String {
        self.lines.join("\n")
    }
}

pub fn concat(writers: &[CodeWriter]) -> CodeWriter {
    let mut result = CodeWriter::default();
    writers.iter().for_each(|w| result.concat(w));
    result
}
