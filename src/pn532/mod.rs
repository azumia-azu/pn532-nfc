use std::fmt;
use std::error::Error;

use log::{info, warn, debug, error};

pub struct BusyError;

impl Error for BusyError { }


#[derive(Debug)]
pub struct PN532Error {
    code: usize,
    msg: String,
}

impl PN532Error {
    fn error(code: usize) -> Self {
        let msg = match self.code {
            0x01 => "PN532 ERROR TIMEOUT",
            0x02 => "PN532 ERROR CRC",
            0x03 => "PN532 ERROR PARITY",
            0x04 => "PN532 ERROR COLLISION_BITCOUNT",
            0x05 => "PN532 ERROR MIFARE_FRAMING",
            0x06 => "PN532 ERROR MIFARE_FRAMING",
            0x07 => "PN532 ERROR NOBUFS",
            0x09 => "PN532 ERROR RFNOBUFS",
            0x0a => "PN532 ERROR ACTIVE_TOOSLOW",
            0x0b => "PN532 ERROR RFPROTO",
            0x0d => "PN532 ERROR TOOHOT",
            0x0e => "PN532 ERROR INTERNAL_NOBUFS",
            0x10 => "PN532 ERROR INVAL",
            0x12 => "PN532 ERROR DEP_INVALID_COMMAND",
            0x13 => "PN532 ERROR DEP_BADDATA",
            0x14 => "PN532 ERROR MIFARE_AUTH",
            0x18 => "PN532 ERROR NOSECURE",
            0x19 => "PN532 ERROR I2CBUSY",
            0x23 => "PN532 ERROR UIDCHECKSUM",
            0x25 => "PN532 ERROR DEPSTATE",
            0x26 => "PN532 ERROR HCIINVAL",
            0x27 => "PN532 ERROR CONTEXT",
            0x29 => "PN532 ERROR RELEASED",
            0x2a => "PN532 ERROR CARDSWAPPED",
            0x2b => "PN532 ERROR NOCARD",
            0x2c => "PN532 ERROR MISMATCH",
            0x2d => "PN532 ERROR OVERCURRENT",
            0x2e => "PN532 ERROR NONAD",
        }.to_owned();

        Self {
            code,
            msg
        }
    }
}

impl fmt::Display for PN532Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

trait PN532 {
    fn gpio_init(&self);

    fn reset(&self);

    fn read_data(&self);

    fn write_data(&self);

    fn wait_ready(&self);

    fn wake_up(&self);

    fn write_frame(&self);

    fn read_frame(&self);

    fn call_function(&self);

    fn get_firmware_version(&self);

    fn SAM_configuration(&self);

    fn read_passive_target(&self);

    fn mifare_classic_authenticate_block(&self);

    fn mifare_classic_read_block(&self);

    fn mifare_classic_write_block(&self);

    fn ntag2xx_write_block(&self);
    
    fn ntag2xx_read_block(&self);

    fn read_gpio(&self);

    fn write_gpio(&self);

    fn tg_init_as_target(&self);

}