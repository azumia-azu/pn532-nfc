use std::intrinsics::bitreverse;
use std::thread;
use std::time::{Duration, Instant};
use rppal::spi::{Bus, SlaveSelect, Mode, Spi};
use rppal::gpio::Gpio;
use rppal::uart::Error::Gpio;
use crate::pn532::PN532;

const SPI_STATREAD: u8 =    0x02;
const SPI_DATAWRITE: u8 =   0x01;
const SPI_DATAREAD: u8 =    0x03;
const SPI_READY: u8 =       0x01;

struct PN532Spi {
    spi: Spi,
    gpio: Gpio,
}

impl PN532Spi {
    fn new(cs: u32) -> Self {
        let spi =
            Spi::new(Bus::Spi0, SlaveSelect::Ss0, cs, Mode::Mode2)?;
        let gpio = Gpio::new()?;
        Self {
            spi,
            gpio
        }
    }
}

impl PN532 for PN532Spi {
    fn gpio_init(&self) {
        todo!()
    }

    fn reset(&self, pin: u8) {
        let mut pin = self.gpio.get(pin)?.into_output_high();
        thread::sleep(Duration::from_millis(100));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
        pin.set_high();
        thread::sleep(Duration::from_millis(100));
    }

    fn read_data(&self, len: usize) -> Vec<u8> {
        todo!()
    }

    fn write_data(&self, frame: &[u8]) -> crate::pn532::Result<()> {
        todo!()
    }

    fn wait_ready(&self, timeout: f64) -> bool {
        let mut write_buf = [bitreverse(SPI_STATREAD), 0x00];
        let mut read_buf = [0; 2];
        let timestamp = Instant::now();
        while timestamp.elapsed() < timeout {
            thread::sleep(Duration::from_millis(10));

            self.spi.transfer(&mut read_buf,&write_buf)?;
            if bitreverse(read_buf[1]) == SPI_READY {
                return true;
            } else {
                thread::sleep(Duration::from_millis(5))
            }
            write_buf.copy_from_slice(&read_buf);
        }
        false
    }

    fn wake_up(&self) {
    }
}