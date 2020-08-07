pub mod data_cards {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Card {
        pub grpid: i64,
        pub title_id: i64,
        pub art_id: i64,
        pub is_token: bool,
        pub is_primary_card: bool,
        pub art_size: i64,
        pub power: String,
        pub toughness: String,
        pub flavor_id: i64,
        #[serde(rename = "CollectorNumber")]
        pub collector_number: String,
        pub alt_deck_limit: Option<i64>,
        pub cmc: i64,
        pub rarity: i64,
        pub artist_credit: String,
        pub set: String,
        pub linked_face_type: i64,
        pub types: Vec<i64>,
        pub subtypes: Vec<i64>,
        pub supertypes: Vec<i64>,
        pub card_type_text_id: i64,
        pub subtype_text_id: i64,
        pub colors: Vec<i64>,
        pub frame_colors: Vec<i64>,
        pub frame_details: Vec<String>,
        pub color_identity: Vec<i64>,
        pub abilities: Vec<Ability>,
        pub hidden_abilities: Vec<HiddenAbility>,
        pub linked_faces: Vec<i64>,
        pub castingcost: String,
        pub known_supported_styles: Vec<String>,
        #[serde(rename = "DigitalReleaseSet")]
        pub digital_release_set: String,
        pub ability_id_to_linked_token_grp_id: Vec<AbilityIdToLinkedTokenGrpId>,
    }

    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Ability {
        pub ability_id: i64,
        pub text_id: i64,
    }

    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct HiddenAbility {
        pub ability_id: i64,
        pub text_id: i64,
    }

    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AbilityIdToLinkedTokenGrpId {
        pub ability_id: i64,
        pub linked_token_grp_id: i64,
    }
}
pub mod data_abilities {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Ability {
        pub id: i64,
        pub text: i64,
        pub base_id: i64,
        pub base_id_numeral: i64,
        pub category: i64,
        pub sub_category: i64,
        pub ability_word: i64,
        pub requires_confirmation: i64,
        pub miscellaneous_term: i64,
        pub numeric_aid: i64,
        pub mana_cost: String,
        pub fully_parsed: bool,
        pub payment_types: Vec<i64>,
        pub relevant_zones: Vec<i64>,
        pub linked_hidden_abilities: Vec<i64>,
        pub referenced_keywords: Vec<i64>,
        pub referenced_keyword_types: Vec<i64>,
        pub modal_ability_children: Vec<i64>,
    }
}
pub mod data_location {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Location {
        pub langkey: String,
        pub iso_code: String,
        pub keys: Vec<Key>,
    }

    #[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Key {
        pub id: i64,
        pub text: String,
    }


}
