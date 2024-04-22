use super::{Lcd, SubBank};

macro_rules! impl_display_interface {
    ($display_interface:ident) => {
        impl<S> $display_interface::WriteOnlyDataCommand for Lcd<S, u16>
        where
            S: SubBank,
        {
            fn send_commands(
                &mut self,
                cmd: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                use $display_interface::DataFormat;
                match cmd {
                    DataFormat::U8(slice) => {
                        for value in slice {
                            self.write_command(u16::from(*value));
                        }
                    }
                    DataFormat::U16(slice) => {
                        for value in slice {
                            self.write_command(*value);
                        }
                    }
                    DataFormat::U16BE(slice) | DataFormat::U16LE(slice) => {
                        // As long as the data bus is 16 bits wide, the byte order doesn't matter.
                        for value in slice {
                            self.write_command(*value);
                        }
                    }
                    DataFormat::U8Iter(iter) => {
                        for value in iter {
                            self.write_command(u16::from(value));
                        }
                    }
                    DataFormat::U16BEIter(iter) | DataFormat::U16LEIter(iter) => {
                        // As long as the data bus is 16 bits wide, the byte order doesn't matter.
                        for value in iter {
                            self.write_command(value);
                        }
                    }
                    _ => return Err($display_interface::DisplayError::DataFormatNotImplemented),
                }
                Ok(())
            }

            fn send_data(
                &mut self,
                buf: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                use $display_interface::DataFormat;
                match buf {
                    DataFormat::U8(slice) => {
                        for value in slice {
                            self.write_data(u16::from(*value));
                        }
                    }
                    DataFormat::U16(slice) => {
                        for value in slice {
                            self.write_data(*value);
                        }
                    }
                    DataFormat::U16BE(slice) | DataFormat::U16LE(slice) => {
                        // As long as the data bus is 16 bits wide, the byte order doesn't matter.
                        for value in slice {
                            self.write_data(*value);
                        }
                    }
                    DataFormat::U8Iter(iter) => {
                        for value in iter {
                            self.write_data(u16::from(value));
                        }
                    }
                    DataFormat::U16BEIter(iter) | DataFormat::U16LEIter(iter) => {
                        // As long as the data bus is 16 bits wide, the byte order doesn't matter.
                        for value in iter {
                            self.write_data(value);
                        }
                    }
                    _ => return Err($display_interface::DisplayError::DataFormatNotImplemented),
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_display_interface_u8 {
    ($display_interface:ident) => {
        impl<S> $display_interface::WriteOnlyDataCommand for Lcd<S, u8>
        where
            S: SubBank,
        {
            fn send_commands(
                &mut self,
                cmd: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                use $display_interface::DataFormat;
                match cmd {
                    DataFormat::U8(slice) => {
                        for value in slice {
                            self.write_command(*value);
                        }
                    }
                    DataFormat::U16(slice) => {
                        for value in slice {
                            let bytes = value.to_ne_bytes();
                            self.write_command(bytes[0]);
                            self.write_command(bytes[1]);
                        }
                    }
                    DataFormat::U16BE(slice) => {
                        for value in slice {
                            let bytes = value.to_be_bytes();
                            self.write_command(bytes[0]);
                            self.write_command(bytes[1]);
                        }
                    }
                    DataFormat::U16LE(slice) => {
                        for value in slice {
                            let bytes = value.to_le_bytes();
                            self.write_command(bytes[0]);
                            self.write_command(bytes[1]);
                        }
                    }
                    DataFormat::U8Iter(iter) => {
                        for value in iter {
                            self.write_command(value);
                        }
                    }
                    DataFormat::U16BEIter(iter) => {
                        for value in iter {
                            let bytes = value.to_be_bytes();
                            self.write_command(bytes[0]);
                            self.write_command(bytes[1]);
                        }
                    }
                    DataFormat::U16LEIter(iter) => {
                        for value in iter {
                            let bytes = value.to_le_bytes();
                            self.write_command(bytes[0]);
                            self.write_command(bytes[1]);
                        }
                    }
                    _ => return Err($display_interface::DisplayError::DataFormatNotImplemented),
                }
                Ok(())
            }

            fn send_data(
                &mut self,
                buf: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                use $display_interface::DataFormat;
                match buf {
                    DataFormat::U8(slice) => {
                        for value in slice {
                            self.write_data(*value);
                        }
                    }
                    DataFormat::U8Iter(iter) => {
                        for value in iter {
                            self.write_data(value);
                        }
                    }
                    DataFormat::U16(slice) => {
                        for value in slice {
                            let bytes = value.to_ne_bytes();
                            self.write_data(bytes[0]);
                            self.write_data(bytes[1]);
                        }
                    }
                    DataFormat::U16BE(slice) => {
                        for value in slice {
                            let bytes = value.to_be_bytes();
                            self.write_data(bytes[0]);
                            self.write_data(bytes[1]);
                        }
                    }
                    DataFormat::U16LE(slice) => {
                        for value in slice {
                            let bytes = value.to_le_bytes();
                            self.write_data(bytes[0]);
                            self.write_data(bytes[1]);
                        }
                    }
                    DataFormat::U16BEIter(iter) => {
                        for value in iter {
                            let bytes = value.to_be_bytes();
                            self.write_data(bytes[0]);
                            self.write_data(bytes[1]);
                        }
                    }
                    DataFormat::U16LEIter(iter) => {
                        for value in iter {
                            let bytes = value.to_le_bytes();
                            self.write_data(bytes[0]);
                            self.write_data(bytes[1]);
                        }
                    }
                    _ => return Err($display_interface::DisplayError::DataFormatNotImplemented),
                }
                Ok(())
            }
        }
    };
}

impl_display_interface!(display_interface);
impl_display_interface!(display_interface_04);
impl_display_interface_u8!(display_interface);
impl_display_interface_u8!(display_interface_04);
