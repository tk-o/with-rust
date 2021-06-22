const VERSION: &'static str = "0.0.1";

pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(version(), VERSION);
    }
}
