pub mod models;

pub mod data {
    use serde::de::DeserializeOwned;
    use std::fs;
    #[derive(Debug)]
    pub struct Data<T>
    where
        T: DeserializeOwned,
    {
        data: Vec<T>,
        file_path: String,
    }

    impl<T> Data<T>
    where
        T: DeserializeOwned,
    {
        pub fn new(data_type: DataType) -> Data<T> {
            let file_path = match data_type {
                DataType::Cards => "data/files/data_cards_somehash.mtga",
                DataType::Abilities => "data/files/data_abilities_somehash.mtga",
                DataType::Locations => "data/files/data_loc_somehash.mtga",
            }
            .to_string();
            let f = fs::read_to_string(&file_path).expect("Error reading file handle_me");
            let data: Vec<T> = serde_json::from_str(&f).expect("handle_me");

            Data { data, file_path }
        }

        pub fn get(self) -> Vec<T> {
            self.data
        }
    }

    pub enum DataType {
        Cards,
        Abilities,
        Locations,
    }
}

pub mod tests {
    use crate::models::data_models::data_abilities::Ability;
    use crate::models::data_models::data_location::Location;
    use crate::models::data_models::data_cards::Card;
    use crate::data::Data;
    use crate::data::DataType;
    #[test]
    fn should_return_card() {
        let data: Data<Card> = Data::new(DataType::Cards);
        let data = data.get();

        assert!(data.len() > 0);
    }
    #[test]
    fn should_return_location() {
        let data: Data<Location> = Data::new(DataType::Locations);
        let data = data.get();

        assert!(data.len() > 0);
    }

    #[test]
    fn should_return_abilities() {
        let data: Data<Ability> = Data::new(DataType::Abilities);
        let data = data.get();

        assert!(data.len() > 0);
    }
}
