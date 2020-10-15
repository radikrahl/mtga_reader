use crate::data::Data;
use crate::data::DataType;
use crate::models::data_models::data_cards::Ability;
use crate::models::data_models::data_cards::Card;
use crate::models::data_models::data_location::Key;
use crate::models::data_models::data_location::Location;
use serde::Serialize;

#[derive(Default, Serialize, Clone, Debug)]
pub struct CardData {
    pub title: String,
    pub title_id: i64,
    pub card_type: Vec<i64>,
    pub cost: String,
    pub text: String,
    pub abilities: Vec<Ability>,
    pub set_key: String,
    // card_info: String,
    pub power: String,
    pub toughness: String,
    pub rarity: i64,
    pub artist_name: String,
}

impl CardData {
    pub fn get_by_title_id(id: i64) -> Result<CardData, String> {
        let data: Vec<Card> = Data::new(DataType::Cards).get();
        let data = data.into_iter().find(|key| key.title_id == id);
        let loca_data: Vec<Key> = LocationData::get_all()?;

        match data {
            Some(data) => Ok(CardData {
                title: loca_data
                    .into_iter()
                    .find(|key| key.id == data.title_id)
                    .unwrap()
                    .text,
                title_id: data.title_id,
                card_type: data.types,
                cost: data.castingcost.to_string(),
                text: data.subtype_text_id.to_string(),
                abilities: data.abilities,
                set_key: data.set,
                power: data.power,
                toughness: data.toughness,
                rarity: data.rarity,
                artist_name: data.artist_credit,
            }),
            None => Err("No card found".to_string()),
        }
    }

    pub fn get_by_name(text: &str) -> Result<CardData, String> {
        let text_key = LocationData::get_by_name(text);
        match text_key {
            Ok(val) => CardData::get_by_title_id(val.id),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn get_all() -> Result<Vec<CardData>, String> {
        let data: Vec<Card> = Data::new(DataType::Cards).get();
        let mut vec = Vec::new();
        data.into_iter().for_each(|key| {
            vec.push(CardData {
                title: String::from("empty"),
                title_id: key.title_id,
                card_type: key.types,
                cost: key.castingcost.to_string(),
                text: key.subtype_text_id.to_string(),
                abilities: key.abilities,
                set_key: key.set,
                power: key.power,
                toughness: key.toughness,
                rarity: key.rarity,
                artist_name: key.artist_credit,
            });
        });

        let loca_data: Vec<Key> = LocationData::get_all()?;

        for elem in vec.iter_mut() {
            let t = loca_data.iter().find(|key| key.id == elem.title_id);

            match t {
                Some(t) => elem.title = t.text.to_string(),
                None => continue,
            }
        }

        // resolve abilities

        match vec.len() > 0 {
            true => Ok(vec),
            false => Err("no data found".to_string()),
        }
    }
}

pub struct LocationData;
impl LocationData {
    pub fn get_by_id(id: i64) -> Result<Key, String> {
        let data: Vec<Location> = Data::new(DataType::Locations).get();

        let data = data
            .into_iter()
            .find(|x| x.iso_code == "en-US")
            .expect("could not find by iso_code");

        let data = data.keys.into_iter().find(|key| key.id == id);
        match data {
            Some(data) => Ok(data),
            None => Err("no key for name found".to_string()),
        }
    }

    pub fn get_by_name(name: &str) -> Result<Key, String> {
        let data: Vec<Location> = Data::new(DataType::Locations).get();

        let data = data
            .into_iter()
            .find(|x| x.iso_code == "en-US")
            .expect("could not find by iso_code");

        let data = data.keys.into_iter().find(|key| key.text == name);

        match data {
            Some(data) => Ok(data),
            None => Err("no key for name found".to_string()),
        }
    }

    pub fn get_all() -> Result<Vec<Key>, String> {
        let data: Vec<Location> = Data::new(DataType::Locations).get();

        let data = data.into_iter().find(|x| x.iso_code == "en-US");

        match data {
            Some(data) => Ok(data.keys),
            None => Err("no key for name found".to_string()),
        }
    }
}

pub mod tests {
    use crate::models::output_data::*;
    #[test]
    fn should_return_title_id() {
        let data = LocationData::get_by_name("Crash of Rhinos");
        let data = data.unwrap();
        assert_eq!(2929, data.id)
    }
    #[test]
    #[ignore]
    fn should_return_title_string() {
        let data = CardData::get_by_title_id(2929);
        let data = data.unwrap();
        assert_eq!("Crash of Rhinos", data.title);
    }

    #[test]
    fn should_return_text_key() {
        let data = LocationData::get_by_id(2929);
        let data = data.unwrap();
        assert_eq!("Crash of Rhinos", data.text);
    }

    #[test]
    fn should_return_card_by_name() {
        let data = CardData::get_by_name("Opt");
        let data = data.unwrap();
        assert_eq!(
            (16650, 2, &"Craig J Spearing".to_string()),
            (data.title_id, data.rarity, &data.artist_name)
        );
    }

    #[test]
    fn should_resolve_card() {
        let data = CardData::get_all().unwrap();
        assert_eq!(2929, data[0].title_id);
    }

    #[test]
    fn should_resolve_keys() {
        let data = LocationData::get_all().unwrap();
        assert_eq!(
            (2, "Basic".to_string()),
            (data[1].id, data[1].text.to_string())
        )
    }

    #[test]
    fn should_return_no_key_found() {
        let data = LocationData::get_by_name("nonsense");

        assert_eq!(Err("no key for name found".to_string()), data)
    }
}
