use message_macro::generate_emitable;
use netlink_packet_generic::GenlFamily;
use netlink_packet_utils::Emitable;

use super::{FAMILY_NAME, HWSIM_CMD_REGISTER};

#[generate_emitable]
pub struct Resgiter;

impl GenlFamily for Resgiter {
    fn family_name() -> &'static str {
        FAMILY_NAME
    }

    fn command(&self) -> u8 {
        HWSIM_CMD_REGISTER
    }

    fn version(&self) -> u8 {
        1
    }
}
