use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
static PRIV: &str = r#"MIIBOgIBAAJAd0q+AKji6utYtrnKB8LoIKzafLFw07YdCa4Lg156932SDbPIUdJw
N3tsE8GNxTPcgeH4K3Pl4F8HfTdPknCd5wIDAQABAkBnRa13pL3N4jDK/8yHK5UJ
tJrxmCu1HNPGrYYKFKEf372XEau+dX+9FHLt3TPwZdfCl4Dbyfvdg8uKp3wFw2cB
AiEA4HPJMYGteDMDufBAi6FEHV/57VYo02YngQddEZIFOMMCIQCIDxcxPEkS7ybE
LT/hl3mZsqij8Rx49Yab6LMVOXSUDQIhAKn/aEHjyuQAF3FsAyb+VJ2/BId6u18p
vv/d+OKG6weLAiAkkEmfRaAtom88kRx1t4tKLqT7SbRpHTJLe8GElqgpkQIhAIKo
poKkqucHrgYw6A+pRgB8Eve/w3ID0xhYeXRh/5Uj"#;

static PUB: &str = "MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgE60D6/HCaKKvQmjeazL79F2k4V3
wIPopWhNfA3YJuJtrw7fGSRDFSF4uyGin1pt2sd54Vui6p8PPJ6e2lcnOji05Wf3
TVSt43V8MrNqJb1veCG0rFbw0uQ0uOLLf2kPCYOXjepCtQXTvJ2+gDMx6eIvwA6T
ba9kxGyeNT3EeG41AgMBAAE=";
fn get_priv() -> RSAPrivateKey {
    // &String::from_utf8(base64::decode(PRIV).unwrap()).unwrap()

    let der_encoded =
        PRIV.lines()
            .filter(|line| !line.starts_with("-"))
            .fold(String::new(), |mut data, line| {
                data.push_str(&line);
                data
            });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    RSAPrivateKey::from_pkcs1(&der_bytes).unwrap()
}
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
struct Message {
    previous: Option<String>, // Hash is a String for now
    content: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
struct Chain {
    messages: Vec<Message>,
}
impl Chain {
    fn save(&self) {
        std::fs::write("chain.json", serde_json::to_string(self).unwrap()).unwrap()
    }
    fn load() -> Self {
        let content = std::fs::read("chain.json").unwrap();
        serde_json::from_slice(&content).unwrap()
    }
    fn new_message(&mut self, content: Vec<u8>) {
        let message = Message {
            content: content,
            previous: Some(Box::new(self.messages.last().unwrap().to_owned())),
        };
        self.messages.push(message);
        self.save();
    }
}
fn main() {
    let mut chain = Chain::load();

    let mut rng = rand::thread_rng();
    let private_key = get_priv();
    let public_key = RSAPublicKey::from(private_key.clone());

    let data = b"hello world";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key.encrypt(&mut rng, padding, data).unwrap();
    chain.new_message(encrypted_data);
    // println!("{:0X?}", encrypted_data);
    // let padding = PaddingScheme::new_pkcs1v15_encrypt();

    // let decrypted_data = private_key.decrypt(padding, &encrypted_data).unwrap();
    // println!("{:0X?}", decrypted_data);
}
