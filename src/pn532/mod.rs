use std::fmt;
use std::error::Error;
use std::result;
use std::rt::panic_display;

use log::{info, warn, debug, error};

pub mod spi;

type Result<U> = result::Result<U, Box<dyn Error>>;

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

pub enum PN532Gpio {
    P30,
    P31,
    P32,
    P33,
    P34,
    P35,
    P71,
    P72,
    I0,
    I1
}

impl PN532Gpio {
    fn idx(&self) -> usize {
        match self {
            PN532Gpio::P30 | PN532Gpio::P31 | PN532Gpio::P32 |
            PN532Gpio::P33 | PN532Gpio::P34 | PN532Gpio::P35 => 0,
            PN532Gpio::P71 | PN532Gpio::P72 => 1,
            PN532Gpio::I0 | PN532Gpio::I1 => 2,
        }
    }

    fn get(&self, response: u8) -> bool {
        response >> self.offset() & 1 == 1
    }

    fn offset(&self) -> u8 {
         match self {
            PN532Gpio::P30 | PN532Gpio::I0 => 0,
            PN532Gpio::P31 | PN532Gpio::P71 | PN532Gpio::I1 => 1,
            PN532Gpio::P32 | PN532Gpio::P72 => 2,
            PN532Gpio::P33 => 3,
            PN532Gpio::P34 => 4,
            PN532Gpio::P35 => 5,
        }
    }

}

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

impl Error for RuntimeError {}


#[derive(Debug)]
pub struct PN532Error {
    code: u8,
    msg: String,
}

