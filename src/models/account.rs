use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalance {
    pub account: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub balance: u128,
}