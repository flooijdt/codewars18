fn main() {
    let c: CaesarCipher = CaesarCipher::new(5);
    println!("{}", c.encode("Codewars"));
    println!("{}", c.decode("HTIJBFWX"));
    println!("{}", c.decode("IT'S A SHIFT CIPHER!!"));
    println!("{}", 'Z' as u8);
    println!("{}", 'A' as u8);
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

        let mut vecky: Vec<u8> = messageu.chars().map(|x| x as u8).collect();

        for i in vecky.iter_mut() {
            if *i < 91 && *i > 64 {
                *i += self.shift as u8;
                if *i > 90 {
                    *i = 64 + *i - 90;
                }
            }
        }

        vecky.iter().map(|x| *x as char).collect()
    }

    fn decode(&self, message: &str) -> String {
        let messageu = message.to_uppercase();

        let mut vecky: Vec<u8> = messageu.chars().map(|x| x as u8).collect();

        for i in vecky.iter_mut() {
            if *i < 91 && *i > 64 {
                *i -= self.shift as u8;
                if *i < 65 {
                    *i = *i + 90 - 64;
                }
            }
        }

        vecky.iter().map(|x| *x as char).collect()
    }
}
