use crate::common::BlockPtr;
use crate::common::Source;
use crate::critical;
use crate::error;
use crate::info;
use crate::warn;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum ProgressCheckResult {
    OkToProceed,
    BlockAlreadyProcessed,
    UnexpectedBlock,
    MaybeReorg,
    ForkBlock,
    UnrecognizedBlock,
}

#[derive(Clone)]
pub struct ProgressCtrl {
    recent_block_ptrs: VecDeque<BlockPtr>,
    sources: Vec<Source>,
    reorg_threshold: u16,
}

impl ProgressCtrl {
    pub fn new(
        recent_block_ptrs: Vec<BlockPtr>,
        sources: Vec<Source>,
        reorg_threshold: u16,
    ) -> Self {
        Self {
            recent_block_ptrs: VecDeque::from(recent_block_ptrs),
            sources,
            reorg_threshold,
        }
    }

    pub fn get_expected_block_number(&self) -> u64 {
        let min_start_block = self.sources.iter().filter_map(|s| s.startBlock).min();
        min_start_block.unwrap_or(0).max(
            self.recent_block_ptrs
                .front()
                .cloned()
                .map(|b| b.number + 1)
                .unwrap_or_default(),
        )
    }

    pub fn check_block(&mut self, new_block_ptr: BlockPtr) -> ProgressCheckResult {
        match self.recent_block_ptrs.front() {
            None => {
                let min_start_block = self.get_expected_block_number();

                if new_block_ptr.number == min_start_block {
                    self.recent_block_ptrs.push_front(new_block_ptr);
                    return ProgressCheckResult::OkToProceed;
                }

                error!(
                    ProgressCtrl,
                    "received an unexpected block whose number does not match subgraph's required start-block";
                    expected_block_number => min_start_block,
                    received_block_number => new_block_ptr.number
                );
                return ProgressCheckResult::UnexpectedBlock;
            }
            Some(last_processed) => {
                if last_processed.is_parent(&new_block_ptr) {
                    self.recent_block_ptrs.push_front(new_block_ptr);
                    if self.recent_block_ptrs.len() > self.reorg_threshold as usize {
                        self.recent_block_ptrs.pop_back();
                    }
                    return ProgressCheckResult::OkToProceed;
                }

                if new_block_ptr.number > last_processed.number + 1 {
                    critical!(
                        ProgressCtrl,
                        "received an invalid block whose number is larger than expected";
                        expected_block_number => last_processed.number + 1,
                        received_block_number => new_block_ptr.number
                    );
                    return ProgressCheckResult::UnexpectedBlock;
                }

                if new_block_ptr.number < self.recent_block_ptrs.back().unwrap().number {
                    critical!(
                        ProgressCtrl,
                        r#"
Block not recognized!
Please check your setup - as it can be either:
1) a reorg is too deep for runtime to handle, or
2) you have set a reorg-threshold which is too shallow, or
3) you are using a WRONG block source, or
4) Data-store & subgraph's block-pointers do not match!
"#;
                        received_block => new_block_ptr,
                        recent_blocks_processed => format!(
                            "{} ... {}",
                            self.recent_block_ptrs.back().unwrap(),
                            last_processed
                        )
                    );
                    return ProgressCheckResult::UnrecognizedBlock;
                }

                for block in self.recent_block_ptrs.iter() {
                    if *block == new_block_ptr {
                        if new_block_ptr.number % 10 == 0 {
                            warn!(
                                ProgressCtrl,
                                "Received a block that was already processed before";
                                block => new_block_ptr
                            );
                        }
                        return ProgressCheckResult::BlockAlreadyProcessed;
                    }

                    if block.is_parent(&new_block_ptr) {
                        info!(
                            ProgressCtrl,
                            "Reorg happened and a proper fork-block received";
                            fork_block => new_block_ptr,
                            parent_block => block
                        );
                        self.recent_block_ptrs
                            .retain(|b| b.number < new_block_ptr.number);
                        self.recent_block_ptrs.push_front(new_block_ptr);
                        return ProgressCheckResult::ForkBlock;
                    }
                }

                return ProgressCheckResult::MaybeReorg;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_progress(#[values(None, Some(0), Some(1), Some(2))] start_block: Option<u64>) {
        env_logger::try_init().unwrap_or_default();
        let sources = vec![
            Source {
                address: None,
                abi: "".to_owned(),
                startBlock: start_block,
            },
            Source {
                address: None,
                abi: "".to_owned(),
                startBlock: start_block.map(|n| n + 1),
            },
        ];
        let mut pc = ProgressCtrl::new(vec![], sources, 10);
        assert!(pc.recent_block_ptrs.is_empty());

        let actual_start_block = pc.get_expected_block_number();

        match start_block {
            None => assert_eq!(actual_start_block, 0),
            Some(block_number) => assert_eq!(actual_start_block, block_number),
        }

        for n in 0..20 {
            if n == pc.get_expected_block_number() {
                let result = pc.check_block(BlockPtr {
                    number: n,
                    hash: format!("n={n}"),
                    parent_hash: if n > 0 {
                        format!("n={}", n - 1)
                    } else {
                        "".to_string()
                    },
                });
                assert_eq!(result, ProgressCheckResult::OkToProceed);
            }
        }
        assert_eq!(pc.recent_block_ptrs.len(), 10);
        assert_eq!(pc.recent_block_ptrs.front().unwrap().number, 19);
        assert_eq!(pc.recent_block_ptrs.back().unwrap().number, 10);

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 22,
                hash: "".to_string(),
                parent_hash: "".to_string()
            }),
            ProgressCheckResult::UnexpectedBlock
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 21,
                hash: "".to_string(),
                parent_hash: "".to_string()
            }),
            ProgressCheckResult::UnexpectedBlock
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 20,
                hash: "".to_string(),
                parent_hash: "".to_string()
            }),
            ProgressCheckResult::MaybeReorg,
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 19,
                hash: "".to_string(),
                parent_hash: "".to_string()
            }),
            ProgressCheckResult::MaybeReorg,
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 15,
                hash: "n=15".to_string(),
                parent_hash: "n=some-fork-block".to_string()
            }),
            ProgressCheckResult::MaybeReorg,
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 9,
                hash: "".to_string(),
                parent_hash: "".to_string(),
            }),
            ProgressCheckResult::UnrecognizedBlock,
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 10,
                hash: "n=10".to_string(),
                parent_hash: "n=9".to_string(),
            }),
            ProgressCheckResult::BlockAlreadyProcessed
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 19,
                hash: "n=19".to_string(),
                parent_hash: "n=18".to_string(),
            }),
            ProgressCheckResult::BlockAlreadyProcessed
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 15,
                hash: "n=15".to_string(),
                parent_hash: "n=14".to_string(),
            }),
            ProgressCheckResult::BlockAlreadyProcessed
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 20,
                hash: "n=20".to_string(),
                parent_hash: "n=19".to_string(),
            }),
            ProgressCheckResult::OkToProceed
        );

        assert_eq!(
            pc.recent_block_ptrs.front().cloned().unwrap(),
            BlockPtr {
                number: 20,
                hash: "n=20".to_string(),
                parent_hash: "n=19".to_string(),
            }
        );

        assert_eq!(
            pc.recent_block_ptrs.back().cloned().unwrap(),
            BlockPtr {
                number: 11,
                hash: "n=11".to_string(),
                parent_hash: "n=10".to_string(),
            }
        );

        assert_eq!(
            pc.check_block(BlockPtr {
                number: 19,
                hash: "n=fork19".to_string(),
                parent_hash: "n=18".to_string(),
            }),
            ProgressCheckResult::ForkBlock
        );

        assert_eq!(pc.recent_block_ptrs.len(), 9);
        assert_eq!(
            pc.recent_block_ptrs.front().cloned().unwrap(),
            BlockPtr {
                number: 19,
                hash: "n=fork19".to_string(),
                parent_hash: "n=18".to_string(),
            }
        );
        assert_eq!(pc.recent_block_ptrs.back().unwrap().number, 11);
    }
}
