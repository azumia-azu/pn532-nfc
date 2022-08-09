use std::thread;
use std::time::{Duration, Instant};
use log::debug;
use rppal::spi::{Bus, SlaveSelect, Mode, Spi};
use rppal::gpio::Gpio;
use crate::pn532::PN532;

const SPI_STATREAD: u8 =    0x02;
const SPI_DATAWRITE: u8 =   0x01;
const SPI_DATAREAD: u8 =    0x03;
const SPI_READY: u8 =       0x01;

struct SpiDevice {
    spi: Spi,
    gpio: Gpio,
    cs: Option<u8>,
}

impl SpiDevice {
    fn new(cs: Option<u8>) -> crate::pn532::Result<Self> {
        let spi =
            Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode2)?;
        let gpio = Gpio::new()?;

        let this = Self {
            spi,
            gpio,
            cs
        };

        if let Some(pin) = this.cs {
            this.gpio.get(pin)?.into_output_high();
        }

        Ok(this)
    }

    fn write(&mut self, buf: &[u8]) -> crate::pn532::Result<usize> {
        let cs = if let Some(pin) = self.cs {
            Some(self.gpio.get(pin)?.into_output_low())
        } else {
            None
        };
        thread::sleep(Duration::from_millis(1));
        let ret = self.spi.write(buf)?;

        cs.map(|mut pin| {
            thread::sleep(Duration::from_millis(1));
            pin.set_high();
            Some(pin)
        });

        Ok(ret)
    }

    fn read(&mut self, buf: &mut [u8]) -> crate::pn532::Result<usize> {
        let cs = if let Some(pin) = self.cs {
            Some(self.gpio.get(pin)?.into_output_low())
        } else {
            None
        };
        thread::sleep(Duration::from_millis(1));
        let ret = self.spi.read(buf)?;

        cs.map(|mut pin| {
            thread::sleep(Duration::from_millis(1));
            pin.set_high();
            Some(pin)
        });

        Ok(ret)
    }

    fn transfer(&mut self, read_buf: &mut [u8], write_buf: &[u8]) -> crate::pn532::Result<usize> {
        let cs = if let Some(pin) = self.cs {
            Some(self.gpio.get(pin)?.into_output_low())
        } else {
            None
        };
        thread::sleep(Duration::from_millis(1));
        let ret = self.spi.transfer(read_buf, write_buf)?;

        cs.map(|mut pin| {
            thread::sleep(Duration::from_millis(1));
            pin.set_high();
            Some(pin)
        });

        Ok(ret)
    }
}

struct PN532Spi {
    spi: SpiDevice,
    cs: Option<u8>,
    irq: Option<u8>,
    reset: Option<u8>
}

impl PN532Spi {
    fn new(cs: Option<u8>, irq: Option<u8>, reset: Option<u8>) -> crate::pn532::Result<Self> {
        let spi= SpiDevice::new(cs)?;
        let mut this = Self {
            spi,
            cs,
            irq,
            reset
        };

        this.gpio_init()?;
        this.init(reset)?;

        Ok(this)
    }
}

impl PN532 for PN532Spi {
    fn gpio_init(&self) -> crate::pn532::Result<()> {
        if let Some(pin) = self.reset {
            self.spi.gpio.get(pin)?.into_output_high();
        }
        if let Some(pin) = self.cs {
            self.spi.gpio.get(pin)?.into_output_high();
        }
        if let Some(pin) = self.irq {
            self.spi.gpio.get(pin)?.into_input();
        }
        Ok(())
    }

    fn reset(&self, pin: u8) -> crate::pn532::Result<()> {
        let mut pin = self.spi.gpio.get(pin)?.into_output_high();
        thread::sleep(Duration::from_millis(100));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
        pin.set_high();
        thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    fn read_data(&mut self, len: usize) -> crate::pn532::Result<Vec<u8>> {
        let mut write_buf = vec![0x00; len];
        let mut read_buf = vec![0x00; len];
        write_buf[0] = SPI_DATAREAD.reverse_bits();
        thread::sleep(Duration::from_millis(5));

        self.spi.transfer(read_buf.as_mut_slice(), &write_buf)?;

        let read_buf: Vec<u8> = read_buf.into_iter().map(u8::reverse_bits).collect();
        debug!("Reading: {:?}", read_buf);

        Ok(read_buf[1..].to_owned())
    }

    fn write_data(&mut self, frame: &[u8]) -> crate::pn532::Result<()> {
        let mut write_buf = vec![SPI_DATAWRITE];
        write_buf.extend_from_slice(frame);
        let write_buf: Vec<u8> = write_buf.into_iter().map(u8::reverse_bits).collect();
        debug!("Writing: {:?}", write_buf);
        thread::sleep(Duration::from_millis(20));

        self.spi.write(&write_buf).map(|_| ())
    }

    fn wait_ready(&mut self, timeout: f64) -> crate::pn532::Result<bool> {
        let mut write_buf = [SPI_STATREAD.reverse_bits(), 0x00];
        let mut read_buf = [0; 2];
        let timestamp = Instant::now();
        while timestamp.elapsed() < Duration::from_secs_f64(timeout) {
            thread::sleep(Duration::from_millis(10));

            self.spi.transfer(&mut read_buf,&write_buf)?;
            if read_buf[1].reverse_bits() == SPI_READY {
                return Ok(true);
            } else {
                thread::sleep(Duration::from_millis(5))
            }
            write_buf.copy_from_slice(&read_buf);
        }

        Ok(false)
    }

    fn wake_up(&mut self) -> crate::pn532::Result<()> {
        thread::sleep(Duration::from_secs(1));
        if let Some(pin) = self.cs {
            self.spi.gpio.get(pin)?.into_output_low();
        }
        thread::sleep(Duration::from_millis(2));
        self.spi.write(&[0x00])?;
        thread::sleep(Duration::from_secs(1));

        Ok(())
    }
}