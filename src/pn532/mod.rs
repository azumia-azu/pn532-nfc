use core::panicking::panic;
use std::fmt;
use std::io;
use std::error::Error;

use log::{info, warn, debug, error};

const PREAMBLE: u8 =    0x00;
const STARTCODE1: u8 =  0x00;
const STARTCODE2: u8 =  0xFF;
const POSTAMBLE: u8 =   0x00;

const HOSTTOPN532: u8 = 0xD4;
const PN532TOHOST: u8 = 0xD5;

// PN532 Commands
const COMMAND_DIAGNOSE: u8 =                0x00;
const COMMAND_GETFIRMWAREVERSION: u8 =      0x02;
const COMMAND_GETGENERALSTATUS: u8 =        0x04;
const COMMAND_READREGISTER: u8 =            0x06;
const COMMAND_WRITEREGISTER: u8 =           0x08;
const COMMAND_READGPIO: u8 =                0x0C;
const COMMAND_WRITEGPIO: u8 =               0x0E;
const COMMAND_SETSERIALBAUDRATE: u8 =       0x10;
const COMMAND_SETPARAMETERS: u8 =           0x12;
const COMMAND_SAMCONFIGURATION: u8 =        0x14;
const COMMAND_POWERDOWN: u8 =               0x16;
const COMMAND_RFCONFIGURATION: u8 =         0x32;
const COMMAND_RFREGULATIONTEST: u8 =        0x58;
const COMMAND_INJUMPFORDEP: u8 =            0x56;
const COMMAND_INJUMPFORPSL: u8 =            0x46;
const COMMAND_INLISTPASSIVETARGET: u8 =     0x4A;
const COMMAND_INATR: u8 =                   0x50;
const COMMAND_INPSL: u8 =                   0x4E;
const COMMAND_INDATAEXCHANGE: u8 =          0x40;
const COMMAND_INCOMMUNICATETHRU: u8 =       0x42;
const COMMAND_INDESELECT: u8 =              0x44;
const COMMAND_INRELEASE: u8 =               0x52;
const COMMAND_INSELECT: u8 =                0x54;
const COMMAND_INAUTOPOLL: u8 =              0x60;
const COMMAND_TGINITASTARGET: u8 =          0x8C;
const COMMAND_TGSETGENERALBYTES: u8 =       0x92;
const COMMAND_TGGETDATA: u8 =               0x86;
const COMMAND_TGSETDATA: u8 =               0x8E;
const COMMAND_TGSETMETADATA: u8 =           0x94;
const COMMAND_TGGETINITIATORCOMMAND: u8 =   0x88;
const COMMAND_TGRESPONSETOINITIATOR: u8 =   0x90;
const COMMAND_TGGETTARGETSTATUS: u8 =       0x8A;

const RESPONSE_INDATAEXCHANGE: u8 =         0x41;
const RESPONSE_INLISTPASSIVETARGET: u8 =    0x4B;

const WAKEUP: u8 = 0x55;

const MIFARE_ISO14443A: u8 = 0x00;

// Mifare Commands
const MIFARE_CMD_AUTH_A: u8 =           0x60;
const MIFARE_CMD_AUTH_B: u8 =           0x61;
const MIFARE_CMD_READ: u8 =             0x30;
const MIFARE_CMD_WRITE: u8 =            0xA0;
const MIFARE_CMD_TRANSFER: u8 =         0xB0;
const MIFARE_CMD_DECREMENT: u8 =        0xC0;
const MIFARE_CMD_INCREMENT: u8 =        0xC1;
const MIFARE_CMD_STORE: u8 =            0xC2;
const MIFARE_ULTRALIGHT_CMD_WRITE: u8 = 0xA2;

