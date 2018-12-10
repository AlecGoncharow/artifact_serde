use super::de::DeserializedDeck;
use ammonia::clean;

const CURRENT_VERSION: u8 = 2;
const ENCODED_PREFIX: &str = "ADC";
const MAX_BYTES_FOR_VAR_U32: u8 = 5;
const HEADER_SIZE: u8 = 3;

pub fn encode(deck: &mut DeserializedDeck) -> Result<String, String> {
    Err(String::from("not implemented"))
}

struct Card {
    id: u32,
    field: u32,
}

fn encode_bytes(deck: &mut DeserializedDeck) -> Result<Vec<u8>, String> {
    deck.cards.sort();
    deck.heroes.sort();

    let count_heroes = deck.heroes.len();
    let mut cards: Vec<Card> = Vec::new();
    for hero in &deck.heroes {
        cards.push(Card {
            id: hero.id,
            field: hero.turn,
        });
    }
    for card in &deck.cards {
        cards.push(Card {
            id: card.id,
            field: card.count,
        });
    }

    let mut bytes: Vec<u8> = Vec::new();
    let version = CURRENT_VERSION << 4 | extract_n_bits_with_carry(count_heroes as u8, 3);
    bytes.push(version);

    let mut safe_name = String::new();
    let mut name_len = 0;
    if deck.name.len() > 0 {
        safe_name = clean(&deck.name);
        let mut trim_len = safe_name.len();
        while trim_len > 63usize {
            let mut amount_to_trim = (trim_len - 63) / 4;
            amount_to_trim = if amount_to_trim > 1 {
                amount_to_trim
            } else {
                1
            };

            let split_len = safe_name.len() - amount_to_trim;
            safe_name.split_off(split_len);
            trim_len = safe_name.len();
        }
        name_len = safe_name.len();
    }

    bytes.push(name_len as u8);

    Err(String::from(""))
}

fn extract_n_bits_with_carry(value: u8, num_bits: u8) -> u8 {
    let unlimit_bit = 1 << num_bits;
    let mut unresult = value & (unlimit_bit - 1);
    if value >= unlimit_bit {
        unresult |= unlimit_bit;
    }
    unresult
}
