use crate::*;
use crate::team::Goalies;

#[near_bindgen]
impl Hockey {
    pub fn get_owner_team(&mut self, account_id: AccountId) -> Promise {
        ext_manage_team::get_owner_team(account_id, &NFT_CONTRACT, 0, 100_000_000_000_000)
    }

    pub fn insert_nft_field_players(&mut self, five: Fives,
                                    players: Vec<(PlayerPosition, TokenId)>) -> Promise {
        ext_manage_team::insert_nft_field_players(five, players, &NFT_CONTRACT, 0, 50_000_000_000_000)
    }

    pub fn insert_nft_goalie(&mut self, priority: Goalies, token_id: TokenId) -> Promise {
        ext_manage_team::insert_nft_goalie(priority, token_id, &NFT_CONTRACT, 0, 50_000_000_000_000)
    }
}