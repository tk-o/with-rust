#[derive(Debug, Default)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str, data: &[u8]) -> Self {
        Self {
            name: String::from(name),
            data: data.clone().into(),
        }
    }

    pub fn open(&mut self) -> bool {
        true
    }

    pub fn close(&mut self) -> bool {
        true
    }

    pub fn read(&mut self, buffer: &mut Vec<u8>) -> usize {
        let mut data = self.data.clone();
        let read_length = data.len();

        buffer.reserve(read_length);
        buffer.append(&mut data);

        read_length
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
