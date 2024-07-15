fn main() {
    println!("Hello, world!");
}

struct CaesarCipher {
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> CaesarCipher {
        CaesarCipher { shift }
    }

    fn encode(&self, message: &str) -> String {
        todo!();
    }

    fn decode(&self, message: &str) -> String {
        todo!();
    }
}
