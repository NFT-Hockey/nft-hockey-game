use near_sdk::{AccountId, Balance};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use crate::{Hockey, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};



#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameConfig {
    pub(crate) deposit: Option<Balance>,
    pub(crate) opponent_id: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VGameConfig {
    Current(GameConfig)
}

impl From<VGameConfig> for GameConfig {
    fn from(v_game_config: VGameConfig) -> Self {
        match v_game_config {
            VGameConfig::Current(game_config) => game_config,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameConfigOutput {
    deposit: U128,
    opponent_id: Option<AccountId>,
}

impl From<GameConfig> for GameConfigOutput {
    fn from(config: GameConfig) -> Self {
        GameConfigOutput {
            deposit: U128::from(config.deposit.unwrap_or(0)),
            opponent_id: config.opponent_id,
        }
    }
}

#[derive(PartialEq)]
pub enum UpdateStatsAction {
    AddPlayedGame,
    AddReferral,
    AddAffiliate,
    AddWonGame,
    AddTotalReward,
    AddAffiliateReward,
    AddPenaltyGame,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Stats {
    referrer_id: Option<AccountId>,
    affiliates: UnorderedSet<AccountId>,
    games_num: u64,
    victories_num: u64,
    penalties_num: u64,
    total_reward: UnorderedMap<Option<AccountId>, Balance>,
    total_affiliate_reward: UnorderedMap<Option<AccountId>, Balance>,
}

impl Stats {
    pub fn new(account_id: &AccountId) -> Stats {
        Stats {
            referrer_id: None,
            affiliates: UnorderedSet::new(StorageKey::Affiliates { account_id: account_id.clone() }),
            games_num: 0,
            victories_num: 0,
            penalties_num: 0,
            total_reward: UnorderedMap::new(StorageKey::TotalRewards { account_id: account_id.clone() }),
            total_affiliate_reward: UnorderedMap::new(StorageKey::TotalAffiliateRewards { account_id: account_id.clone() }),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VStats {
    Current(Stats),
}

impl From<VStats> for Stats {
    fn from(v_stats: VStats) -> Self {
        match v_stats {
            VStats::Current(stats) => stats,
        }
    }
}

impl Hockey {
    pub(crate) fn internal_update_stats(&mut self,
                                        account_id: &AccountId,
                                        action: UpdateStatsAction,
                                        additional_account_id: Option<AccountId>,
                                        balance: Option<Balance>) {
        let mut stats = self.internal_get_stats(account_id);

        if action == UpdateStatsAction::AddPlayedGame {
            stats.games_num += 1
        } else if action == UpdateStatsAction::AddReferral {
            if additional_account_id.is_some() {
                stats.referrer_id = additional_account_id;
            }
        } else if action == UpdateStatsAction::AddAffiliate {
            if let Some(additional_account_id_unwrapped) = additional_account_id {
                stats.affiliates.insert(&additional_account_id_unwrapped);
            }
        } else if action == UpdateStatsAction::AddWonGame {
            stats.victories_num += 1;
        } else if action == UpdateStatsAction::AddTotalReward {
            if let Some(balance_unwrapped) = balance {
                // TODO Add FT
                let total_reward = stats.total_reward.get(&None).unwrap_or(0);
                stats.total_reward.insert(&None, &(total_reward + balance_unwrapped));
            }
        } else if action == UpdateStatsAction::AddAffiliateReward {
            if let Some(balance_unwrapped) = balance {
                // TODO Add FT
                let total_affiliate_reward = stats.total_affiliate_reward.get(&None).unwrap_or(0);
                stats.total_affiliate_reward.insert(&None, &(total_affiliate_reward + balance_unwrapped));
            }
        } else if action == UpdateStatsAction::AddPenaltyGame {
            stats.penalties_num += 1;
        }

        self.stats.insert(account_id, &VStats::Current(stats));
    }

    pub(crate) fn internal_get_stats(&self, account_id: &AccountId) -> Stats {
        if let Some(stats) = self.stats.get(account_id) {
            stats.into()
        } else {
            Stats::new(&account_id)
        }
    }

    pub(crate) fn is_account_exists(&self, account_id: &Option<AccountId>) -> bool {
        if let Some(account_id_unwrapped) = account_id {
            self.stats.get(account_id_unwrapped).is_some()
        } else {
            false
        }
    }
}