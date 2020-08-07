extern crate data;
use std::io;

fn main() {
    // loop {
        let mut text: String = String::new();

        println!("Search for a card...");
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read user Input");
        
        let x = data::models::CardData::get_by_name(&text.trim());
        print_card(x);
    // }
}

fn print_card(card: data::models::CardData) {
    println!("title: {}", card.title.text);

    for (i, elem) in card.card_type.iter().enumerate() {
        println!("card_type <{}> has id: {}", i + 1, elem);
    }

    println!("cost: {}", card.cost);
    println!("tecardt: {}", card.text);
    for (i, elem) in card.abilities.iter().enumerate() {
        println!("ability {}: {}", i + 1, elem.text_id);
    }
    println!("set: {}", card.set_key);
    println!("power: {}", card.power);
    println!("toughness: {}", card.toughness);
    println!("rarity: {}", card.rarity);
}










// pub mod process_dump {
//     use crate::data::DataType;
//     use crate::dump::data_dump::data_cards::Card;
//     use crate::dump::data_dump::data_location::Key;
//     use crate::dump::data_dump::data_location::Location;

//     use crate::data::Data;
//     pub fn get_card_by_name(name: &str) -> Option<Card> {
//         let cards: Vec<Card> = Data::new(DataType::Cards);
//         let text_key = get_text(name).unwrap_or_default();
//         let cards = cards.into_iter().find(|key| key.title_id == text_key.id);

//         match cards {
//             Some(val) => Some(val),
//             None => {
//                 println!("no card found handle_me");
//                 None
//             }
//         }
//     }

//     fn get_text(text: &str) -> Option<Key> {
//         let loc: Vec<Location> = Data::new(DataType::Cards).get().expect("error");
//         let keys = loc
//             .into_iter()
//             .find(|x| x.iso_code == "en-US")
//             .expect("could not resolve key_code");

//         let keys = keys
//             .keys
//             .into_iter()
//             .find(|key| key.text.to_lowercase() == text.to_lowercase());

//         match keys {
//             Some(val) => Some(val),
//             None => {
//                 println!("no text key found");
//                 None
//             }
//         }
//     }

//     pub fn get_text_by_id(id: i64) -> Option<Key> {
//         let loc: Vec<Location> = Data::new(DataType::Cards).get().expect("error");
//         let keys = loc
//             .into_iter()
//             .find(|x| x.iso_code == "en-US")
//             .expect("could not resolve key_code");

//         let keys = keys.keys.into_iter().find(|key| key.id == id);

//         match keys {
//             Some(val) => Some(val),
//             None => {
//                 println!("no text key found");
//                 None
//             }
//         }
//     }
// }
