type Error = Box<dyn std::error::Error>;

const PHRASE: &str = "Hello, World!";

pub fn handle(_body: Vec<u8>) -> Result<Vec<u8>, Error> {
    Ok(PHRASE.as_bytes().to_vec())
}
