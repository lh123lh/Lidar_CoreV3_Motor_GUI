use crate::ymodem::xymodem_util::*;
use log::warn;
use std::io::{Read, Write};

// TODO: Send CAN byte after too many errors
// TODO: Handle CAN bytes while sending
// TODO: Implement Error for Error

pub const SOH: u8 = 0x01;
pub const STX: u8 = 0x02;
pub const EOT: u8 = 0x04;
pub const ACK: u8 = 0x06;
#[allow(dead_code)]
pub const NAK: u8 = 0x15;
pub const CAN: u8 = 0x18;
pub const CRC: u8 = 0x43;

pub type Result<T> = std::result::Result<T, Error>;

/// Configuration for the YMODEM transfer.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Ymodem {
    /// The number of errors that can occur before the communication is
    /// considered a failure. Errors include unexpected bytes and timeouts waiting for bytes.
    pub max_errors: u32,

    /// The number of errors that can occur before the communication is
    /// considered a failure. Errors include unexpected bytes and timeouts waiting for bytes.
    ///
    /// This only applies to the initial packet
    pub max_initial_errors: u32,

    /// The byte used to pad the last block. YMODEM can only send blocks of a certain size,
    /// so if the message is not a multiple of that size the last block needs to be padded.
    pub pad_byte: u8,

    /// Ignores all non-digit characters on the file_size string
    /// in the start frame (Ex. 12345V becomes 12345)
    pub ignore_non_digits_on_file_size: bool,

    errors: u32,
    initial_errors: u32,
}

impl Ymodem {
    /// Creates the YMODEM config with default parameters.
    pub fn new() -> Self {
        // Ymodem doesn't support 128 byte packages
        // or regular checksum
        Ymodem {
            max_errors: 20,
            max_initial_errors: 16,
            pad_byte: 0x1a,
            errors: 0,
            initial_errors: 0,
            ignore_non_digits_on_file_size: false,
        }
    }

