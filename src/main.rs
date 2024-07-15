fn main() {
    let c: CaesarCipher = CaesarCipher::new(5);
    c.encode("mess");
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

        let messageu = message.to_uppercase();

        vecky = messageu.chars().collect();

        println!("{:x}", vecky[0] as u32);
        println!("{}", String::from("\u{004D}"));
        "string".to_string()
    }

    fn decode(&self, message: &str) -> String {
        todo!();
    }
}
