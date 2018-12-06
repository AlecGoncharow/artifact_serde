//! # artifact_serde
//! This is a small crate to handle deseralizing and potentially serializing Artifact Deck Codes.\
//! See this link for reference implementation: [link](https://github.com/ValveSoftware/ArtifactDeckCode)
//!

extern crate base64;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use regex::Regex;
use std::collections::HashMap;

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
/// # Example:  
/// ```
/// artifact_serde::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__");
/// ```

pub fn decode(adc: &str) -> DeserializedDeck {
    let re = Regex::new(r"^ADC").unwrap();
    let mut stripped_adc = re.replace_all(adc, "");
    stripped_adc = stripped_adc
        .chars()
        .map(|x| match x {
            '-' => '/',
            '_' => '=',
            _ => x,
        }).collect();

    let adc_string = String::from(stripped_adc);
    let decoded = base64::decode(&adc_string).unwrap();
    parse_deck(adc_string, decoded)
}

/// Takes in a vector of JSON formatted &str and attempts to coerce them into CardSetJson,
/// the JSON should take the form mentioned
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode) or:
/// ```ignore
///{
///  "card_set": {
///    "version": 1,
///  "set_info": {
///   "set_id": 0,
///    "pack_item_def": 0,
///     "name": {
///        "english": "Base Set"
///      }
///    },
///   "card_list": [{
///
///   "card_id": 4000,
///   "base_card_id": 4000,
///    "card_type": "Hero",
///   "card_name": {
///     "english": "Farvhan the Dreamer"
///  },
///   "card_text": {
///      "english": "Pack Leadership<BR>\nFarvhan the Dreamer's allied neighbors have +1 Armor."
///    },
///     "mini_image": {
///       "default": "<url to png>"
///     },
///    "large_image": {
///       "default": "<url to png>"
///      },
///     "ingame_image": {
///       "default": "<url to png>"
///    },
///    "is_green": true,
///    "attack": 4,
///    "hit_points": 10,
///      "references": [{
///      "card_id": 4002,
///        "ref_type": "includes",
///          "count": 3
///  },
///        {
///        "card_id": 4001,
///      "ref_type": "passive_ability"
///        }
///    ]
///
///
///    },
///    ..... more cards ....
///
///    ]
///  }
///}
///```
///
pub fn json_to_deck_hashmap(sets: Vec<&str>) -> HashMap<usize, Card> {
    let mut d_sets = Vec::new();
    for set in sets {
        let s: CardSetJson = match serde_json::from_str(set) {
            Ok(s) => s,
            Err(e) => panic!("Not Valid JSON: {}", e),
        };

        let d = s.card_set;
        d_sets.push(d);
    }

    set_up_deck_map(d_sets)
}
fn set_up_deck_map(sets: Vec<CardSet>) -> HashMap<usize, Card> {
    let mut map = HashMap::<usize, Card>::new();
    for set in sets {
        for card in set.card_list {
            map.insert(card.card_id, card);
        }
    }
    map
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedHero {
    pub id: usize,
    pub turn: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedCard {
    pub id: usize,
    pub count: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedDeck {
    pub heroes: Vec<DeserializedHero>,
    pub cards: Vec<DeserializedCard>,
    pub name: String,
}

fn parse_deck(_deck_code: String, deck_bytes: Vec<u8>) -> DeserializedDeck {
    let total_bytes = deck_bytes.len();
    let mut current_byte_index = 0 as usize;
    let version_and_heroes = deck_bytes.get(0).unwrap();
    current_byte_index += 1;

    let version = deck_bytes.get(0).unwrap() >> 4;

    let _checksum = deck_bytes.get(1).unwrap();
    current_byte_index += 1;

    let total_card_bytes = if version > 1 as u8 {
        current_byte_index += 1;
        total_bytes - *deck_bytes.get(2).unwrap() as usize
    } else {
        total_bytes
    };

    let mut num_heroes = 0;
    read_encoded_u32(
        *version_and_heroes as usize,
        3,
        &deck_bytes,
        &mut current_byte_index,
        total_card_bytes as usize,
        &mut num_heroes,
    );

    let mut heroes = Vec::<DeserializedHero>::new();
    let mut prev_card_base = 0;
    for curr_hero in 0..num_heroes {
        let mut hero_turn = 0 as usize;
        let mut hero_card_id = 0 as usize;
        if !read_serialized_card(
            &deck_bytes,
            &mut current_byte_index,
            total_card_bytes as usize,
            &mut prev_card_base,
            &mut hero_turn,
            &mut hero_card_id,
        ) {
            println!(
                "error reading read_serialized_card, curr_hero: {}",
                curr_hero
            );
            break;
        }
        heroes.push(DeserializedHero {
            id: hero_card_id,
            turn: hero_turn,
        });
    }

    let mut cards = Vec::<DeserializedCard>::new();
    prev_card_base = 0;
    while current_byte_index < total_card_bytes as usize {
        let mut card_count = 0;
        let mut card_id = 0;
        if !read_serialized_card(
            &deck_bytes,
            &mut current_byte_index,
            total_card_bytes as usize,
            &mut prev_card_base,
            &mut card_count,
            &mut card_id,
        ) {
            println!(
                "out of card_bytes, current_byte_index: {}",
                current_byte_index
            );
            break;
        }
        cards.push(DeserializedCard {
            id: card_id,
            count: card_count,
        });
    }

    let name = if current_byte_index <= total_card_bytes {
        let bytes = &deck_bytes[total_card_bytes..];
        let out: String = bytes.iter().map(|x| *x as char).collect();
        out
    } else {
        String::from("")
    };

    DeserializedDeck {
        heroes,
        cards,
        name,
    }
}

fn read_bits_chunk(
    n_chunk: usize,
    n_bits: usize,
    n_curr_shift: usize,
    n_out_bits: &mut usize,
) -> bool {
    let continue_bit = 1 << n_bits;
    let new_bits = n_chunk & (continue_bit - 1);
    *n_out_bits |= (new_bits << n_curr_shift) as usize;

    n_chunk & continue_bit != 0
}

fn read_encoded_u32(
    base_value: usize,
    base_bits: usize,
    deck_bytes: &Vec<u8>,
    start_index: &mut usize,
    end_index: usize,
    out_value: &mut usize,
) {
    *out_value = 0;
    let mut delta_shift = 0;

    if base_bits == 0 || read_bits_chunk(base_value, base_bits, delta_shift, out_value) {
        delta_shift += base_bits;
        loop {
            if *start_index > end_index {
                break;
            }

            let next_byte = deck_bytes.get(*start_index).unwrap();
            *start_index += 1;
            if !read_bits_chunk(*next_byte as usize, 7, delta_shift, out_value) {
                break;
            }

            delta_shift += 7;
        }
    }
}

fn read_serialized_card(
    deck_bytes: &Vec<u8>,
    start_index: &mut usize,
    end_index: usize,
    prev_card_base: &mut usize,
    out_count: &mut usize,
    out_id: &mut usize,
) -> bool {
    //end of the memory block?
    if *start_index > end_index {
        return false;
    }

    //header contains the count (2 bits), a continue flag, and 5 bits of offset data.
    //If we have 11 for the count bits we have the count
    //encoded after the offset
    let header = deck_bytes.get(*start_index).unwrap();
    *start_index += 1;

    let has_extended_count = (header >> 6) == 0x03 as u8;

    //read in the delta, which has 5 bits in the header, then additional bytes while the value is set
    let mut card_delta = 0;
    read_encoded_u32(
        *header as usize,
        5,
        &deck_bytes,
        start_index,
        end_index,
        &mut card_delta,
    );

    *out_id = *prev_card_base + card_delta;

    //now parse the count if we have an extended count
    match has_extended_count {
        true => {
            read_encoded_u32(0, 0, &deck_bytes, start_index, end_index, &mut (*out_count));
        }
        false => {
            //the count is just the upper two bits + 1 (since we don't encode zero)
            *out_count = (header >> 6) as usize + 1;
        }
    }

    *prev_card_base = *out_id;
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
