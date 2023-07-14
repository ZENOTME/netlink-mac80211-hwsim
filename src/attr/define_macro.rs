/// This module defines a macro which help to define attr.
macro_rules! define_attr {
    ($name:ident, bool, $kind:ident) => {
        define_bool!($name, bool, $kind);
    };
    ($name:ident, $ty:ty, $kind:ident) => {
        define_general!($name, $ty, $kind);
    };
}

macro_rules! define_general {
    ($name:ident, $ty:ty, $kind:ident) => {
        paste!{
            pub struct $name(pub $ty);

            impl Nla for $name {
                fn value_len(&self) -> usize {
                    self.0.value_len()
                }

                fn kind(&self) -> u16 {
                    $kind
                }

                fn emit_value(&self, buffer: &mut [u8]) {
                    self.0.emit(buffer)
                }
            }

            impl ParseableMut for $name {
                fn parse(iter: &mut NlasIterator<&impl AsRef<[u8]>>) -> Result<Self, netlink_packet_utils::DecodeError> {
                    let buf = iter.next().ok_or(anyhow!("Missing attr {}",stringify!($name)))??;
                    if buf.kind() != $kind {
                        return Err(anyhow!("Can't parse kind {} as {}", buf.kind(),stringify!($name)).into());
                    }
                    let payload = buf.value();
                    Ok($name($ty::parse(payload).context(format!("failed to parse {}",stringify!($name)))?))
                }
            }
        }
    };
}

macro_rules! define_bool {
    ($name:ident, bool, $kind:ident) => {
        paste!{
            pub struct $name(pub bool);

            impl Emitable for $name {
                fn buffer_len(&self) -> usize {
                    NLA_HEADER_SIZE
                }

                fn emit(&self, buffer: &mut [u8]) {
                    if self.0 == false {
                        return;
                    }

                    let mut buffer = NlaBuffer::new(buffer);
                    buffer.set_kind($kind);

                    if ($kind & NLA_F_NET_BYTEORDER) != 0 {
                        buffer.set_network_byte_order_flag()
                    }

                    if ($kind & NLA_F_NESTED) != 0 {
                        buffer.set_nested_flag()
                    }

                    buffer.set_length(NLA_HEADER_SIZE as u16);
                }
            }

            impl ParseableMut for $name {
                fn parse(iter: &mut NlasIterator<&impl AsRef<[u8]>>) -> Result<Self, netlink_packet_utils::DecodeError> {
                    match iter.peekable().peek() {
                        Some(Ok(buf)) => {
                            if buf.kind() != $kind {
                                return Ok($name(false));
                            } else {
                                iter.next().unwrap().unwrap();
                                return Ok($name(true));
                            }
                        },
                        Some(Err(_)) => {
                            if let Err(e) = iter.next().unwrap() {
                                return Err(e);
                            } else {
                                unreachable!();
                            }
                        },
                        None => {
                            return Ok($name(false));
                        }
                    }
                }
            }
        }
    };
}
