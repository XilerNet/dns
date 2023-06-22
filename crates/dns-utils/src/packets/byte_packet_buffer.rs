use shared::prelude::*;

use crate::bitwise::{has_flag, JUMP_FLAG, merge_u16_as_u32, merge_u8_as_u16, split_32_as_u8s, split_u16_as_u8s};

const BUFFER_MAX_SIZE: usize = 512;
const MAX_JUMP_COUNT: usize = 5;

/// Hold track of the packet contents and where we are
pub struct BytePacketBuffer {
    pub buf: [u8; BUFFER_MAX_SIZE],
    pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; BUFFER_MAX_SIZE],
            pos: 0,
        }
    }

    /// Current position in the buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Validate if a requested position is valid
    pub fn is_valid_pos(&self, pos: usize) -> bool {
        pos <= BUFFER_MAX_SIZE
    }

    /// Validate whether a requested position is valid and
    /// return this in a result format.
    fn validate_position(&self, pos: usize) -> Result<()> {
        if self.is_valid_pos(pos) {
            return Ok(());
        }

        Err("Invalid position (maybe the end of the buffer has been reached?)".into())
    }

    /// Step the buffer position forward with a specific number steps
    pub fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    /// Get a single byte from the buffer without advancing the position
    pub fn get(&mut self, pos: usize) -> Result<u8> {
        self.validate_position(pos)?;
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        let end = start + len;
        self.validate_position(end)?;
        Ok(&self.buf[start..end])
    }

    /// Read a single byte and step forward
    pub fn read(&mut self) -> Result<u8> {
        let res = self.get(self.pos)?;
        self.step(1)?;
        Ok(res)
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(merge_u8_as_u16(self.read()?, self.read()?))
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(merge_u16_as_u32(self.read_u16()?, self.read_u16()?))
    }

    /// Reads a domain name into the outstr buffer, returning the domain name taking labels into consideration.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut pos = self.pos();

        let mut jumped = false;
        let mut jumps_performed = 0;

        let mut delim = "";  // empty because we don't want our first iteration to start with a delimiter (.)

        loop {
            // Prevent DNS cycle attacks
            if jumps_performed > MAX_JUMP_COUNT {
                return Err(format!("Exceeded limit of allowed jumps. ({})", MAX_JUMP_COUNT).into());
            }

            let len = self.get(pos)?;

            if has_flag(len, JUMP_FLAG) {
                // Update buffer position
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let offset = merge_u8_as_u16(len ^ 0xC0, self.get(pos + 1)?);
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;
                continue;
            } else {
                pos += 1;

                // Domain name is terminated
                if len == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";
                pos += len as usize; // move length of label forward
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }

    // Write a single byte to the buffer
    fn write(&mut self, val: u8) -> Result<()> {
        if self.pos >= 512 {
            return Err("End of buffer reached".into());
        }

        self.buf[self.pos] = val;
        self.pos += 1;

        Ok(())
    }

    // Write a single byte to the buffer
    fn write_u8(&mut self, val: u8) -> Result<()> {
        self.write(val)
    }

    // Write a slice of bytes to the buffer
    fn write_u8_slice(&mut self, val: &[u8]) -> Result<()> {
        val.into_iter().try_for_each(|byte| self.write_u8(*byte))
    }

    // Write a u16 to the buffer (converts to two u8s)
    fn write_u16(&mut self, val: u16) -> Result<()> {
        self.write_u8_slice(&split_u16_as_u8s(val))
    }

    // Write a u32 to the buffer (converts to four u8s)
    fn write_u32(&mut self, val: u32) -> Result<()> {
        self.write_u8_slice(&split_32_as_u8s(val))
    }

    // Write a domain name to the buffer
    fn write_qname(&mut self, qname: &str) -> Result<()> {
        for label in qname.split('.') {
            let len = label.len();

            if len > 0x3F {
                return Err("Single label exceeds maximum length of 63 characters".into());
            }

            self.write_u8(len as u8)?;
            self.write_u8_slice(label.as_bytes())?;
        }

        self.write_u8(0x00)?;
        Ok(())
    }
}