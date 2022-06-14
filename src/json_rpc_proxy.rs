use crate::Etherscan;
use crate::models::EtherResponse;
use crate::constants::MODULE_PROXY;

impl Etherscan {
    pub async fn latest_block_id(&self) -> EtherResponse<String> {
        self.execute_action(
            MODULE_PROXY,
            "eth_blockNumber",
            &vec![],
        ).await.unwrap()
    }
}