    /// Receive an YMODEM transmission.
    ///
    /// `dev` should be the serial communication channel (e.g. the serial device).
    /// The received data will be written to `outstream`.
    /// `checksum` indicates which checksum mode should be used; Checksum::Standard is
    /// a reasonable default.
    ///
    /// # Timeouts
    /// This method has no way of setting the timeout of `dev`, so it's up to the caller
    /// to set the timeout of the device before calling this method. Timeouts on receiving
    /// bytes will be counted against `max_errors`, but timeouts on transmitting bytes
    /// will be considered a fatal error.
    #[allow(dead_code)]
    pub fn recv<D: Read + Write, W: Write>(
        &mut self,
        dev: &mut D,
        outstream: &mut W,
        file_name: &mut String,
        file_size: &mut u32,
    ) -> Result<()> {
        let mut file_buf: Vec<u8> = Vec::new();

        self.errors = 0;
        dbg!("Starting YMODEM receive");
        // Initialize transfer
        loop {
            (dev.write(&[CRC])?);

            match get_byte_timeout(dev) {
                Ok(v) => {
                    // The first SOH is used to initialize the transfer
                    if v == Some(SOH) {
                        break;
                    }
                }
                Err(_err) => {
                    self.initial_errors += 1;
                    if self.initial_errors > self.max_initial_errors {
                        eprint!(
                            "Exhausted max retries ({}) while waiting for SOH or STX",
                            self.max_initial_errors
                        );
                        return Err(Error::ExhaustedRetries);
                    }
                }
            }
        }
        // First packet
        // In YModem the header packet is 0
        let mut packet_num: u8 = 0;
        let mut file_name_buf: Vec<u8> = Vec::new();
        let mut file_size_buf: Vec<u8> = Vec::new();
        let mut padding_buf: Vec<u8> = Vec::new();

        loop {
            let pnum = (get_byte(dev))?; // specified packet number
            let pnum_1c = (get_byte(dev))?; // same, 1's complemented
                                            // We'll respond with cancel later if the packet number is wrong
            let cancel_packet = packet_num != pnum || (255 - pnum) != pnum_1c;

            loop {
                let b = get_byte(dev)?;
                file_name_buf.push(b);
                if b == 0x00 {
                    break;
                };
            }
            *file_name = String::from(
                std::str::from_utf8(&file_name_buf[0..file_name_buf.len() - 1]).unwrap(),
            );

            loop {
                let b = get_byte(dev)?;
                file_size_buf.push(b);
                if b == 0x00 {
                    break;
                };
            }

            // We read the padding
            // The 2 is the 2 zeroes
            for _ in 0..(128 - file_name_buf.len() - file_size_buf.len()) {
                padding_buf.push(get_byte(dev)?);
            }

            let recv_checksum = (((get_byte(dev))? as u16) << 8) + (get_byte(dev))? as u16;

            let mut data_buf: Vec<u8> = Vec::new();
            data_buf.extend(&file_name_buf);
            data_buf.extend(&file_size_buf);
            data_buf.extend(&padding_buf);

            let success = calc_crc(&mut data_buf) == recv_checksum;

            if cancel_packet {
                (dev.write(&[CAN]))?;
                (dev.write(&[CAN]))?;
                return Err(Error::Canceled);
            }
            if !success {
                (dev.write(&[NAK]))?;
                self.errors += 1;
            } else {
                // First packet received succesfully
                packet_num = packet_num.wrapping_add(1);
                (dev.write(&[ACK]))?;
                (dev.write(&[CRC]))?;
                break;
            }
        }

        let mut file_size_str =
            std::string::String::from_utf8(file_size_buf[0..file_size_buf.len() - 1].to_vec())
                .unwrap();
        if self.ignore_non_digits_on_file_size {
            file_size_str = file_size_str.chars().filter(|c| c.is_digit(10)).collect();
        }

        let file_size_num: u32 = match file_size_str.parse::<u32>() {
            Ok(v) => v,
            // If the first parse fails, we try everything before the space
            // if that fails too, then we panic
            _ => file_size_str
                .split(" ")
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        };
        *file_size = file_size_num;

        let num_of_packets = (file_size_num as f32 / 1024.0).ceil() as u32;
        let final_packet = num_of_packets + 2;
        let mut received_first_eot = false;

        for range in 0..(num_of_packets + 3) {
            dbg!(range);
            match get_byte_timeout(dev)? {
                bt @ Some(SOH) | bt @ Some(STX) => {
                    // Handle next packet
                    let packet_size = match bt {
                        Some(SOH) => 128,
                        Some(STX) => 1024,
                        _ => 0, // Why does the compiler need this?
                    };
                    let pnum = (get_byte(dev))?; // specified packet number
                    let pnum_1c = (get_byte(dev))?; // same, 1's complemented
                                                    // We'll respond with cancel later if the packet number is wrong

                    let cancel_packet = match range {
                        // Final packet num is 0
                        cp if cp == final_packet => 0x00 != pnum || (255 - pnum) != pnum_1c,
                        _ => packet_num != pnum || (255 - pnum) != pnum_1c,
                    };
                    let mut data: Vec<u8> = Vec::new();
                    data.resize(packet_size, 0);
                    (dev.read_exact(&mut data))?;
                    let recv_checksum = (((get_byte(dev))? as u16) << 8) + (get_byte(dev))? as u16;
                    let success = calc_crc(&data) == recv_checksum;

                    if cancel_packet {
                        (dev.write(&[CAN]))?;
                        (dev.write(&[CAN]))?;
                        return Err(Error::Canceled);
                    }
                    if success {
                        packet_num = packet_num.wrapping_add(1);
                        (dev.write(&[ACK]))?;
                        (file_buf.write_all(&data))?;
                    } else {
                        (dev.write(&[NAK]))?;
                        self.errors += 1;
                    }
                }
                Some(EOT) => {
                    packet_num = packet_num.wrapping_add(1);
                    // End of file
                    if !received_first_eot {
                        (dev.write(&[NAK]))?;
                        received_first_eot = true;
                    } else {
                        (dev.write(&[ACK]))?;
                        (dev.write(&[CRC]))?;
                    }
                }
                Some(_) => {
                    warn!("Unrecognized symbol!");
                }
                None => {
                    self.errors += 1;
                    warn!("Timeout!")
                }
            }
            if self.errors >= self.max_errors {
                eprint!(
                    "Exhausted max retries ({}) while waiting for ACK for EOT",
                    self.max_errors
                );
                return Err(Error::ExhaustedRetries);
            }
        }

        outstream
            .write_all(&file_buf[0..file_size_num as usize])
            .unwrap();
        Ok(())
    }

