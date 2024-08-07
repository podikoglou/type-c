#[derive(Default, Clone)]
pub struct CodeWriter {
    lines: Vec<String>,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, content: &str) {
        if let Some(last) = self.lines.last_mut() {
            last.push_str(content);
        } else {
            self.lines.push(content.to_string());
        }
    }

    pub fn write_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn concat(&mut self, other: &CodeWriter) {
        self.lines.extend(other.lines.clone());
    }
}

pub fn concat(writers: &[CodeWriter]) -> CodeWriter {
    writers.iter().fold(CodeWriter::new(), |mut acc, w| {
        acc.concat(w);
        acc
    })
}

impl From<&str> for CodeWriter {
    fn from(value: &str) -> Self {
        let mut writer = Self::new();
        writer.write_line(value);
        writer
    }
}

impl From<String> for CodeWriter {
    fn from(value: String) -> Self {
        let mut writer = Self::new();
        writer.write_line(&value);
        writer
    }
}

impl From<CodeWriter> for String {
    fn from(value: CodeWriter) -> Self {
        value.lines.join("\n")
    }
}
