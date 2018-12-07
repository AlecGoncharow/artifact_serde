//! # artifact_serde
//! This is a small crate to handle deseralizing and potentially serializing Artifact Deck Codes.\
//! See this link for reference implementation: [link](https://github.com/ValveSoftware/ArtifactDeckCode)
//!
#[macro_use]
extern crate serde_derive;
pub mod de;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSetJson {
    pub card_set: CardSet,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CardSet {
    pub version: usize,
    pub set_info: SetInfo,
    pub card_list: Vec<Card>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetInfo {
    pub set_id: usize,
    pub pack_item_def: usize,
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
    pub card_id: usize,
    pub base_card_id: usize,
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
    pub gold_cost: usize,
    #[serde(default)]
    pub mana_cost: usize,
    #[serde(default)]
    pub attack: usize,
    #[serde(default)]
    pub hit_points: usize,
    pub references: Vec<Reference>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HeroCard {
    pub card: Card,
    pub turn: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CardCard {
    pub card: Card,
    pub count: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub name: String,
    pub heroes: Vec<HeroCard>,
    pub cards: Vec<CardCard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(default)]
    pub default: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    #[serde(default)]
    pub card_id: usize,
    #[serde(default)]
    pub ref_type: String,
    #[serde(default)]
    pub count: usize,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
