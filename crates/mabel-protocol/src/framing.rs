//! Soundcore packet framing: header, command, body, checksum.
//!
//! Wire format:
//! ```text
//! [direction: 5 bytes] [command: 2 bytes] [length: 2 bytes LE] [body: N bytes] [checksum: 1 byte]
//! ```
//!
//! - Direction: Outbound `[0x08, 0xEE, 0x00, 0x00, 0x00]`, Inbound `[0x09, 0xFF, 0x00, 0x00, 0x01]`
//! - Command: 2 bytes identifying the operation (e.g. `[0x01, 0x01]` = state update)
//! - Length: total packet length as u16 LE (includes header + command + length + body + checksum)
//! - Checksum: sum of all preceding bytes mod 256

use nom::bytes::complete::take;
use nom::number::complete::le_u16;
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::{ProtocolError, Result};

/// Packet direction — who sent it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Host → Device
    Outbound,
    /// Device → Host
    Inbound,
}

impl Direction {
    /// Wire bytes for this direction.
    pub fn header_bytes(&self) -> &'static [u8; 5] {
        match self {
            Direction::Outbound => &[0x08, 0xEE, 0x00, 0x00, 0x00],
            Direction::Inbound => &[0x09, 0xFF, 0x00, 0x00, 0x01],
        }
    }

    /// Parse direction from 5 header bytes.
    fn from_header(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 5 {
            return None;
        }
        if bytes[0] == 0x08 {
            Some(Direction::Outbound)
        } else if bytes[0] == 0x09 {
            Some(Direction::Inbound)
        } else {
            None
        }
    }
}

/// A framed Soundcore protocol packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Packet {
    pub direction: Direction,
    pub command: [u8; 2],
    pub body: Vec<u8>,
}

/// Minimum packet size: 5 (direction) + 2 (command) + 2 (length) + 1 (checksum) = 10
const MIN_PACKET_SIZE: usize = 10;

impl Packet {
    /// Create a new outbound packet.
    pub fn outbound(command: [u8; 2], body: Vec<u8>) -> Self {
        Self {
            direction: Direction::Outbound,
            command,
            body,
        }
    }

    /// Create a new inbound packet (typically from parsing).
    pub fn inbound(command: [u8; 2], body: Vec<u8>) -> Self {
        Self {
            direction: Direction::Inbound,
            command,
            body,
        }
    }

    /// Serialize the packet into wire bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let total_len: u16 = (MIN_PACKET_SIZE + self.body.len()) as u16;
        let mut buf = Vec::with_capacity(total_len as usize);

        // Direction header (5 bytes)
        buf.extend_from_slice(self.direction.header_bytes());
        // Command (2 bytes)
        buf.extend_from_slice(&self.command);
        // Length (2 bytes LE)
        buf.extend_from_slice(&total_len.to_le_bytes());
        // Body
        buf.extend_from_slice(&self.body);
        // Checksum: sum of all preceding bytes mod 256
        let checksum = compute_checksum(&buf);
        buf.push(checksum);

        buf
    }

    /// Parse a complete packet from a byte slice.
    /// Returns the parsed Packet or a ProtocolError.
    pub fn parse(input: &[u8]) -> Result<Self> {
        match parse_packet(input) {
            Ok((_, packet)) => Ok(packet),
            Err(nom::Err::Incomplete(_)) => Err(ProtocolError::Incomplete),
            Err(e) => Err(ProtocolError::InvalidPacket(format!("{}", e))),
        }
    }

    /// Try to parse a packet from the front of a buffer using nom.
    /// Returns (remaining_bytes, Packet) on success.
    pub fn parse_from_stream(input: &[u8]) -> Result<(&[u8], Self)> {
        match parse_packet(input) {
            Ok((remaining, packet)) => Ok((remaining, packet)),
            Err(nom::Err::Incomplete(_)) => Err(ProtocolError::Incomplete),
            Err(e) => Err(ProtocolError::InvalidPacket(format!("{}", e))),
        }
    }
}

/// Compute checksum: sum of all bytes mod 256.
fn compute_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

