pub mod helpers;
pub mod types;

#[cosmwasm_schema::cw_serde]
pub enum JackalMsg {
    /// Add viewers to file
    AddViewers {
        viewer_ids: String,
        viewer_keys: String,
        address: String,
        owner: String,
    },
    /// buy storage
    BuyStorage {
        for_address: String,
        duration_days: u64,
        bytes: u64,
        payment_denom: String,
    },
    ///
    UpgradeStorage {
        for_address: String,
        duration_days: u64,
        bytes: u64,
        payment_denom: String,
    },
    /// cancel an active contract via cid
    CancelContract {
        cid: String,
    },
    Delete {},
    /// create absolute root folder for your accounts storage on jackal
    MakeRoot {
        /// ?
        editors: String,
        /// ?
        viewers: String,
        /// unique id used in editors map??
        tracking_number: String,
    },
    /// create and save new file or folder.
    PostFile {
        hash_parent: String,
        hash_child: String,
        contents: String,
        viewers: String,
        editors: String,
        tracking_number: String,
    },
    /// Post a ecies.PublicKey
    PostKey {
        key: String,
    },
    DeleteViewers {
        viewer_ids: String,
        address: String,
        owner: String,
    },
    SignContract {
        cid: String,
    },
}
