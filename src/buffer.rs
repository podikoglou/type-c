#[derive(Default, Clone)]
pub struct CodeBuffer {
    lines: Vec<String>,
}

impl CodeBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write<S: Into<String>>(&mut self, content: S) {
        let content = content.into();

        if let Some(last) = self.lines.last_mut() {
            last.push_str(&content);
        } else {
            self.write_line(content);
        }
    }

    pub fn write_line<S: Into<String>>(&mut self, content: S) {
        let content = content.into();

        self.lines.push(content);
    }

    pub fn concat(&mut self, other: &CodeBuffer) {
        self.lines.extend(other.lines.clone());
    }
}

pub fn concat(writers: &[CodeBuffer]) -> CodeBuffer {
    writers.iter().fold(CodeBuffer::new(), |mut acc, w| {
        acc.concat(w);
        acc
    })
}

impl From<&str> for CodeBuffer {
    fn from(value: &str) -> Self {
        let mut writer = Self::new();
        writer.write_line(value);
        writer
    }
}

impl From<String> for CodeBuffer {
    fn from(value: String) -> Self {
        let mut writer = Self::new();
        writer.write_line(&value);
        writer
    }
}

impl From<CodeBuffer> for String {
    fn from(value: CodeBuffer) -> Self {
        value.lines.join("\n")
    }
}
