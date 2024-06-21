//! # filetree
//!
//! Contains all the transaction msgs needed to interact with canine-chain's filetree module.
//! TODO: add remaining msgs and storage module's transaction msgs

/// Create an absolute root folder for a storage account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBuyStorage {
    /// jkl address of public key owner
    #[prost(string, tag = "1")]
    pub creator: String,
    /// for_address
    #[prost(string, tag = "2")]
    pub for_address: String,
    /// duration_days
    #[prost(uint64, tag = "3")]
    pub duration_days: u64,
    /// storage space to buy
    #[prost(uint64, tag = "4")]
    pub bytes: u64,
    /// payment_denom
    #[prost(string, tag = "5")]
    pub payment_denom: String,
}
/// Sign Contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignContract {
    /// jkl address of public key owner
    #[prost(string, tag = "1")]
    pub creator: String,
    /// contract id
    #[prost(string, tag = "2")]
    pub cid: String,
}
/// Cancel Contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelContract {
    /// jkl address of public key owner
    #[prost(string, tag = "1")]
    pub creator: String,
    /// contract id
    #[prost(string, tag = "2")]
    pub cid: String,
}
/// Upgrade Storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeStorage {
    /// jkl address of public key owner
    #[prost(string, tag = "1")]
    pub creator: String,
    /// for_address
    #[prost(string, tag = "2")]
    pub for_address: String,
    /// duration_days
    #[prost(uint64, tag = "3")]
    pub duration_days: u64,
    /// storage space to buy
    #[prost(uint64, tag = "4")]
    pub bytes: u64,
    /// payment_denom
    #[prost(string, tag = "5")]
    pub payment_denom: String,
}