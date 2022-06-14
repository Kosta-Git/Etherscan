pub enum Network {
    MainNet,
    Goerli,
    Kovan,
    RInkeby,
    Ropsten,
}

impl Network {
    pub fn to_url(&self) -> String {
        match self {
            Network::MainNet => "https://api.etherscan.io/api?".to_string(),
            Network::Goerli => "https://api-goerli.etherscan.io/api?".to_string(),
            Network::Kovan => "https://api-kovan.etherscan.io/api?".to_string(),
            Network::RInkeby => "https://api-rinkeby.etherscan.io/api?".to_string(),
            Network::Ropsten => "https://api-ropsten.etherscan.io/api?".to_string(),
        }
    }
}