//! # artifact_serde
//! This is a small crate to handle deseralizing and serializing Artifact Deck Codes.\
//! See this link for reference implementation: [link](https://github.com/ValveSoftware/ArtifactDeckCode)
//! \
//! Most structs in this crate will mimic the JSON structure provided by Valve, either by API or
//! base64 encoding.
#[macro_use]
extern crate serde_derive;
pub mod de;
pub mod ser;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSetJson {
    pub card_set: CardSet,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CardSet {
    pub version: u32,
    pub set_info: SetInfo,
    pub card_list: Vec<Card>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetInfo {
    pub set_id: u32,
    pub pack_item_def: u32,
    pub name: TranslatedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatedText {
    #[serde(default)]
    pub english: String,
    #[serde(default)]
    pub german: String,
    #[serde(default)]
    pub french: String,
    #[serde(default)]
    pub italian: String,
    #[serde(default)]
    pub koreana: String,
    #[serde(default)]
    pub spanish: String,
    #[serde(default)]
    pub schinese: String,
    #[serde(default)]
    pub tchinese: String,
    #[serde(default)]
    pub russian: String,
    #[serde(default)]
    pub thai: String,
    #[serde(default)]
    pub japanese: String,
    #[serde(default)]
    pub portuguese: String,
    #[serde(default)]
    pub polish: String,
    #[serde(default)]
    pub danish: String,
    #[serde(default)]
    pub dutch: String,
    #[serde(default)]
    pub finnish: String,
    #[serde(default)]
    pub norwegian: String,
    #[serde(default)]
    pub swedish: String,
    #[serde(default)]
    pub hungarian: String,
    #[serde(default)]
    pub czech: String,
    #[serde(default)]
    pub romanian: String,
    #[serde(default)]
    pub turkish: String,
    #[serde(default)]
    pub brazilian: String,
    #[serde(default)]
    pub bulgarian: String,
    #[serde(default)]
    pub greek: String,
    #[serde(default)]
    pub ukrainian: String,
    #[serde(default)]
    pub latam: String,
    #[serde(default)]
    pub vietnamese: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub card_id: u32,
    pub base_card_id: u32,
    pub card_type: String,
    #[serde(default)]
    pub sub_type: String,
    pub card_name: TranslatedText,
    pub card_text: TranslatedText,
    pub mini_image: Image,
    pub large_image: Image,
    pub ingame_image: Image,
    #[serde(default)]
    pub illustrator: String,
    #[serde(default)]
    pub is_red: bool,
    #[serde(default)]
    pub is_green: bool,
    #[serde(default)]
    pub is_blue: bool,
    #[serde(default)]
    pub is_black: bool,
    #[serde(default)]
    pub gold_cost: u32,
    #[serde(default)]
    pub mana_cost: u32,
    #[serde(default)]
    pub attack: u32,
    #[serde(default)]
    pub hit_points: u32,
    pub references: Vec<Reference>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HeroCard {
    pub card: Card,
    pub turn: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CardCard {
    pub card: Card,
    pub count: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub name: String,
    pub heroes: Vec<HeroCard>,
    pub cards: Vec<CardCard>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            heroes: Vec::new(),
            cards: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(default)]
    pub default: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    #[serde(default)]
    pub card_id: u32,
    #[serde(default)]
    pub ref_type: String,
    #[serde(default)]
    pub count: u32,
}

/// Takes in an Artifact Deck Code as a &str and returns a DeserializedDeck matching the structure
/// refer to deck_decoder.php for reference implementation and expected structure
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode)
/// # Example  
/// ```
/// artifact_serde::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__");
/// ```
pub fn decode(adc: &str) -> Result<de::DeserializedDeck, String> {
    de::decode(adc)
}

/// Takes in a mutable reference to a [DeserializedDeck](struct.DeserializedDeck.html) returns
/// the corresponding Artifact Deck Code refer to deck_encoder.php for reference implementation
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode)
/// # Example  
/// ```
/// let mut my_deck = artifact_serde::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__").unwrap();
/// let my_adc = artifact_serde::encode(&mut my_deck).unwrap();
/// ```
pub fn encode(deck: &mut de::DeserializedDeck) -> Result<String, String> {
    ser::encode(deck)
}

/// Takes in a mutable reference to a [Deck](struct.Deck.html) returns
/// the corresponding Artifact Deck Code refer to deck_encoder.php for reference implementation
pub fn encode_from_deck(deck: &crate::Deck) -> Result<String, String> {
    ser::encode_from_deck(deck)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn decode_to_encode() {
        let mut deck = crate::decode(
            "ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__",
        )
        .unwrap();

        let string = crate::ser::encode(&mut deck).unwrap();
        assert_eq!(
            string.as_str(),
            "ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__"
        );
    }

    #[test]
    fn decode_deck_from_json_file() {
        use std::fs::File;
        let json_file = File::open("tests/data/deck_one.json").expect("file not found");

        let deck: crate::Deck = serde_json::from_reader(json_file).unwrap();
        let string = crate::ser::encode_from_deck(&deck).unwrap();
        assert_eq!(
            string.as_str(),
            "ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__"
        );
    }

    #[test]
    fn empty_deck_encode_err() {
        let deck = crate::Deck::new();
        match crate::ser::encode_from_deck(&deck) {
            Ok(_) => panic!("This should not happen"),
            Err(_) => (),
        };

        let mut de_deck = crate::de::DeserializedDeck::new();
        match crate::ser::encode(&mut de_deck) {
            Ok(_) => panic!("This should not happen"),
            Err(_) => (),
        };
    }

    #[test]
    fn sanatize_bad_name() {
        use std::fs::File;
        let json_file = File::open("tests/data/deck_one.json").expect("file not found");

        let mut deck: crate::Deck = serde_json::from_reader(json_file).unwrap();
        deck.name = String::from("<script>things</script>");
        let string = crate::ser::encode_from_deck(&deck).unwrap();
        let de_deck = crate::de::decode_from_string(&string).unwrap();

        assert_eq!(de_deck.name.as_str(), "things");
    }
}