    /// Starts the YMODEM transmission.
    ///
    /// `dev` should be the serial communication channel (e.g. the serial device).
    /// `stream` should be the message to send (e.g. a file).
    ///
    /// # Timeouts
    /// This method has no way of setting the timeout of `dev`, so it's up to the caller
    /// to set the timeout of the device before calling this method. Timeouts on receiving
    /// bytes will be counted against `max_errors`, but timeouts on transmitting bytes
    /// will be considered a fatal error.
    pub fn send<D: Read + Write, R: Read>(
        &mut self,
        dev: &mut D,
        stream: &mut R,
        file_name: String,
        file_size_in_bytes: u64,
    ) -> Result<()> {
        self.errors = 0;
        let packets_to_send = f64::ceil(file_size_in_bytes as f64 / 1024.0) as u32;
        let last_packet_size = file_size_in_bytes % 1024;

        dbg!("Starting YMODEM transfer");
        (self.start_send(dev))?;
        dbg!("First byte received. Sending start frame.");
        (self.send_start_frame(dev, file_name, file_size_in_bytes))?;
        dbg!("Start frame acknowledged. Sending stream.");
        (self.send_stream(dev, stream, packets_to_send, last_packet_size))?;
        dbg!("Sending EOT");
        (self.finish_send(dev))?;

        Ok(())
    }

    fn start_send<D: Read + Write>(&mut self, dev: &mut D) -> Result<()> {
        let mut cancels = 0u32;
        loop {
            match (get_byte_timeout(dev))? {
                Some(c) => match c {
                    CRC => {
                        dbg!("16-bit CRC requested");
                        return Ok(());
                    }
                    CAN => {
                        warn!("Cancel (CAN) byte received");
                        cancels += 1;
                    }
                    c => warn!("Unknown byte received at start of YMODEM transfer: {}", c),
                },
                None => warn!("Timed out waiting for start of YMODEM transfer."),
            }

            self.errors += 1;

            if cancels >= 2 {
                eprint!(
                    "Transmission canceled: received two cancel (CAN) bytes \
                        at start of YMODEM transfer"
                );
                return Err(Error::Canceled);
            }

            if self.errors >= self.max_errors {
                eprint!(
                    "Exhausted max retries ({}) at start of YMODEM transfer.",
                    self.max_errors
                );
                if let Err(err) = dev.write_all(&[CAN]) {
                    warn!("Error sending CAN byte: {}", err);
                }
                return Err(Error::ExhaustedRetries);
            }
        }
    }

    fn send_start_frame<D: Read + Write>(
        &mut self,
        dev: &mut D,
        file_name: String,
        file_size_in_bytes: u64,
    ) -> Result<()> {
        let mut buff = vec![0x00; 128 as usize + 3];
        buff[0] = SOH;
        buff[1] = 0x00;
        buff[2] = 0xFF;

        let mut curr_buff_idx = 3;
        for byte in file_name.as_bytes() {
            buff[curr_buff_idx] = *byte;
            curr_buff_idx += 1;
        }

        // We leave one 0 to indicate the name ends here
        curr_buff_idx += 1;

        for byte in file_size_in_bytes.to_string().as_bytes() {
            buff[curr_buff_idx] = *byte;
            curr_buff_idx += 1;
        }

        let crc = calc_crc(&buff[3..]);
        buff.push(((crc >> 8) & 0xFF) as u8);
        buff.push((crc & 0xFF) as u8);

        (dev.write_all(&buff))?;

        loop {
            match (get_byte_timeout(dev))? {
                Some(c) => {
                    if c == ACK {
                        dbg!("Received ACK for start frame");
                        break;
                    } else {
                        warn!("Expected ACK, got {}", c);
                    }
                    // TODO handle CAN bytes
                }
                None => warn!("Timeout waiting for ACK for start frame"),
            }

            self.errors += 1;
            if self.errors >= self.max_errors {
                eprint!(
                    "Exhausted max retries ({}) while sending start frame in YMODEM transfer",
                    self.max_errors
                );
                return Err(Error::ExhaustedRetries);
            }
        }

        loop {
            match (get_byte_timeout(dev))? {
                Some(c) => {
                    if c == CRC {
                        dbg!("Received C for start frame");
                        break;
                    } else {
                        warn!("Expected C, got {}", c);
                    }
                    // TODO handle CAN bytes
                }
                None => warn!("Timeout waiting for C for start frame"),
            }

            self.errors += 1;
            if self.errors >= self.max_errors {
                eprint!(
                    "Exhausted max retries ({}) while sending start frame in YMODEM transfer",
                    self.max_errors
                );
                return Err(Error::ExhaustedRetries);
            }
        }

        return Ok(());
    }

