mod new_radio;
mod register;

pub const FAMILY_NAME: &str = "MAC80211_HWSIM";

/// CMD_ID
///
/// Every messsage represents a command. This constant is used to identify the command.
///
/// # NOTE
///
/// Assign a new value to this constant if you add a new command.
pub const HWSIM_CMD_REGISTER: u8 = 1;
// pub const HWSIM_CMD_FRAME: u8 = 2;
// pub const HWSIM_CMD_TX_INFO_FRAME: u8 = 3;
pub const HWSIM_CMD_NEW_RADIO: u8 = 4;
// pub const HWSIM_CMD_DEL_RADIO: u8 = 5;
// pub const HWSIM_CMD_GET_RADIO: u8 = 6;
// pub const HWSIM_CMD_ADD_MAC_ADDR: u8 = 7;
// pub const HWSIM_CMD_DEL_MAC_ADDR: u8 = 8;
