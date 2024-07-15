fn main() {
    let c: CaesarCipher = CaesarCipher::new(5);
    println!("{}", c.encode("mess"));
}

struct CaesarCipher {
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> CaesarCipher {
        CaesarCipher { shift }
    }

    fn encode(&self, message: &str) -> String {
        let messageu = message.to_uppercase();

        let vecky: String = messageu
            .chars()
            .map(|x| (x as u8 + self.shift as u8) as char)
            .collect();

        // println!("{}", vecky);
        // "string".to_string()
        vecky
    }

    fn decode(&self, message: &str) -> String {
        todo!();
    }
}
