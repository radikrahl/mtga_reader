extern crate data;
use crate::csv::read_csv;
use std::io;

fn main() {
    read_csv("output.csv").expect("kaputt")
}

mod csv {
    use csv::StringRecord;
    use data::models::output_data::CardData;
    use std::error::Error;
    use std::fs;
    use std::io;

    pub fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string(&file_path).expect("Error reading file handle_me");
        let mut reader = csv::Reader::from_reader(file.as_bytes());
        let records = reader.records();

        let mapping = |key: Result<StringRecord, csv::Error>| -> CardData {
            let x = key.expect("kaputt");
            let s = x.get(0).expect("kaputt").to_string();
            let data = data::models::output_data::CardData::get_by_name(&s).unwrap_or_default();

            CardData { title: s, ..data }
        };

        let iter: Vec<CardData> = records.map(mapping).collect();
        for elem in iter {
            println!("{} {}", elem.title, elem.artist_name);
        }
        Ok(())
    }

    pub fn write_csv() -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.flush()?;
        Ok(())
    }
}

pub fn run() {
    fn run_get_all() -> Result<String, serde_json::error::Error> {
        let x = data::models::output_data::CardData::get_all().ok();
        serde_json::to_string(&x)
    }
    fn run_print_to_json() -> Result<String, serde_json::error::Error> {
        let x = data::models::output_data::CardData::get_all();
        serde_json::to_string(&x)
    }
    fn run_search_single() {
        let mut text: String = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read user Input");

        let x =
            data::models::output_data::CardData::get_by_name(&text.trim()).expect("failed again");
        let x = serde_json::to_string(&x).expect("failed");
        println!("{}", x);
    }
    fn run_search() {
        loop {
            let mut text: String = String::new();
            println!("Search for a card...");
            io::stdin()
                .read_line(&mut text)
                .expect("Failed to read user Input");
            let x = data::models::output_data::CardData::get_by_name(&text.trim());
            match x {
                Ok(val) => print_card(val),
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            }
        }
    }

    fn print_card(card: data::models::output_data::CardData) {
        println!("title: {}", card.title);

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
}
