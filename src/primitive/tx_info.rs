use std::mem::size_of_val;

use super::{Emit, Parse};
use anyhow::anyhow;
use netlink_packet_utils::{parsers::parse_u8, DecodeError};

const IEEE80211_TX_MAX_RATES: usize = 4;

#[derive(Clone, Copy)]
pub struct TxRate {
    pub rate: i8,
    pub count: u8,
}

pub struct TxInfo([TxRate; IEEE80211_TX_MAX_RATES]);

impl Parse for TxInfo {
    #[inline]
    fn parse(buf: &[u8]) -> Result<Self, DecodeError> {
        if buf.len() < 8 {
            return Err(anyhow!("parse buf.len() < 8 for TxInfo").into());
        }
        let mut rates = [TxRate { rate: 0, count: 0 }; IEEE80211_TX_MAX_RATES];
        for (idx, rate) in rates.iter_mut().enumerate() {
            rate.rate = buf[idx * 2] as i8;
            rate.count = parse_u8(&buf[idx * 2 + 1..])?;
        }

        Ok(TxInfo(rates))
    }
}

impl Emit for TxInfo {
    #[inline]
    fn value_len(&self) -> usize {
        size_of_val(&self.0)
    }

    #[inline]
    fn emit(&self, buf: &mut [u8]) {
        let mut idx = 0;
        for i in 0..IEEE80211_TX_MAX_RATES {
            buf[idx] = self.0[i].rate as u8;
            buf[idx + 1] = self.0[i].count;
            idx += 2;
        }
    }
}
