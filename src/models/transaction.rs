use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    #[serde(rename = "timeStamp")]
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub nonce: Option<String>,
    #[serde(rename = "blockHash", default)]
    pub block_hash: Option<String>,
    #[serde(rename = "transactionIndex", default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub transaction_index: Option<u64>,
    pub from: String,
    pub to: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub value: u128,
    #[serde(rename = "type", default)]
    pub tx_type: Option<String>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub gas: u128,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "gasPrice", default)]
    pub gas_price: Option<u64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "isError", default)]
    pub is_error: Option<u8>,
    #[serde(rename = "txreceipt_status", default)]
    pub tx_receipt_status: Option<String>,
    pub input: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "cumulativeGasUsed", default)]
    pub cumulative_gas_used: Option<u64>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "tokenID", default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub token_id: Option<u32>,
    #[serde(rename = "tokenName", default)]
    pub token_name: Option<String>,
    #[serde(rename = "tokenSymbol", default)]
    pub token_symbol: Option<String>,
    #[serde(rename = "tokenDecimal", default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub token_decimal: Option<u8>,
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub confirmations: Option<u64>,
    #[serde(rename = "traceId", default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trace_id: Option<u8>,
    #[serde(rename = "errCode", default)]
    pub error_code: Option<String>,
    #[serde(rename = "v", default)]
    pub v: Option<String>,
    #[serde(rename = "r", default)]
    pub r: Option<String>,
    #[serde(rename = "s", default)]
    pub s: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::models::transaction::Transaction;

    #[test]
    fn serialize_transaction() {
        let data = r#"
        {
         "blockNumber":"0",
         "timeStamp":"1438269973",
         "hash":"GENESIS_ddbd2b932c763ba5b1b7ae3b362eac3e8d40121a",
         "nonce":"",
         "blockHash":"",
         "transactionIndex":"0",
         "from":"GENESIS",
         "to":"0xddbd2b932c763ba5b1b7ae3b362eac3e8d40121a",
         "value":"10000000000000000000000",
         "gas":"0",
         "gasPrice":"0",
         "isError":"0",
         "txreceipt_status":"",
         "input":"",
         "contractAddress":"",
         "cumulativeGasUsed":"0",
         "gasUsed":"0",
         "confirmations":"12698061"
        }
        "#;

        let tx: Transaction = serde_json::from_str(data).unwrap();

        assert_eq!(0, tx.block_number);
        assert_eq!("GENESIS_ddbd2b932c763ba5b1b7ae3b362eac3e8d40121a", &tx.hash);
        assert_eq!("", &tx.nonce.unwrap());
        assert_eq!("", &tx.block_hash.unwrap());
        assert_eq!(None, tx.error_code);
    }

    #[test]
    fn serialize_internal_transaction() {
        let data = r#"
        {
         "blockNumber":"2535479",
         "timeStamp":"1477839134",
         "hash":"0x1a50f1dc0bc912745f7d09b988669f71d199719e2fb7592c2074ede9578032d0",
         "from":"0x2c1ba59d6f58433fb1eaee7d20b26ed83bda51a3",
         "to":"0x20d42f2e99a421147acf198d775395cac2e8b03d",
         "value":"100000000000000000",
         "contractAddress":"",
         "input":"",
         "type":"call",
         "gas":"235231",
         "gasUsed":"0",
         "traceId":"0",
         "isError":"0",
         "errCode":""
        }
        "#;

        let tx: Transaction = serde_json::from_str(data).unwrap();

        assert_eq!("0x1a50f1dc0bc912745f7d09b988669f71d199719e2fb7592c2074ede9578032d0", &tx.hash);
        assert_eq!("", &tx.error_code.unwrap());
        assert_eq!(None, tx.cumulative_gas_used);
    }

    #[test]
    fn serialize_erc20_transaction() {
        let data = r#"
        {
         "blockNumber":"4730207",
         "timeStamp":"1513240363",
         "hash":"0xe8c208398bd5ae8e4c237658580db56a2a94dfa0ca382c99b776fa6e7d31d5b4",
         "nonce":"406",
         "blockHash":"0x022c5e6a3d2487a8ccf8946a2ffb74938bf8e5c8a3f6d91b41c56378a96b5c37",
         "from":"0x642ae78fafbb8032da552d619ad43f1d81e4dd7c",
         "contractAddress":"0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2",
         "to":"0x4e83362442b8d1bec281594cea3050c8eb01311c",
         "value":"5901522149285533025181",
         "tokenName":"Maker",
         "tokenSymbol":"MKR",
         "tokenDecimal":"18",
         "transactionIndex":"81",
         "gas":"940000",
         "gasPrice":"32010000000",
         "gasUsed":"77759",
         "cumulativeGasUsed":"2523379",
         "input":"deprecated",
         "confirmations":"7968350"
        }
        "#;

        let tx: Transaction = serde_json::from_str(data).unwrap();

        assert_eq!("0xe8c208398bd5ae8e4c237658580db56a2a94dfa0ca382c99b776fa6e7d31d5b4", &tx.hash);
        assert_eq!("Maker", &tx.token_name.unwrap());
        assert_eq!("", &tx.error_code.unwrap());
        assert_eq!(None, tx.cumulative_gas_used);
    }

    #[test]
    fn serialize_erc721_transaction() {
        let data = r#"
        {
         "blockNumber":"4708120",
         "timeStamp":"1512907118",
         "hash":"0x031e6968a8de362e4328d60dcc7f72f0d6fc84284c452f63176632177146de66",
         "nonce":"0",
         "blockHash":"0x4be19c278bfaead5cb0bc9476fa632e2447f6e6259e0303af210302d22779a24",
         "from":"0xb1690c08e213a35ed9bab7b318de14420fb57d8c",
         "contractAddress":"0x06012c8cf97bead5deae237070f9587f8e7a266d",
         "to":"0x6975be450864c02b4613023c2152ee0743572325",
         "tokenID":"202106",
         "tokenName":"CryptoKitties",
         "tokenSymbol":"CK",
         "tokenDecimal":"0",
         "transactionIndex":"81",
         "gas":"158820",
         "gasPrice":"40000000000",
         "gasUsed":"60508",
         "cumulativeGasUsed":"4880352",
         "input":"deprecated",
         "confirmations":"7990490"
      }
        "#;

        let tx: Transaction = serde_json::from_str(data).unwrap();

        assert_eq!("0x031e6968a8de362e4328d60dcc7f72f0d6fc84284c452f63176632177146de66", &tx.hash);
        assert_eq!("CryptoKitties", &tx.token_name.unwrap());
        assert_eq!(202106, tx.token_id.unwrap());
        assert_eq!("", &tx.error_code.unwrap());
        assert_eq!(None, tx.cumulative_gas_used);
    }
}