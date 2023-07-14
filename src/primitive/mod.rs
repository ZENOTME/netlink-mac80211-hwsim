use netlink_packet_utils::byteorder::ByteOrder;
use netlink_packet_utils::byteorder::NativeEndian;
use netlink_packet_utils::parsers::parse_string;
use netlink_packet_utils::parsers::parse_u32;
use netlink_packet_utils::parsers::parse_u64;
use netlink_packet_utils::DecodeError;
use paste::paste;

mod mac_addr;
pub use mac_addr::MacAddr;
mod tx_info;

pub trait Parse
where
    Self: Sized,
{
    fn parse(buf: &[u8]) -> Result<Self, DecodeError>;
}

pub trait Emit
where
    Self: Sized,
{
    fn value_len(&self) -> usize;
    fn emit(&self, buf: &mut [u8]);
}

macro_rules! impl_for_primitive_type {
    ($ty:ty) => {
        paste! {
            impl Parse for $ty {
                #[inline]
                fn parse(buf: &[u8]) -> Result<Self, DecodeError> {
                    [<parse_ $ty>](buf)
                }
            }

            impl Emit for $ty {
                #[inline]
                fn value_len(&self) -> usize {
                    std::mem::size_of_val(self)
                }

                #[inline]
                fn emit(&self, buf: &mut [u8]) {
                    NativeEndian::[<write_ $ty>](buf, *self)
                }
            }
        }
    };
}

impl_for_primitive_type!(u32);
impl_for_primitive_type!(u64);

impl Parse for String {
    #[inline]
    fn parse(buf: &[u8]) -> Result<Self, DecodeError> {
        parse_string(buf)
    }
}

impl Emit for String {
    #[inline]
    fn value_len(&self) -> usize {
        self.as_bytes().len()
    }

    #[inline]
    fn emit(&self, buf: &mut [u8]) {
        buf.copy_from_slice(self.as_bytes());
    }
}
