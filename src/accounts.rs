use crate::constants::MODULE_ACCOUNT;
use crate::etherscan::Etherscan;
use crate::models::{EtherResponse, Sort, Tag};
use crate::models::account::AccountBalance;
use crate::models::transaction::Transaction;

impl Etherscan {
    // ================================================================
    // Account Balance
    // ================================================================

    pub async fn balance(
        &self,
        address: String,
        tag: Tag,
    ) -> EtherResponse<String> {
        self.execute_action(
            MODULE_ACCOUNT,
            "balance",
            &vec![
                ("address", &address),
                ("tag", &tag.to_string()),
            ],
        ).await.unwrap()
    }

    pub async fn balances(
        &self,
        address: Vec<String>,
        tag: Tag,
    ) -> EtherResponse<Vec<AccountBalance>> {
        self.execute_action(
            MODULE_ACCOUNT,
            "balancemulti",
            &vec![
                ("address", &address.join(",")),
                ("tag", &tag.to_string()),
            ],
        ).await.unwrap()
    }

    // ================================================================
    // Transactions
    // ================================================================

    pub async fn transactions_by_address(
        &self,
        address: String,
        start_block: u32,
        end_block: u32,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.execute_action(
            MODULE_ACCOUNT,
            "txlist",
            &vec![
                ("address", &address),
                ("startblock", &start_block.to_string()),
                ("endblock", &end_block.to_string()),
                ("page", &page.to_string()),
                ("offset", &offset.to_string()),
                ("sort", &sort.to_string()),
            ],
        ).await.unwrap()
    }

    pub async fn internal_transactions_by_address(
        &self,
        address: String,
        start_block: u32,
        end_block: u32,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.execute_action(
            MODULE_ACCOUNT,
            "txlistinternal",
            &vec![
                ("address", &address),
                ("startblock", &start_block.to_string()),
                ("endblock", &end_block.to_string()),
                ("page", &page.to_string()),
                ("offset", &offset.to_string()),
                ("sort", &sort.to_string()),
            ],
        ).await.unwrap()
    }

    pub async fn internal_transactions_by_hash(
        &self,
        tx_hash: String,
        start_block: u32,
        end_block: u32,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.execute_action(
            MODULE_ACCOUNT,
            "txlistinternal",
            &vec![
                ("txhash", &tx_hash),
                ("startblock", &start_block.to_string()),
                ("endblock", &end_block.to_string()),
                ("page", &page.to_string()),
                ("offset", &offset.to_string()),
                ("sort", &sort.to_string()),
            ],
        ).await.unwrap()
    }

    pub async fn internal_transactions_by_block_range(
        &self,
        start_block: u32,
        end_block: u32,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.execute_action(
            MODULE_ACCOUNT,
            "txlistinternal",
            &vec![
                ("startblock", &start_block.to_string()),
                ("endblock", &end_block.to_string()),
                ("page", &page.to_string()),
                ("offset", &offset.to_string()),
                ("sort", &sort.to_string()),
            ],
        ).await.unwrap()
    }

    // ================================================================
    // Token Transfer events
    // ================================================================

    async fn tte_by_address(
        &self,
        action: &str,
        contract_address: String,
        address: String,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.execute_action(
            MODULE_ACCOUNT,
            action,
            &vec![
                ("contractaddress", &contract_address),
                ("address", &address),
                ("page", &page.to_string()),
                ("offset", &offset.to_string()),
                ("sort", &sort.to_string()),
            ],
        ).await.unwrap()
    }

    pub async fn erc20_tte_by_address(
        &self,
        contract_address: String,
        address: String,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.tte_by_address(
            "tokentx",
            contract_address,
            address,
            page,
            offset,
            sort
        ).await
    }

    pub async fn erc721_tte_by_address(
        &self,
        contract_address: String,
        address: String,
        page: u32,
        offset: u32,
        sort: Sort,
    ) -> EtherResponse<Vec<Transaction>> {
        self.tte_by_address(
            "tokennfttx",
            contract_address,
            address,
            page,
            offset,
            sort
        ).await
    }
}