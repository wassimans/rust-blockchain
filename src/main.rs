use chrono::Utc;
use log::{error, warn};
use serde::{Deserialize, Serialize};

pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let res: String = String::default();

    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
            timestamp: Utc::now().timestamp(),
            data: String::from("genesis!"),
            nonce: 2836,
        };
        self.blocks.push(genesis_block)
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self
            .blocks
            .last()
            .expect("There should be at least one valid block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block)
        } else {
            error!("could not add block - invalid")
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.hash != previous_block.hash {
            warn!("block with ID {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::decode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id {} has invalid hash", block.id);
            return false;
        }
        true
    }

    fn is_chain_valide(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(first, second) {
                return false;
            }
        }
        true
    }

    fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valide(&local);
        let is_remote_valid = self.is_chain_valide(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() > remote.len() {
                local
            } else {
                remote
            }
        } else if is_local_valid && !is_remote_valid {
            local
        } else if !is_local_valid && is_remote_valid {
            remote
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}

fn main() {}