// Prefixes for NDEF Records (to identify record type)
const NDEF_URIPREFIX_NONE: u8 =         0x00;
const NDEF_URIPREFIX_HTTP_WWWDOT: u8 =  0x01;
const NDEF_URIPREFIX_HTTPS_WWWDOT: u8 = 0x02;
const NDEF_URIPREFIX_HTTP: u8 =         0x03;
const NDEF_URIPREFIX_HTTPS: u8 =        0x04;
const NDEF_URIPREFIX_TEL: u8 =          0x05;
const NDEF_URIPREFIX_MAILTO: u8 =       0x06;
const NDEF_URIPREFIX_FTP_ANONAT: u8 =   0x07;
const NDEF_URIPREFIX_FTP_FTPDOT: u8 =   0x08;
const NDEF_URIPREFIX_FTPS: u8 =         0x09;
const NDEF_URIPREFIX_SFTP: u8 =         0x0A;
const NDEF_URIPREFIX_SMB: u8 =          0x0B;
const NDEF_URIPREFIX_NFS: u8 =          0x0C;
const NDEF_URIPREFIX_FTP: u8 =          0x0D;
const NDEF_URIPREFIX_DAV: u8 =          0x0E;
const NDEF_URIPREFIX_NEWS: u8 =         0x0F;
const NDEF_URIPREFIX_TELNET: u8 =       0x10;
const NDEF_URIPREFIX_IMAP: u8 =         0x11;
const NDEF_URIPREFIX_RTSP: u8 =         0x12;
const NDEF_URIPREFIX_URN: u8 =          0x13;
const NDEF_URIPREFIX_POP: u8 =          0x14;
const NDEF_URIPREFIX_SIP: u8 =          0x15;
const NDEF_URIPREFIX_SIPS: u8 =         0x16;
const NDEF_URIPREFIX_TFTP: u8 =         0x17;
const NDEF_URIPREFIX_BTSPP: u8 =        0x18;
const NDEF_URIPREFIX_BTL2CAP: u8 =      0x19;
const NDEF_URIPREFIX_BTGOEP: u8 =       0x1A;
const NDEF_URIPREFIX_TCPOBEX: u8 =      0x1B;
const NDEF_URIPREFIX_IRDAOBEX: u8 =     0x1C;
const NDEF_URIPREFIX_FILE: u8 =         0x1D;
const NDEF_URIPREFIX_URN_EPC_ID: u8 =   0x1E;
const NDEF_URIPREFIX_URN_EPC_TAG: u8 =  0x1F;
const NDEF_URIPREFIX_URN_EPC_PAT: u8 =  0x20;
const NDEF_URIPREFIX_URN_EPC_RAW: u8 =  0x21;
const NDEF_URIPREFIX_URN_EPC: u8 =      0x22;
const NDEF_URIPREFIX_URN_NFC: u8 =      0x23;

const GPIO_VALIDATIONBIT: u8 = 0x80;

const ACK: &[u8] = b"\x00\x00\xFF\x00\xFF\x00";
const FRAME_START: &[u8] = b"\x00\x00\xFF";

#[derive(Debug)]
pub struct BusyError;

impl fmt::Display for BusyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Busy Error!")
    }
}

impl Error for BusyError { }

#[derive(Debug)]
pub struct RuntimeError(String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    } 
}



#[derive(Debug)]
pub struct PN532Error {
    code: usize,
    msg: String,
}

impl PN532Error {
    fn error(code: usize) -> Self {
        let msg = match code {
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
            _ => panic!("Error State: Unexpected PN532 Error Code: {}", code)
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

    fn read_data(&self, len: usize) -> Vec<u8>;

    fn write_data(&self, frame: &[u8]);

    fn wait_ready(&self);

    fn wake_up(&self);

    fn write_frame(&self, data: &[u8]) {
        assert!(data.len() > 1 && data.len() < 255);

        let len = data.len() as u8;
        let mut frame = vec![0_u8; (len+7) as usize];
        frame[0] = PREAMBLE;
        frame[1] = STARTCODE1;
        frame[2] = STARTCODE2;
        let mut checksum = sum(frame[0..3]);
        frame[3] = len & 0xFF;
        frame[4] = (!len + 1) & 0xFF;
        frame[5..(len-2)].copy_from_slice(data);
        checksum += sum(data);
        frame[len-2] = !checksum & 0xFF;
        frame[len-1] = POSTAMBLE;

        debug!("Write frame: {:?}", frame);
        self.write_data(&frame);
    }

    fn read_frame(&self, len: usize) -> Result<Vec<u8>, RuntimeError> {
        let mut response = self.read_data(len + 7);
        debug!("Read frame: {:?}", response);

        let mut offset = 0_usize;
        while response[offset] == 0x00 {
            offset += 1;
            if offset >= response.len() {
                return Err(RuntimeError("Response frame preamble does not contain 0x00FF!".to_owned()));
            }
        }
        if response[offset] != 0xFF { 
            return Err(RuntimeError("Response frame preamble does not contain 0x00FF!".to_owned()));
        }
        offset += 1;
        if offset >= response.len() {
            return Err(RuntimeError("Response contains no data!".to_owned()));
        }
        
        let frame_len = response[offset];
        if (frame_len + response[offset + 1]) & 0xFF != 0 {
            return Err(RuntimeError("Response length checksum did not match length!".to_owned()));
        }
        let checksum = sum(response[offset+2..offset+2+frame_len+1]) & 0xFF;
        if checksum != 0 {
            return Err(RuntimeError(format!("Response checksum did not match expected value: {}", checksum)));
        }
        
        Ok(response[offset+2..offset+2+frame_len].into())
    }

    fn call_function(&self, command: u8, response_length: u8);

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
