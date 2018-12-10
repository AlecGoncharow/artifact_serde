use super::de::DeserializedDeck;

const CURRENT_VERSION: u8 = 2;
const ENCODED_PREFIX: &str = "ADC";
const MAX_BYTES_FOR_VAR_U32: u8 = 5;
const HEADER_SIZE: u8 = 3;

pub fn encode(deck: &mut DeserializedDeck) -> Result<String, String> {
    Err(String::from("not implemented"))
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

    Err(String::from(""))
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
