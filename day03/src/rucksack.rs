#[derive(Debug)]
pub struct RuckSack {
    pub compartment1: String,
    pub compartment2: String,
    pub full_sack: String,
}

pub fn build_sack(sack_string: String) -> RuckSack {
    return RuckSack {
        compartment1: sack_string.split_at(sack_string.len() / 2).0.to_string(),
        compartment2: sack_string.split_at(sack_string.len() / 2).1.to_string(),
        full_sack: sack_string,
    };
}

pub trait BuildSack {

}

impl BuildSack for RuckSack {
}

