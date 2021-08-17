use anyhow::{anyhow, Error};
use std::alloc::Global;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Closed,
    Open,
}

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, Error>;
}

impl File {
    /// Creates a new `File`.
    ///
    /// # Examples
    ///
    /// ```
    /// let file_data: &[u8] = &[114, 117, 115, 116, 33];
    /// let my_file = MyFile::new("f1.txt", file_data);
    /// ```
    pub fn new(name: &str, data: &[u8]) -> Self {
        Self {
            name: String::from(name),
            data: data.clone().into(),
            state: FileState::Closed,
        }
    }

    pub fn open(&mut self) -> Result<bool, Error> {
        self.state = FileState::Open;
        Ok(true)
    }

    pub fn close(&mut self) -> Result<bool, Error> {
        self.state = FileState::Closed;
        Ok(true)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Read for File {
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, Error> {
        if self.state != FileState::Open {
            return Err(anyhow!("File was not open while trying to read from it"));
        }

        let mut data = self.data.clone();
        let read_length = data.len();

        buffer.reserve(read_length);
        buffer.append(&mut data);

        Ok(read_length)
    }
}
