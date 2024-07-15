fn main() {
    let c: CaesarCipher = CaesarCipher::new(5);
    c.encode("mess");
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
        let mut vecky: Vec<char> = Vec::new();
        vecky = message.chars().collect();
        println!("{:#?}", vecky);
        "string".to_string()
    }

    fn decode(&self, message: &str) -> String {
        todo!();
    }
}
