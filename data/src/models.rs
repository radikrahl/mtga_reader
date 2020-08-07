use crate::data::Data;
use crate::data::DataType;
use crate::models::data_models::data_cards::Ability;
use crate::models::data_models::data_cards::Card;
use crate::models::data_models::data_location::Key;
use crate::models::data_models::data_location::Location;

pub mod data_models;
#[derive(Default)]
pub struct CardData {
    pub title: Key,
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
    pub fn get_by_title_id(id: i64) -> CardData {
        let data: Vec<Card> = Data::new(DataType::Cards).get();
        let data = data
            .into_iter()
            .find(|key| key.title_id == id)
            .expect("failure");
        CardData {
            title: LocationData::get_by_id(data.title_id),
            card_type: data.types,
            cost: data.castingcost.to_string(),
            text: data.subtype_text_id.to_string(),
            abilities: data.abilities,
            set_key: data.set,
            power: data.power,
            toughness: data.toughness,
            rarity: data.rarity,
            artist_name: data.artist_credit
        }
    }

    pub fn get_by_name(text: &str) -> CardData {
        let text_key = LocationData::get_by_name(text);
        let data: CardData = CardData::get_by_title_id(text_key.id);

        data
    }
}

pub struct LocationData;
impl LocationData {
    pub fn get_by_id(id: i64) -> Key {
        let data: Vec<Location> = Data::new(DataType::Locations).get();

        let data = data
            .into_iter()
            .find(|x| x.iso_code == "en-US")
            .expect("could not find by iso_code");

        data.keys
            .into_iter()
            .find(|key| key.id == id)
            .expect("couldn not find by id")
    }

    pub fn get_by_name(name: &str) -> Key {
        let data: Vec<Location> = Data::new(DataType::Locations).get();

        let data = data
            .into_iter()
            .find(|x| x.iso_code == "en-US")
            .expect("could not find by iso_code");

        data.keys
            .into_iter()
            .find(|key| key.text == name)
            .expect("could not find by name")
    }
}

pub mod tests {
    use crate::models::*;
    #[test]
    fn should_return_title_id() {
        let data = LocationData::get_by_name("Crash of Rhinos");
        assert_eq!(2929, data.id)
    }

    #[test]
    fn should_return_title_string() {
        let data = CardData::get_by_title_id(2929);

        assert_eq!("Crash of Rhinos", data.title.text);
    }

    #[test]
    fn should_return_text_key() {
        let data = LocationData::get_by_id(2929);
        assert_eq!("Crash of Rhinos", data.text);
    }

    #[test]
    fn should_return_card_by_name() {
        let data = CardData::get_by_name("Opt");
        assert_eq!(
            (16650, 2, &"Craig J Spearing".to_string()),
            (data.title.id, data.rarity, &data.artist_name)
        );
    }
}