/// nom parser: parse one complete packet.
fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    // Direction header: 5 bytes
    let (input, dir_bytes) = take(5usize)(input)?;
    let direction = Direction::from_header(dir_bytes).ok_or(nom::Err::Failure(
        nom::error::Error::new(input, nom::error::ErrorKind::Tag),
    ))?;

    // Command: 2 bytes
    let (input, cmd_bytes) = take(2usize)(input)?;
    let command: [u8; 2] = [cmd_bytes[0], cmd_bytes[1]];

    // Length: 2 bytes LE (total packet length including everything)
    let (input, total_length) = le_u16(input)?;

    // Body length = total_length - 5 (dir) - 2 (cmd) - 2 (len) - 1 (checksum)
    let body_len = (total_length as usize).saturating_sub(MIN_PACKET_SIZE);

    // Body
    let (input, body_bytes) = take(body_len)(input)?;

    // Checksum: 1 byte
    let (input, checksum_bytes) = take(1usize)(input)?;
    let received_checksum = checksum_bytes[0];

    // Verify checksum: sum of dir + cmd + len + body
    let mut check_buf = Vec::with_capacity(total_length as usize - 1);
    check_buf.extend_from_slice(direction.header_bytes());
    check_buf.extend_from_slice(&command);
    check_buf.extend_from_slice(&total_length.to_le_bytes());
    check_buf.extend_from_slice(body_bytes);
    let expected_checksum = compute_checksum(&check_buf);

    if received_checksum != expected_checksum {
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    Ok((
        input,
        Packet {
            direction,
            command,
            body: body_bytes.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_empty_body() {
        let pkt = Packet::outbound([0x01, 0x01], vec![]);
        let bytes = pkt.to_bytes();
        let parsed = Packet::parse(&bytes).unwrap();
        assert_eq!(pkt, parsed);
    }

    #[test]
    fn test_roundtrip_with_body() {
        let pkt = Packet::outbound([0x08, 0x06], vec![0x01, 0x02, 0x03, 0x04]);
        let bytes = pkt.to_bytes();
        let parsed = Packet::parse(&bytes).unwrap();
        assert_eq!(pkt, parsed);
    }

    #[test]
    fn test_roundtrip_inbound() {
        let pkt = Packet::inbound([0x01, 0x01], vec![0xAA, 0xBB, 0xCC]);
        let bytes = pkt.to_bytes();
        let parsed = Packet::parse(&bytes).unwrap();
        assert_eq!(pkt, parsed);
    }

    #[test]
    fn test_checksum_calculation() {
        let data = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0A, 0x00];
        let checksum = compute_checksum(&data);
        // 0x08 + 0xEE + 0x00 + 0x00 + 0x00 + 0x01 + 0x01 + 0x0A + 0x00 = 0x02 (mod 256)
        assert_eq!(checksum, 0x02);
    }

    #[test]
    fn test_invalid_direction_fails() {
        let mut bytes = Packet::outbound([0x01, 0x01], vec![]).to_bytes();
        bytes[0] = 0xFF; // corrupt direction
        assert!(Packet::parse(&bytes).is_err());
    }

    #[test]
    fn test_corrupted_checksum_fails() {
        let mut bytes = Packet::outbound([0x01, 0x01], vec![0xDE, 0xAD]).to_bytes();
        let last = bytes.len() - 1;
        bytes[last] = bytes[last].wrapping_add(1); // corrupt checksum
        assert!(Packet::parse(&bytes).is_err());
    }

    #[test]
    fn test_min_packet_size() {
        let pkt = Packet::outbound([0x01, 0x01], vec![]);
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), MIN_PACKET_SIZE);
    }

    #[test]
    fn test_parse_from_stream_with_trailing_data() {
        let pkt = Packet::outbound([0x01, 0x01], vec![0xAA]);
        let mut bytes = pkt.to_bytes();
        bytes.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // trailing garbage

        let (remaining, parsed) = Packet::parse_from_stream(&bytes).unwrap();
        assert_eq!(parsed, pkt);
        assert_eq!(remaining, &[0xDE, 0xAD, 0xBE, 0xEF]);
    }
}
