pub struct Erc20(String);

impl Erc20 {
    pub fn new() -> Self {
        Self("Hello from ERC-20".into())
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
        let token = Erc20::new();
        assert_eq!(token.name(), "Hello from ERC-20");
    }
}
