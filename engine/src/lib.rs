#[derive(Debug, Clone, PartialEq)]
pub enum Trouble {}

pub type Result<T> = std::result::Result<T, Trouble>;

pub fn run() -> Result<String> {
    Ok("wrooooom!".to_owned())
}

#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn it_works() {
        assert_eq!(run(), Ok("wrooooom!".to_owned()));
    }
}
