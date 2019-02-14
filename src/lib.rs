
const VERSION: &'static str = "0.0.0";

#[derive(Debug)]
pub struct Arguments {
    pub flag: String,
    pub keyword: String,
    pub file_name: String,
    pub threads: u32,
}

pub struct Output (String, u64);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
