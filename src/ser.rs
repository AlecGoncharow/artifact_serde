use crate::de::{DeserializedCard, DeserializedDeck, DeserializedHero};

const CURRENT_VERSION: u8 = 2;
const ENCODED_PREFIX: &str = "ADC";
const HEADER_SIZE: u32 = 3;

/// Takes in a mutable reference to a [DeserializedDeck](struct.DeserializedDeck.html) returns
/// the corresponding Artifact Deck Code refer to deck_encoder.php for reference implementation
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode)
/// # Example  
/// ```
/// let mut my_deck = artifact_serde::de::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__").unwrap();
/// let my_adc = artifact_serde::ser::encode(&mut my_deck).unwrap();
/// ```
pub fn encode(deck: &mut DeserializedDeck) -> Result<String, String> {
    if deck.heroes.len() != 5 {
        return Err(String::from("Decks must have 5 heroes"));
    }
    if deck.cards.len() == 0 {
        return Err(String::from("Decks must have cards"));
    }

    let bytes = encode_bytes(deck).unwrap();

    encode_bytes_to_string(&bytes)
}

/// Takes in a mutable reference to a [Deck](struct.Deck.html) returns
/// the corresponding Artifact Deck Code refer to deck_encoder.php for reference implementation
pub fn encode_from_deck(deck: &crate::Deck) -> Result<String, String> {
    let mut de_deck = DeserializedDeck::new();
    let mut refs: Vec<u32> = Vec::new();

    for hero in &deck.heroes {
        de_deck.heroes.push(DeserializedHero {
            id: hero.card.card_id,
            turn: hero.turn,
        });
        for reference in &hero.card.references {
            refs.push(reference.card_id);
        }
    }

    for card in &deck.cards {
        match refs.contains(&card.card.card_id) {
            true => continue,
            false => {
                de_deck.cards.push(DeserializedCard {
                    id: card.card.card_id,
                    count: card.count,
                });
            }
        }
    }
    de_deck.name = deck.name.clone();

    encode(&mut de_deck)
}

fn encode_bytes_to_string(bytes: &Vec<u8>) -> Result<String, String> {
    let byte_count = bytes.len();
    if byte_count == 0 {
        return Err(String::from(
            "No bytes were encoded, did you pass a non-empty deck?",
        ));
    }

    let encoded = base64::encode(&bytes);

    let mut deck_string = format!("{}{}", ENCODED_PREFIX, encoded);

    // make string url safe
    deck_string = deck_string
        .chars()
        .map(|x| match x {
            '/' => '-',
            '=' => '_',
            _ => x,
        })
        .collect();

    Ok(deck_string)
}

fn encode_bytes(deck: &mut DeserializedDeck) -> Result<Vec<u8>, String> {
    deck.cards.sort();
    deck.heroes.sort();

    let count_heroes = deck.heroes.len();

    let mut bytes: Vec<u8> = Vec::new();
    //our version and hero count
    let version = CURRENT_VERSION << 4 | extract_n_bits_with_carry(count_heroes as u32, 3);
    bytes.push(version);

    //the checksum which will be updated at the end
    let dummy_checksum = 0;
    let checksum_byte = bytes.len();
    bytes.push(dummy_checksum);

    // write the name size
    let mut safe_name = String::new();
    let mut name_len = 0;
    if deck.name.len() > 0 {
        safe_name = ammonia::clean(&deck.name);
        let mut trim_len = safe_name.len();
        while trim_len > 63 {
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
    add_remaining_number_to_buffer(count_heroes as u32, 3, &mut bytes);

    let mut prev_card_id = 0;
    for hero in &deck.heroes {
        add_card_to_buffer(hero.turn, hero.id - prev_card_id, &mut bytes);
        prev_card_id = hero.id;
    }

    // reset our card offset
    prev_card_id = 0;

    for card in &deck.cards {
        add_card_to_buffer(card.count, card.id - prev_card_id, &mut bytes);
        prev_card_id = card.id;
    }

    // save off pre string bytes for checksum
    let pre_string_byte_count = bytes.len();

    // write the string
    let name_bytes = safe_name.as_bytes();
    for byte in name_bytes {
        bytes.push(*byte);
    }

    let full_checksum = compute_checksum(&bytes, pre_string_byte_count as u32 - HEADER_SIZE);
    let small_checksum = full_checksum & 0x0FF;

    // borrow checksum index and overwrite value with real checksum
    {
        let checksum_ref = &mut bytes[checksum_byte];
        *checksum_ref = small_checksum as u8;
    }

    Ok(bytes)
}

fn extract_n_bits_with_carry(value: u32, num_bits: u8) -> u8 {
    let limit_bit = 1 << num_bits;
    let mut result = value & (limit_bit - 1);
    if value >= limit_bit {
        result |= limit_bit;
    }
    if result > 255 {
        panic!("Something broke in extract_n_bits_with_carry: {}", result);
    }
    result as u8
}

// Utility to write the rest of a number into a buffer.
// This will first strip the specified N bits off, and then write a series
// of bytes of the structure of 1 overflow bit and 7 data bits
fn add_remaining_number_to_buffer(value: u32, already_written_bits: u8, bytes: &mut Vec<u8>) {
    let mut curr_value = value;
    curr_value >>= already_written_bits;
    let mut num_bytes = 0;
    while curr_value > 0 {
        let next_byte = extract_n_bits_with_carry(curr_value, 7);
        curr_value >>= 7;
        bytes.push(next_byte);
        num_bytes = num_bytes + 1;
    }
}

fn add_card_to_buffer(count: u32, value: u32, bytes: &mut Vec<u8>) {
    let count_start = bytes.len();

    // Determine our count. We can only store 2 bits, and we know the value
    // is at least one, so we can encode values 1-5. However,
    // we set both bits to indicate an extended count encodin
    let first_byte_max_count: u8 = 0x03;
    let extended_count = (count - 1) as u8 >= first_byte_max_count;

    // Determine our first byte, which contains our count, a continue flag,
    // and the first few bits of our value
    let first_byte_count: u8 = if extended_count {
        first_byte_max_count
    } else {
        count as u8 - 1
    };

    let mut first_byte: u8 = first_byte_count << 6;
    first_byte |= extract_n_bits_with_carry(value, 5);

    bytes.push(first_byte);

    // Now continue writing out the rest of the number with a carry flag
    add_remaining_number_to_buffer(value, 5, bytes);

    // Now if we overflowed on the count, encode the remaining count
    if extended_count {
        add_remaining_number_to_buffer(count, 0, bytes);
    }

    let count_end = bytes.len();

    if count_end - count_start > 11 {
        panic!("something went horribly wrong, per reference");
    }
}

fn compute_checksum(bytes: &Vec<u8>, num_bytes: u32) -> u32 {
    let mut checksum = 0;

    for add_check in HEADER_SIZE..num_bytes + HEADER_SIZE {
        checksum = checksum + *bytes.get(add_check as usize).unwrap() as u32;
    }

    checksum
}
