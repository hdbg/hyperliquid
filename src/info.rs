use std::{collections::HashMap, time::SystemTime};

use ethers::types::Address;

use crate::{
    client::Client,
    error::Result,
    types::{
        exchange::request::Chain,
        info::{
            request::{CandleSnapshotRequest, Request},
            response::{
                AssetContext, CandleSnapshot, FrontendOpenOrders, FundingHistory, L2Book,
                OpenOrder, Universe, UserFill, UserFunding, UserState,
            },
        },
        API,
    },
};

/// Endpoint to fetch information about the exchange and specific users.
pub struct Info {
    pub client: Client,
    pub chain: Chain,
}

impl Info {
    /// Retrieve exchange metadata
    pub async fn metadata(&self) -> Result<Universe> {
        self.client.post(&API::Info, &Request::Meta).await
    }

    /// Retrieve all mids for all actively traded coins
    pub async fn mids(&self) -> Result<HashMap<String, String>> {
        self.client.post(&API::Info, &Request::AllMids).await
    }

    /// Retrieve asset contexts i.e mark price, current funding, open interest, etc
    pub async fn contexts(&self) -> Result<Vec<AssetContext>> {
        self.client
            .post(&API::Info, &Request::MetaAndAssetCtxs)
            .await
    }

    /// Retrieve a user's state to see user's open positions and margin summary
    ///
    /// # Arguments
    /// * `user` - The user's address
    pub async fn user_state(&self, user: Address) -> Result<UserState> {
        self.client
            .post(&API::Info, &Request::ClearinghouseState { user })
            .await
    }

    /// Retrieve a user's open orders
    ///
    /// # Arguments
    /// * `user` - The user's address
    pub async fn open_orders(&self, user: Address) -> Result<Vec<OpenOrder>> {
        self.client
            .post(&API::Info, &Request::OpenOrders { user })
            .await
    }

    /// Retrieve a user's open orders with additional frontend info.
    /// This is useful for displaying orders in a UI
    ///
    /// # Arguments
    /// * `user` - The user's address
    pub async fn frontend_open_orders(&self, user: Address) -> Result<Vec<FrontendOpenOrders>> {
        self.client
            .post(&API::Info, &Request::FrontendOpenOrders { user })
            .await
    }

    /// Retrieve a user's Userfills
    ///
    /// # Arguments
    /// * `user` - The user's address
    pub async fn user_fills(&self, user: Address) -> Result<Vec<UserFill>> {
        self.client
            .post(&API::Info, &Request::UserFills { user })
            .await
    }

    /// Retrieve a user's fills by time
    ///
    /// # Arguments
    /// * `user` - The user's address
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    /// * Returns a number of fills limited to 2000
    pub async fn user_fills_by_time(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFill>> {
        self.client
            .post(
                &API::Info,
                &Request::UserFillsByTime {
                    user,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve a user's funding history
    ///
    /// # Arguments
    /// * `user` - The user's address
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    pub async fn user_funding(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFunding>> {
        self.client
            .post(
                &API::Info,
                &Request::UserFunding {
                    user,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve historical funding rates for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve funding history for
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    pub async fn funding_history(
        &self,
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingHistory>> {
        self.client
            .post(
                &API::Info,
                &Request::FundingHistory {
                    coin,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve the L2 order book for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve the L2 order book for
    pub async fn l2_book(&self, coin: String) -> Result<L2Book> {
        self.client
            .post(&API::Info, &Request::L2Book { coin })
            .await
    }

    /// Retrieve candle snapshot for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve the candle snapshot for
    /// * `interval` - The interval to retrieve the candle snapshot for
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive.
    pub async fn candle_snapshot(
        &self,
        coin: String,
        interval: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<CandleSnapshot>> {
        self.client
            .post(
                &API::Info,
                &Request::CandleSnapshot {
                    req: CandleSnapshotRequest {
                        coin,
                        interval,
                        start_time,
                        end_time,
                    },
                },
            )
            .await
    }

    /// Query the status of an order
    pub async fn order_status(&self, user: Address, oid: u64) -> Result<()> {
        // TODO: This should return an OrderStatus
        self.client
            .post(&API::Info, &Request::OrderStatus { user, oid })
            .await
    }
}
