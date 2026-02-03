// Copyright (c) Walrus Foundation
// SPDX-License-Identifier: Apache-2.0

//! A module defining the Balance struct.

use anyhow::Context;
use sui_rpc::proto::sui::rpc::v2::GetBalanceResponse;

/// A struct representing the balance of a specific coin type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Balance {
    /// The coin type.
    pub coin_type: String,
    /// The number of coin objects.
    pub coin_object_count: usize,
    /// The total balance across all coin objects.
    pub total_balance: u128,
}

impl From<sui_sdk::rpc_types::Balance> for Balance {
    fn from(balance: sui_sdk::rpc_types::Balance) -> Self {
        Self {
            coin_type: balance.coin_type,
            coin_object_count: balance.coin_object_count,
            total_balance: balance.total_balance,
        }
    }
}

impl TryFrom<GetBalanceResponse> for Balance {
    type Error = anyhow::Error;
    fn try_from(response: GetBalanceResponse) -> Result<Self, Self::Error> {
        let balance = response
            .balance
            .context("missing balance in GetBalanceResponse")?;
        Ok(Self {
            coin_type: balance.coin_type.context("missing coin_type in Balance")?,
            coin_object_count: 0,
            total_balance: balance
                .coin_balance
                .context("missing coin_balance in Balance")?,
        })
    }
}
