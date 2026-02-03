// Copyright (c) Walrus Foundation
// SPDX-License-Identifier: Apache-2.0

//! A module defining the Balance struct.

use anyhow::Context;
use move_core_types::language_storage::StructTag;

use crate::coin::Coin;

#[derive(Debug, thiserror::Error)]
#[error("error processing balance information: {0}")]
pub struct BalanceError(anyhow::Error);

impl From<anyhow::Error> for BalanceError {
    fn from(err: anyhow::Error) -> Self {
        BalanceError(err)
    }
}

/// A struct representing the balance of a specific coin type.
#[derive(Debug, Clone)]
pub struct Balance {
    /// The coin type.
    coin_type: StructTag,
    /// The total balance across all coin objects.
    total_balance: u128,
    /// The number of coin objects or the actual coin objects.
    coin_balance: CoinBalance,
}

/// An enum representing either the count of coin objects or the actual coin objects.
#[derive(Debug, Clone)]
pub enum CoinBalance {
    /// The count of coin objects.
    Count(usize),
    /// The actual coin objects.
    Coins(Vec<Coin>),
}

impl Balance {
    pub fn try_from_coins(coin_type: &str, coins: Vec<Coin>) -> Result<Self, BalanceError> {
        let coin_type: StructTag = coin_type.parse().context("invalid coin type")?;
        let total_balance = coins
            .iter()
            .map(|coin| {
                assert_eq!(
                    coin.coin_type
                        .parse::<StructTag>()
                        .expect("coin must have a valid type"),
                    coin_type
                );
                u128::from(coin.balance)
            })
            .sum();
        Ok(Self {
            coin_type,
            total_balance,
            coin_balance: CoinBalance::Coins(coins),
        })
    }
    /// Returns the number of coin objects.
    pub fn coin_object_count(&self) -> usize {
        match self.coin_balance {
            CoinBalance::Count(count) => count,
            CoinBalance::Coins(ref coins) => coins.len(),
        }
    }

    pub(crate) fn total_balance(&self) -> u128 {
        self.total_balance
    }
}

impl TryFrom<sui_sdk::rpc_types::Balance> for Balance {
    type Error = BalanceError;

    fn try_from(balance: sui_sdk::rpc_types::Balance) -> Result<Self, BalanceError> {
        Ok(Self {
            coin_type: balance
                .coin_type
                .parse::<StructTag>()
                .context("invalid coin type")?,
            coin_balance: CoinBalance::Count(balance.coin_object_count as usize),
            total_balance: balance.total_balance,
        })
    }
}
