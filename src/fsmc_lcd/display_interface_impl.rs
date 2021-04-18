use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

use super::{Lcd, SubBank};

impl<S> WriteOnlyDataCommand for Lcd<S>
where
    S: SubBank,
{
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
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
            _ => return Err(DisplayError::DataFormatNotImplemented),
        }
        Ok(())
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
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
            _ => return Err(DisplayError::DataFormatNotImplemented),
        }
        Ok(())
    }
}
