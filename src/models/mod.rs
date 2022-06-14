pub mod transaction;
pub mod account;
pub mod networks;
mod ether;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EtherResponse<T> {
    pub status: String,
    pub message: String,
    pub result: T
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Tag {
    Earliest,
    Pending,
    Latest,
}

impl Tag {
    pub fn to_string(&self) -> String {
        match self {
            Tag::Earliest => "earliest".to_string(),
            Tag::Pending => "pending".to_string(),
            Tag::Latest => "latest".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sort {
    ASC,
    DESC
}

impl Sort {
    pub fn to_string(&self) -> String {
        match self {
            Sort::ASC => "asc".to_string(),
            Sort::DESC => "desc".to_string()
        }
    }
}