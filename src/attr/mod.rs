use crate::primitive::Emit;
use crate::primitive::MacAddr;
use crate::primitive::Parse;
use anyhow::{anyhow, Context};
use netlink_packet_utils::nla::Nla;
use netlink_packet_utils::nla::NlaBuffer;
use netlink_packet_utils::nla::NlasIterator;
use netlink_packet_utils::nla::NLA_F_NESTED;
use netlink_packet_utils::nla::NLA_F_NET_BYTEORDER;
use netlink_packet_utils::nla::NLA_HEADER_SIZE;
use netlink_packet_utils::Emitable;
use paste::paste;

#[macro_use]
mod define_macro;

/// ATTR_KIND_ID
///
/// # NOTE
///
/// Assign a new value to this constant if you add a new attribute.
pub const HWSIM_ATTR_FLAGS: u16 = 4;
// pub const HWSIM_ATTR_UNSPEC: u16 = 0;
pub const HWSIM_ATTR_ADDR_RECEIVER: u16 = 1;
pub const HWSIM_ATTR_ADDR_TRANSMITTER: u16 = 2;
// pub const HWSIM_ATTR_FRAME: u16 = 3;
pub const HWSIM_ATTR_RX_RATE: u16 = 5;
pub const HWSIM_ATTR_SIGNAL: u16 = 6;
// pub const HWSIM_ATTR_TX_INFO: u16 = 7;
pub const HWSIM_ATTR_COOKIE: u16 = 8;
pub const HWSIM_ATTR_CHANNELS: u16 = 9;
pub const HWSIM_ATTR_RADIO_ID: u16 = 10;
pub const HWSIM_ATTR_REG_HINT_ALPHA2: u16 = 11;
pub const HWSIM_ATTR_REG_CUSTOM_REG: u16 = 12;
pub const HWSIM_ATTR_REG_STRICT_REG: u16 = 13;
pub const HWSIM_ATTR_SUPPORT_P2P_DEVICE: u16 = 14;
pub const HWSIM_ATTR_USE_CHANCTX: u16 = 15;
pub const HWSIM_ATTR_DESTROY_RADIO_ON_CLOSE: u16 = 16;
pub const HWSIM_ATTR_RADIO_NAME: u16 = 17;
pub const HWSIM_ATTR_NO_VIF: u16 = 18;
pub const HWSIM_ATTR_FREQ: u16 = 19;
// pub const HWSIM_ATTR_PAD: u16 = 20;
// pub const HWSIM_ATTR_TX_INFO_FLAGS: u16 = 21;
pub const HWSIM_ATTR_PERM_ADDR: u16 = 22;
// pub const HWSIM_ATTR_IFTYPE_SUPPORT: u16 = 23;
// pub const HWSIM_ATTR_CIPHER_SUPPORT: u16 = 24;

pub trait ParseableMut
where
    Self: Sized,
{
    fn parse(
        iter: &mut NlasIterator<&impl AsRef<[u8]>>,
    ) -> Result<Self, netlink_packet_utils::DecodeError>;
}

define_attr!(Flags, u32, HWSIM_ATTR_FLAGS);
define_attr!(RXRate, u32, HWSIM_ATTR_RX_RATE);
define_attr!(Signal, u32, HWSIM_ATTR_SIGNAL);
define_attr!(Cookie, u64, HWSIM_ATTR_COOKIE);
define_attr!(Channels, u32, HWSIM_ATTR_CHANNELS);
define_attr!(RadioID, u32, HWSIM_ATTR_RADIO_ID);
define_attr!(RegCustomReg, u32, HWSIM_ATTR_REG_CUSTOM_REG);
define_attr!(Freq, u32, HWSIM_ATTR_FREQ);
define_attr!(RegStrictReg, bool, HWSIM_ATTR_REG_STRICT_REG);
define_attr!(SupportP2PDevice, bool, HWSIM_ATTR_SUPPORT_P2P_DEVICE);
define_attr!(UseChanctx, bool, HWSIM_ATTR_USE_CHANCTX);
define_attr!(DestroyRadioOnClose, bool, HWSIM_ATTR_DESTROY_RADIO_ON_CLOSE);
define_attr!(NoVif, bool, HWSIM_ATTR_NO_VIF);
define_attr!(RegHintAlpha2, String, HWSIM_ATTR_REG_HINT_ALPHA2);
define_attr!(RadioName, String, HWSIM_ATTR_RADIO_NAME);
define_attr!(AddrReciver, MacAddr, HWSIM_ATTR_ADDR_RECEIVER);
define_attr!(AddrTransmitter, MacAddr, HWSIM_ATTR_ADDR_TRANSMITTER);
define_attr!(PermAddr, MacAddr, HWSIM_ATTR_PERM_ADDR);
