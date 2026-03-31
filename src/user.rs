// displaying users in the user list (ui.rs)
use std::{fmt::Display};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub uuid: u32,
    pub admin: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "{: <16} | {: <25} | {: <5} | admin: {: <5} | active: {: <5}", 
            self.username, 
            self.email,
            self.uuid,
            self.admin,
            self.active,
        )
    }
}
