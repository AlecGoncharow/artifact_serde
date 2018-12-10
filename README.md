# artifact_serde [![Build Status](https://travis-ci.com/AlecGoncharow/artifact_serde.svg?branch=master)](https://travis-ci.com/AlecGoncharow/artifact_serde) [![crates.io](https://img.shields.io/crates/v/artifact_serde.svg)](https://crates.io/crates/artifact_serde) [![docs](https://docs.rs/artifact_serde/badge.svg)](https://docs.rs/artifact_serde/)

A small Rust library for serializing [Artifact](https://playartifact.com) decks and deserialzing Artifact Deck Codes. 

# Usage
The API is still a bit unstable while I figure out the best way to handle certain aspects beyond just decoding and encoding Artifact Deck Codes. The API for basic decoding and encoding should remain relatively stable.  

To install, add to `Cargo.toml`
```toml
[dependencies]
artifact_serde = "0.2.1"
```

If you are still on rust version < 1.31.0 you will need to add this to your crate root
```rust
extern crate artifact_serde
```

Basic usage example  
```rust
fn main() {
  let adc = "ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__";
  let mut my_deck = artifact_serde::decode(&adc).unwrap();
  
  // Should return a new String that is the same as the initial ADC
  let my_serialized_adc = artifact_serde::encode(&mut my_deck).unwrap();
}
```
`my_deck` will be an instance of `DeserializedDeck`, by itself it is not too useful, as you will still need to map the 
card ids to their respective cards, provided by Valve which is described in detail [here](https://github.com/ValveSoftware/ArtifactDeckCode#card-set-api)

This part I am still debating on whether or not to bring into the library. It would be trivial to implement, but I am afraid
of making unnecessary API pings and/or caching on user's systems.

The library currently handles deserializing the JSON provided by Valve's card set API endpoints, you can leverage this with 
by grabbing the JSON somehow and using `serde_json` to deserialize it, example using a stored JSON file: 
```rust
use std::fs::File;
fn main() {
  let my_json_card_set = File::open("Path/To/File").expect("File Not Found");
  let my_card_set: artifact_serde::CardSetJson = serde_json::from_reader(my_json_card_set).unwrap();
}
```
All the structs follow the same structure found in Valve's JSON, documented [here](https://docs.rs/artifact_serde/*/artifact_serde/struct.CardSetJson.html)
