#![cfg(test)]

use hemoglobin::cards::Card;

const CARDS_JSON_PATH: &str = "../files/cards.json";

#[test]
fn cards_json() -> Result<(), std::io::Error> {
    let string = std::fs::read_to_string(CARDS_JSON_PATH)?;

    let _: Vec<Card> = serde_json::from_str(&string)?;

    Ok(())
}