    fn send_stream<D: Read + Write, R: Read>(
        &mut self,
        dev: &mut D,
        stream: &mut R,
        packets_to_send: u32,
        last_packet_size: u64,
    ) -> Result<()> {
        let mut block_num = 0u32;
        loop {
            let packet_size = if block_num + 1 == packets_to_send && last_packet_size <= 128 {
                128
            } else {
                1024
            };
            let mut buff = vec![self.pad_byte; packet_size as usize + 3];
            let n = (stream.read(&mut buff[3..]))?;
            if n == 0 {
                dbg!("Reached EOF");
                return Ok(());
            }

            block_num += 1;
            if packet_size == 128 {
                buff[0] = SOH;
            } else {
                buff[0] = STX;
            }
            buff[1] = (block_num & 0xFF) as u8;
            buff[2] = 0xFF - buff[1];

            let crc = calc_crc(&buff[3..]);
            buff.push(((crc >> 8) & 0xFF) as u8);
            buff.push((crc & 0xFF) as u8);

            println!("Sending block {}", block_num);
            (dev.write_all(&buff))?;

            match (get_byte_timeout(dev))? {
                Some(c) => {
                    if c == ACK {
                        dbg!("Received ACK for block {}", block_num);
                        continue;
                    } else {
                        warn!("Expected ACK, got {}", c);
                    }
                    // TODO handle CAN bytes
                }
                None => warn!("Timeout waiting for ACK for block {}", block_num),
            }

            self.errors += 1;

            if self.errors >= self.max_errors {
                eprint!(
                    "Exhausted max retries ({}) while sending block {} in YMODEM transfer",
                    self.max_errors, block_num
                );
                return Err(Error::ExhaustedRetries);
            }
        }
    }

    fn finish_send<D: Read + Write>(&mut self, dev: &mut D) -> Result<()> {
        (dev.write_all(&[EOT]))?;

        match (get_byte_timeout(dev))? {
            Some(c) => {
                if c == ACK {
                    println!("Got Ack");
                } else {
                    log::warn!("Expected ACK, got {}", c);
                }
            }
            None => warn!("Timeout waiting for ACK for EOT"),
        }

        match (get_stream_timeout(dev))? {
            Some(s) => {
                println!("{}", String::from_utf8(s).unwrap().trim());
            }
            None => warn!("Timeout waiting for stream"),
        }

        std::thread::sleep(std::time::Duration::from_millis(3000));

        self.send_end_frame(dev)?;

        Ok(())
    }

    fn send_end_frame<D: Read + Write>(&mut self, dev: &mut D) -> Result<()> {
        let mut buff = vec![0x00; 128 as usize + 3];
        buff[0] = SOH;
        buff[1] = 0x00;
        buff[2] = 0xFF;

        let crc = calc_crc(&buff[3..]);
        buff.push(((crc >> 8) & 0xFF) as u8);
        buff.push((crc & 0xFF) as u8);

        (dev.write_all(&buff))?;

        match (get_byte_timeout(dev))? {
            Some(c) => {
                if c == ACK {
                    dbg!("Received ACK for end frame");
                    // break;
                } else {
                    warn!("Expected ACK, got {}", c);
                }
                // TODO handle CAN bytes
            }
            None => warn!("Timeout waiting for ACK for start frame"),
        }

        self.errors += 1;
        if self.errors >= self.max_errors {
            eprint!(
                "Exhausted max retries ({}) while sending end frame in YMODEM transfer",
                self.max_errors
            );
            return Err(Error::ExhaustedRetries);
        }

        match (get_stream_timeout(dev))? {
            Some(s) => {
                println!("{}", String::from_utf8_lossy(&s).trim());
            }
            None => warn!("Timeout waiting for stream"),
        }

        return Ok(());
    }
}
