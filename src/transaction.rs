use std::process;

use serde::{Serialize, Deserialize};
use crate::{utils, blockchain::Blockchain};

// mining reward
pub const SUBSIDY: i32 = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct TXInput {
    // the transaction ID correspond to the previous output
    pub txid: Vec<u8>,
    // A transaction can have multiple outputs. Vout tells us which one to look at.
    pub vout: i32,
    // a script which provides data to be used in an output's script_pub_key
    script_sig: String
}

#[derive(Serialize, Deserialize, Clone)]
// TXOuput's character is "indivisible"
pub struct TXOutput {
    pub value: i32,
    /**
     * It defines unloking condition, and only transaction inputs(TXInput) that satisfy
     * these conditions can utilize or "spend" this output. In short, it outlines how
     * Bitcoin is locked and can only be unlocked when the correct input(usually the 
     * corresponding private key) is provided.
     * Internally, Bitcoin uses a scripting language called "Script", that is used
     * to define outputs locking and unlocking logic.
     */
    pub script_pub_key: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TXOutput {
    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}

impl Transaction {
    fn set_id(&mut self) {
        let encode = match bincode::serialize(&self) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                Vec::new()
            }
        };
        let hash = utils::compute_sha256(&encode);
        self.id = hash;
    }

    // coinbase is the mining reward, so it only has inputs without outputs, and the 
    // input address originates from 0
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == -1
    }
}

// coinbase transaction: creates outputs out of nowhere
pub fn new_coinbase_tx(to: &str, data: &str) -> Transaction {
    let input_data;
    if data.is_empty() {
        input_data = format!("Reward to {}", to);
    } else {
        input_data = data.to_string();
    }

    let txin = TXInput {
        txid: vec![],
        vout: -1,
        script_sig: input_data
    };

    let txout = TXOutput {
        value: SUBSIDY,
        script_pub_key: to.to_string()
    };

    let mut tx = Transaction {
        id: vec![],
        vin: vec![txin],
        vout: vec![txout]
    };
    tx.set_id();
    tx
}

pub fn new_utxo_transaction(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Transaction {
    let mut txs_inputs = Vec::new();
    let mut txs_outputs = Vec::new();

    let (acc, valid_outputs) = bc.find_spendable_outputs(from, amount);

    if acc < amount {
        eprintln!("Error: Not enough funds");
        process::exit(-1);
    }

    for (txid, outs) in valid_outputs.iter() {
        
        // spending coins, writing into inputs indicates that the money has been spent
        for out in outs {
            let input = TXInput {
                // txid: txid.as_bytes().to_vec(),
                txid: utils::string_hex(txid),
                vout: *out,
                script_sig: from.to_string()
            };
            txs_inputs.push(input);
        }
    }

    // transfer utxo to the "to" address
    txs_outputs.push(TXOutput {
        value: amount,
        script_pub_key: to.to_string()
    });

    // change coins
    if acc > amount {
        txs_outputs.push(TXOutput { 
            value: acc - amount, 
            script_pub_key: from.to_string()
        });
    }

    let mut tx = Transaction {
        id: Vec::new(),
        vin: txs_inputs,
        vout: txs_outputs
    };
    tx.set_id();

    tx
}