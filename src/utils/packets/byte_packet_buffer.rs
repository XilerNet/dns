use std::fmt::format;
use std::fs::read;

use crate::utils::bitwise::{has_flag, JUMP_FLAG, merge_two_numbers_as, merge_u16_as_u32, merge_u8_as_u16};
use crate::utils::common::Result;

const BUFFER_MAX_SIZE: usize = 512;
const MAX_JUMP_COUNT: usize = 5;

/// Hold track of the packet contents and where we are
pub struct BytePacketBuffer {
    pub buf: [u8; BUFFER_MAX_SIZE],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; BUFFER_MAX_SIZE],
            pos: 0,
        }
    }

    /// Current position in the buffer
    fn pos(&self) -> usize {
        self.pos
    }

    /// Validate if a requested position is valid
    fn is_valid_pos(&self, pos: usize) -> bool {
        pos > 0 && pos <= BUFFER_MAX_SIZE
    }

    /// Validate whether a requested position is valid and
    /// return this in a result format.
    fn validate_position(&self, pos: usize) -> Result<()> {
        if self.is_valid_pos(pos) {
            Ok(())
        }

        Err("Invalid position (maybe the end of the buffer has been reached?)".into())
    }

    /// Step the buffer position forward with a specific number steps
    fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    /// Get a single byte from the buffer without advancing the position
    fn get(&mut self, pos: usize) -> Result<u8> {
        self.validate_position(pos)?;
        Ok(self.buf[self.pos])
    }

    /// Get a range of bytes
    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        let end = start + len;
        self.validate_position(end)?;
        Ok(&self.buf[start..end])
    }

    /// Read a single byte and step forward
    fn read(&mut self) -> Result<u8> {
        let res = self.get(self.pos)?;
        self.step(1)?;
        Ok(res)
    }

    /// Read two bytes, stepping two steps forward
    fn read_u16(&mut self) -> Result<u16> {
        Ok(merge_u8_as_u16(self.read()?, self.read()?))
    }

    /// Read four bytes, stepping four steps forward
    fn read_u32(&mut self) -> Result<u32> {
        Ok(merge_u16_as_u32(self.read_u16()?, self.read_u16()?))
    }

    /// Reads a domain name into the outstr buffer, returning the domain name taking labels into consideration.
    fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
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
}