impl PN532Error {
    fn error(code: u8) -> Self {
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

impl Error for PN532Error {}

trait PN532 {
    fn gpio_init(&self);

    fn reset(&self, pin: u8);

    fn read_data(&self, len: usize) -> Vec<u8>;

    fn write_data(&self, frame: &[u8]) -> Result<()>;

    fn wait_ready(&self, timeout: f64) -> bool;

    fn wake_up(&self);

    /// Write a frame to the PN532 with the specified data bytearray.
    fn write_frame(&self, data: &[u8]) -> Result<()> {
        assert!(data.len() > 1 && data.len() < 255);

        // Build frame to send as:
        // - Preamble (0x00)
        // - Start code  (0x00, 0xFF)
        // - Command length (1 byte)
        // - Command length checksum
        // - Command bytes
        // - Checksum
        // - Postamble (0x00)
        let len = data.len() as u8;
        let mut frame = vec![0_u8; (len+7) as usize];
        frame[0] = PREAMBLE;
        frame[1] = STARTCODE1;
        frame[2] = STARTCODE2;
        let mut checksum: u8 = frame[0..3].iter().sum();
        frame[3] = len & 0xFF;
        frame[4] = (!len + 1) & 0xFF;
        frame[5..(len as usize -2)].copy_from_slice(data);
        checksum += data.iter().sum::<u8>();
        frame[len as usize -2] = !checksum & 0xFF;
        frame[len as usize -1] = POSTAMBLE;

        debug!("Write frame: {:?}", frame);
        self.write_data(&frame)?;

        Ok(())
    }

    /// Read a response frame from the PN532 of at most length bytes in size.
    /// Returns the data inside the frame if found, otherwise raises an exception
    /// if there is an error parsing the frame.  Note that less than length bytes
    /// might be returned!
    fn read_frame(&self, len: usize) -> Result<Vec<u8>> {

        // Read frame with expected length of data.
        let response = self.read_data(len + 7);
        debug!("Read frame: {:?}", response);

        // Swallow all the 0x00 values that preceed 0xFF.
        let mut offset = 0_usize;
        while response[offset] == 0x00 {
            offset += 1;
            if offset >= response.len() {
                return Err(box RuntimeError("Response frame preamble does not contain 0x00FF!".to_owned()));
            }
        }
        if response[offset] != 0xFF { 
            return Err(box RuntimeError("Response frame preamble does not contain 0x00FF!".to_owned()));
        }
        offset += 1;
        if offset >= response.len() {
            return Err(box RuntimeError("Response contains no data!".to_owned()));
        }
        // Check length & length checksum match.
        let frame_len = response[offset];
        if (frame_len + response[offset + 1]) & 0xFF != 0 {
            return Err(box RuntimeError("Response length checksum did not match length!".to_owned()));
        }
        // Check frame checksum value matches bytes.
        let checksum: u8 = response[offset+2..offset+2+(frame_len as usize)+1].iter().sum::<u8>() & 0xFF;
        if checksum != 0 {
            return Err(box RuntimeError(format!("Response checksum did not match expected value: {}", checksum)));
        }
        // Return frame data.
        Ok(response[offset+2..offset+2+(frame_len as usize)].into())
    }

    /// Send specified command to the PN532 and expect up to response_length
    /// bytes back in a response.  Note that less than the expected bytes might
    /// be returned!  Params can optionally specify an array of bytes to send as
    /// parameters to the function call.  Will wait up to timeout seconds
    /// for a response and return a bytearray of response bytes, or None if no
    /// response is available within the timeout.
    fn call_function(&self, command: u8, response_length: usize, params: &[u8], timeout: f64) -> Result<Option<Vec<u8>>> {

        // Build frame data with command and parameters.
        let mut data = vec![0; 2 + params.len()];
        data[0] = HOSTTOPN532;
        data[1] = command & 0xFF;

        data[2..2+params.len()].copy_from_slice(params);
        debug!("Calling function.... send command: {}, by data: {:?}", command, data);

        // Send frame and wait for response.
        if let Err(e) = self.write_frame(data.as_slice()) {
            self.wake_up();
            return Err(e);
        }
        if !self.wait_ready(timeout) {
            return Ok(None);
        }
        // Verify ACK response and wait to be ready for function response.
        if ACK != self.read_data(ACK.len()) {
            return Err(box RuntimeError("Did not receive expected ACK from PN532!".to_owned()));
        }
        if !self.wait_ready(timeout) {
            return Ok(None);
        }
        // Read response bytes.
        let response = self.read_frame(response_length + 2)?;
        debug!("called function success!.... response: {:?}", response);
        // Check that response is for the called function.
        if !(response[0] == PN532TOHOST && response[1] == (command + 1)) {
            return Err(box RuntimeError("Received unexpected command response!".to_owned()));
        }

        // Return response data.
        Ok(Some(response[2..].to_owned()))
    }

    /// Call PN532 GetFirmwareVersion function and return a tuple with the IC,
    /// Ver, Rev, and Support values.
    fn get_firmware_version(&self) -> Result<Vec<u8>> {
        let response = self.call_function(COMMAND_GETFIRMWAREVERSION, 4, &[], 0.5)?;
        match response {
            Some(response) => Ok(response),
            None => Err(box RuntimeError("Failed to detect the PN532".to_owned()))
        }
    }

    /// Configure the PN532 to read MiFare cards.
    /// Send SAM configuration command with configuration for:
    /// - 0x01, normal mode
    /// - 0x14, timeout 50ms * 20 = 1 second
    /// - 0x01, use IRQ pin
    /// Note that no other verification is necessary as call_function will
    /// check the command was executed as expected.
    fn SAM_configuration(&self) -> Result<()> {
        self.call_function(COMMAND_SAMCONFIGURATION, 0,&[0x01, 0x14, 0x01], 1.0)?;
        Ok(())
    }

    /// Wait for a MiFare card to be available and return its UID when found.
    /// Will wait up to timeout seconds and return None if no card is found,
    /// otherwise a bytearray with the UID of the found card is returned.
    fn read_passive_target(&self, card_baud: Option<u8>, timeout: f64) -> Result<Option<Vec<u8>>> {
        // Send passive read command for 1 card.  Expect at most a 7 byte UUID.
        let response = self.call_function(
            COMMAND_INLISTPASSIVETARGET,
            19,
            &[0x01, card_baud.unwrap_or(MIFARE_ISO14443A)],
            timeout)?;
        match response {
            // If no response is available return None to indicate no card is present.
            None => Ok(None),
            Some(res) => {
                // Check only 1 card with up to a 7 byte UID is present.
                if res[0] != 0x01 {
                    return Err(box RuntimeError("More than one card detected!".to_owned()));
                }
                if res[5] > 7 {
                    return Err(box RuntimeError("Found card with unexpectedly long UID!".to_owned()));
                }
                // Return UID of card.
                return Ok(Some(res[6..6+(res[5] as usize)].to_owned()));
            }
        }
    }

    /// Authenticate specified block number for a MiFare classic card.  Uid
    /// should be a byte array with the UID of the card, block number should be
    /// the block to authenticate, key number should be the key type (like
    /// `MIFARE_CMD_AUTH_A` or `MIFARE_CMD_AUTH_B`), and key should be a byte array
    /// with the key data.  Returns True if the block was authenticated, or False
    /// if not authenticated.
    fn mifare_classic_authenticate_block(&self, uid: &[u8], block_number: u8, key_number: u8, key: &[u8]) -> Result<bool> {

        // Build parameters for InDataExchange command to authenticate MiFare card.
        let uid_len = uid.len();
        let key_len = key.len();
        let mut params = vec![0; 3 + uid_len + key_len];
        params[0] = 0x01; // Max card numbers
        params[1] = key_number & 0xFF;
        params[2] = block_number & 0xFF;
        params[3..3+key_len].copy_from_slice(key);
        params[3+key_len..].copy_from_slice(uid);

        // Send InDataExchange request and verify response is 0x00.
        let response = self.call_function(
            COMMAND_INDATAEXCHANGE,
            1,
            params.as_slice(),
            1.0,
        )?;

        self.check_response(response)
    }

    /// Read a block of data from the card.  Block number should be the block
    /// to read.  If the block is successfully read a bytearray of length 16 with
    /// data starting at the specified block will be returned.  If the block is
    /// not read then None will be returned.
    fn mifare_classic_read_block(&self, block_number: u8) -> Result<Vec<u8>> {

        // Send InDataExchange request to read block of MiFare data.
        let response = self.call_function(
            COMMAND_INDATAEXCHANGE,
            17,
            &[0x01, MIFARE_CMD_READ, block_number & 0xFF],
            1.0
        )?;

        if let Some(res) = response {
            // Check first response is 0x00 to show success.
            if res[0] != 0 {
                Err(box PN532Error::error(res[0]))
            } else {
                // Return first 4 bytes since 16 bytes are always returned.
                Ok(res[1..].into())
            }
        } else {
            Ok(vec![])
        }
    }

    /// Write a block of data to the card.  Block number should be the block
    /// to write and data should be a byte array of length 4 with the data to
    /// write.  If the data is successfully written then True is returned,
    /// otherwise False is returned.
    fn mifare_classic_write_block(&self, block_number: u8, data: &[u8]) -> Result<bool> {
        assert_eq!(data.len(), 16);

        let mut params = vec![0; 19];
        params[0] = 0x01;
        params[1] = MIFARE_CMD_WRITE;
        params[2] = block_number & 0xFF;
        params[3..].copy_from_slice(data);

        let response = self.call_function(
            COMMAND_INDATAEXCHANGE,
            1,
            params.as_slice(),
            1.0
        )?;

        self.check_response(response)
    }

    fn ntag2xx_write_block(&self, block_number: u8, data: &[u8]) -> Result<bool> {
        assert_eq!(data.len(), 4);

        let mut params = vec![0; 3+data.len()];
        params[0] = 0x01;
        params[1] = MIFARE_ULTRALIGHT_CMD_WRITE;
        params[2] = block_number & 0xFF;
        params[3..].copy_from_slice(data);

        let response = self.call_function(
            COMMAND_INDATAEXCHANGE,
            1,
            params.as_slice(),
            1.0
        )?;

        self.check_response(response)
    }
    
    fn ntag2xx_read_block(&self, block_number: u8) -> Result<Vec<u8>>{
        self.mifare_classic_read_block(block_number)
            .and_then(| res | {Ok(res[..4].to_owned())})
    }

    /// Read the state of the PN532's GPIO pins.
    /// If `pin` is None, returns 3 bytes containing the pin state as `(None, Vec<u7>)`
    /// where:
    /// ```
    /// P3[0] = P30,   P7[0] = 0,   I[0] = I0,
    /// P3[1] = P31,   P7[1] = P71, I[1] = I1,
    /// P3[2] = P32,   P7[2] = P72, I[2] = 0,
    /// P3[3] = P33,   P7[3] = 0,   I[3] = 0,
    /// P3[4] = P34,   P7[4] = 0,   I[4] = 0,
    /// P3[5] = P35,   P7[5] = 0,   I[5] = 0,
    /// P3[6] = 0,     P7[6] = 0,   I[6] = 0,
    /// P3[7] = 0,     P7[7] = 0,   I[7] = 0,
    /// ```
    /// If `pin` is not None, returns the specified pin state as `(Bool, None)`
    fn read_gpio(&self, pin: Option<PN532Gpio>) -> Result<(Option<bool>, Option<Vec<u8>>)> {
        let response = self.call_function(
            COMMAND_READGPIO,
            3,
            &[],
            1.0
        )?.unwrap();
        info!("GPIO Status: {:?}", response);

        match pin {
            Some(pin) => {
                Ok((Some(pin.get(response[pin.idx()])), None))
            }
            None => Ok((None, Some(response[..3].to_owned())))
        }
    }

    /// Write the state to the PN532's GPIO pins.
    /// If p3 or p7 is not `None`, set the pins with p3 or p7, there is
    /// no need to read pin states before write with the param p3 or p7
    /// bits:
    /// ```
    /// P3[0] = P30,   P7[0] = 0,
    /// P3[1] = P31,   P7[1] = P71,
    /// P3[2] = P32,   P7[2] = P72,
    /// P3[3] = P33,   P7[3] = nu,
    /// P3[4] = P34,   P7[4] = nu,
    /// P3[5] = P35,   P7[5] = nu,
    /// P3[6] = nu,    P7[6] = nu,
    /// P3[7] = Val,   P7[7] = Val,
    /// ```
    /// For each port that is validated (bit Val = 1), all the bits are applied
    /// simultaneously. It is not possible for example to modify the state of
    /// the port P32 without applying a value to the ports P30, P31, P33, P34
    /// and P35.
    ///
    /// If p3 and p7 are `None`, set one pin with the params 'pin' and 'state'
    fn write_gpio(&self, pin: PN532Gpio, state: bool, p3: Option<u8>, p7: Option<u8>) -> Result<()> {
        let mut params = [0x00; 2];
        if let (Some(p3), Some(p7)) = (p3, p7) {
            params[0] = if p3 == 0 { 0x00 } else { 0x80 | p3 & 0xFF };
            params[1] = if p7 == 0 { 0x00 } else { 0x80 | p7 & 0xFF };
            self.call_function(
                COMMAND_WRITEGPIO,
                1,
                &params,
                1.0
            ).map(||())
        } else {
            match pin {
                PN532Gpio::I0 | PN532Gpio::I1 => Ok(()),
                _ => {
                    let response = self.read_gpio(None)?.1.unwrap();
                    params[pin.idx()] = if state {
                        0x80 | response[pin.idx()] | (1 << pin.offset()) & 0xFF
                    } else {
                        0x80 | response[pin.idx()] & !(1 << pin.offset()) & 0xFF
                    };

                    self.call_function(
                        COMMAND_WRITEGPIO,
                        1,
                        &params,
                        1.0
                    )
                }
            }
        }
    }

    /// The host controller uses this command to configure the PN532 as
    /// target.
    /// :params mode: a byte indicating which mode the PN532 should respect.
    /// :params mifare_params: information needed to be able to be
    /// activated at 106 kbps in passive mode.
    /// :params felica_params: information to be able to respond to a polling
    /// request at 212/424 kbps in passive mode.
    /// :params nfcid3t: used in the ATR_RES in case of ATR_REQ received from
    /// the initiator
    /// :params gt: an array containing the general bytes to be used in the
    /// ATR_RES. This information is optional and the length is not fixed
    /// (max. 47 bytes),
    /// :params tk: an array containing the historical bytes to be used in the
    /// ATS when PN532 is in ISO/IEC14443-4 PICC emulation mode. This
    /// information is optional.
    /// :returns mode: a byte indicating in which mode the PN532 has been
    /// activated.
    /// :returns initiator_command: an array containing the first valid frame
    /// received by the PN532 once the PN532 has been initialized.
    fn tg_init_as_target(&self, mode: u8,
                         mifare_params: [u8; 6], felica_params: [u8; 18], nfcid3t: [u8; 10],
                         gt: Option<&[u8]>, tk: Option<&[u8]>, timeout: f64) -> Result<Option<(u8, Vec<u8>)>> {
        let mut params = Vec::new();
        params.push(mode);
        params.extend_from_slice(&mifare_params);
        params.extend_from_slice(&felica_params);
        params.extend_from_slice(&nfcid3t);
        let push_slice = | &mut params, slice: Option<&[u8]> | {
            if let Some(slice) = slice {
                params.push(slice.len() as u8);
                params.extend_from_slice(slice);
            } else { params.push(0x00) }
        };
        push_slice(&mut params, gt);
        push_slice(&mut params, tk);

        let response = self.call_function(
            COMMAND_TGINITASTARGET,
            64,
            params.as_slice(),
            timeout
        )?;
        // Try to read 64 bytes although the response length is not fixed
        if let Some(response) = response {
            Ok(Some((response[0], response[1..].to_owned())))
        } else {
            Ok(None)
        }
    }

    fn check_response(&self, response: Option<Vec<u8>>) -> Result<bool> {
        if let Some(res) = response {
            if res[0] != 0x00 {
                Err(box PN532Error::error(res[0]))
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }
}
