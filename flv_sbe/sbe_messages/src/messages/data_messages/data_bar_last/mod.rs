mod sbe_decode;
mod sbe_encode;

use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastDataBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastDataBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastDataBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for LastDataBar {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_data_bar_message(value)
            .expect("Failed to decode LastDataBar message")
    }
}

impl LastDataBar {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}

impl fmt::Display for LastDataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LastDataBar {{ message_type: {:?} }}", self.message_type)
    }
}
