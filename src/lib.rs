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

#[derive(Debug)]
pub enum Error {
    Decode(&'static str),
    Encode(&'static str),
}

/// Takes in an Artifact Deck Code as a &str and returns a DeserializedDeck matching the structure
/// refer to deck_decoder.php for reference implementation and expected structure
/// [here](https://github.com/ValveSoftware/ArtifactDeckCode)
/// # Example  
/// ```
/// artifact_serde::decode("ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__");
/// ```
pub fn decode(adc: &str) -> Result<de::DeserializedDeck, Error> {
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
pub fn encode(deck: &mut de::DeserializedDeck) -> Result<String, Error> {
    ser::encode(deck)
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
}
