use std::mem::size_of_val;

use netlink_packet_utils::{parsers::parse_mac, DecodeError};

use super::{Emit, Parse};

pub struct MacAddr(pub [u8; 6]);

impl Parse for MacAddr {
    #[inline]
    fn parse(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self(parse_mac(buf)?))
    }
}

impl Emit for MacAddr {
    #[inline]
    fn value_len(&self) -> usize {
        size_of_val(&self.0)
    }

    #[inline]
    fn emit(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.0);
    }
}
