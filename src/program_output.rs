use std::{fmt::Display, process::Output};

#[derive(Debug, PartialEq, Eq)]
pub struct ProgramOutput {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl ProgramOutput {
    pub fn new(code: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            code: code,
            stdout,
            stderr,
        }
    }
}

impl From<Output> for ProgramOutput {
    fn from(value: Output) -> Self {
        Self {
            code: value.status.code().unwrap(),
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}

impl Display for ProgramOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            0 => write!(f, "{}", String::from_utf8_lossy(&self.stdout)),
            _ => write!(
                f,
                "Program exited with code {}. Error: {}",
                self.code,
                String::from_utf8_lossy(&self.stderr)
            ),
        }
    }
}
