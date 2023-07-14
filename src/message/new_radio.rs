use message_macro::generate_emitable;
use netlink_packet_generic::GenlFamily;

use crate::attr::{
    Channels, DestroyRadioOnClose, NoVif, PermAddr, RadioName, SupportP2PDevice, UseChanctx,
};
use netlink_packet_utils::nla::NLA_ALIGNTO;
use netlink_packet_utils::Emitable;

use super::{FAMILY_NAME, HWSIM_CMD_NEW_RADIO};

#[generate_emitable]
struct NewRadio {
    channels: Channels,
    support_p2p_device: SupportP2PDevice,
    use_chanctx: UseChanctx,
    destory_radio_on_close: DestroyRadioOnClose,
    radio_name: RadioName,
    no_vif: NoVif,
    perm_addr: PermAddr,
}

impl GenlFamily for NewRadio {
    fn family_name() -> &'static str {
        FAMILY_NAME
    }

    fn command(&self) -> u8 {
        HWSIM_CMD_NEW_RADIO
    }

    fn version(&self) -> u8 {
        1
    }
}
