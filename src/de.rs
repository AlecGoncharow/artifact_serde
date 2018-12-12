use std::cmp::Ordering;

/// Takes in an Artifact Deck Code as a &str and returns a DeserializedDeck matching the structure
/// refer to deck_decoder.php for reference implementation and expected structure
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode)
/// # Example  
/// ```
/// artifact_serde::de::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__");
/// ```

pub fn decode(adc: &str) -> Result<DeserializedDeck, String> {
    let re = regex::Regex::new(r"^ADC").unwrap();
    let mut stripped_adc = re.replace_all(adc, "");
    stripped_adc = stripped_adc
        .chars()
        .map(|x| match x {
            '-' => '/',
            '_' => '=',
            _ => x,
        })
        .collect();

    let adc_string = String::from(stripped_adc);
    let decoded = base64::decode(&adc_string).unwrap();
    parse_deck(&decoded)
}

#[derive(Serialize, Deserialize, Debug, Eq, Default)]
pub struct DeserializedHero {
    pub id: u32,
    pub turn: u32,
}

impl Ord for DeserializedHero {
    fn cmp(&self, other: &DeserializedHero) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DeserializedHero {
    fn partial_cmp(&self, other: &DeserializedHero) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DeserializedHero {
    fn eq(&self, other: &DeserializedHero) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Default)]
pub struct DeserializedCard {
    pub id: u32,
    pub count: u32,
}

impl Ord for DeserializedCard {
    fn cmp(&self, other: &DeserializedCard) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DeserializedCard {
    fn partial_cmp(&self, other: &DeserializedCard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DeserializedCard {
    fn eq(&self, other: &DeserializedCard) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeserializedDeck {
    pub heroes: Vec<DeserializedHero>,
    pub cards: Vec<DeserializedCard>,
    pub name: String,
}

impl DeserializedDeck {
    pub fn new() -> Self {
        Self {
            heroes: Vec::new(),
            cards: Vec::new(),
            name: String::from(""),
        }
    }
}

fn parse_deck(deck_bytes: &[u8]) -> Result<DeserializedDeck, String> {
    let total_bytes = deck_bytes.len();
    let mut current_byte_index = 0 as usize;
    let version_and_heroes = deck_bytes[0];
    current_byte_index += 1;

    let version = version_and_heroes >> 4;

    let _checksum = deck_bytes[1];
    current_byte_index += 1;

    let total_card_bytes = if version > 1 as u8 {
        current_byte_index += 1;
        total_bytes - deck_bytes[2] as usize
    } else {
        total_bytes
    };

    let mut num_heroes = 0;
    read_encoded_u32(
        u32::from(version_and_heroes),
        3,
        deck_bytes,
        &mut current_byte_index,
        total_card_bytes,
        &mut num_heroes,
    );

    let mut heroes = Vec::<DeserializedHero>::new();
    let mut prev_card_base = 0;
    for _curr_hero in 0..num_heroes {
        let mut hero_turn = 0 as u32;
        let mut hero_card_id = 0 as u32;
        if !read_serialized_card(
            deck_bytes,
            &mut current_byte_index,
            total_card_bytes,
            &mut prev_card_base,
            &mut hero_turn,
            &mut hero_card_id,
        ) {
            return Err(
                "error during read_serialized_card, this is a bug if your ADC is confirmed valid, file bug report".to_string()
            );
        }
        heroes.push(DeserializedHero {
            id: hero_card_id,
            turn: hero_turn,
        });
    }

    let mut cards = Vec::<DeserializedCard>::new();
    prev_card_base = 0;
    while current_byte_index < total_card_bytes {
        let mut card_count = 0;
        let mut card_id = 0;
        if !read_serialized_card(
            deck_bytes,
            &mut current_byte_index,
            total_card_bytes,
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

    Ok(DeserializedDeck {
        heroes,
        cards,
        name,
    })
}

fn read_bits_chunk(n_chunk: u32, n_bits: u32, n_curr_shift: u32, n_out_bits: &mut u32) -> bool {
    let continue_bit = 1 << n_bits;
    let new_bits = n_chunk & (continue_bit - 1);
    *n_out_bits |= (new_bits << n_curr_shift) as u32;

    n_chunk & continue_bit != 0
}

fn read_encoded_u32(
    base_value: u32,
    base_bits: u32,
    deck_bytes: &[u8],
    start_index: &mut usize,
    end_index: usize,
    out_value: &mut u32,
) {
    *out_value = 0;
    let mut delta_shift = 0;

    if base_bits == 0 || read_bits_chunk(base_value, base_bits, delta_shift, out_value) {
        delta_shift += base_bits;
        loop {
            if *start_index > end_index {
                break;
            }

            let next_byte = deck_bytes[*start_index];
            *start_index += 1;
            if !read_bits_chunk(u32::from(next_byte), 7, delta_shift, out_value) {
                break;
            }

            delta_shift += 7;
        }
    }
}

fn read_serialized_card(
    deck_bytes: &[u8],
    start_index: &mut usize,
    end_index: usize,
    prev_card_base: &mut u32,
    out_count: &mut u32,
    out_id: &mut u32,
) -> bool {
    //end of the memory block?
    if *start_index > end_index {
        return false;
    }

    //header contains the count (2 bits), a continue flag, and 5 bits of offset data.
    //If we have 11 for the count bits we have the count
    //encoded after the offset
    let header = deck_bytes[*start_index];
    *start_index += 1;

    let has_extended_count = (header >> 6) == 0x03 as u8;

    //read in the delta, which has 5 bits in the header, then additional bytes while the value is set
    let mut card_delta = 0;
    read_encoded_u32(
        u32::from(header),
        5,
        &deck_bytes,
        start_index,
        end_index,
        &mut card_delta,
    );

    *out_id = *prev_card_base + card_delta;

    //now parse the count if we have an extended count
    if has_extended_count {
        read_encoded_u32(0, 0, &deck_bytes, start_index, end_index, &mut (*out_count));
    } else {
        //the count is just the upper two bits + 1 (since we don't encode zero)
        *out_count = u32::from(header >> 6) + 1;
    }

    *prev_card_base = *out_id;
    true
}
