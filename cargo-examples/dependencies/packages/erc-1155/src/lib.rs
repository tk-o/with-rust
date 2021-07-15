pub struct Erc1155(String);

impl Erc1155 {
    pub fn new() -> Self {
        Self("Hello from ERC-1155".into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_token() {
        let token = Erc1155::new();
        assert_eq!(token.name(), "Hello from ERC-1155");
    }
}
