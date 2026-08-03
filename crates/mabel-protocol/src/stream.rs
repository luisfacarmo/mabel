//! Streaming packet parser — accumulates bytes and extracts complete packets.
//!
//! Handles the case where RFCOMM delivers data in chunks that don't align
//! with packet boundaries. Internally buffers and attempts to parse whenever
//! new data arrives.

use crate::error::ProtocolError;
use crate::framing::Packet;

/// A streaming buffer that accumulates incoming bytes and yields complete packets.
pub struct PacketStream {
    buffer: Vec<u8>,
}

impl PacketStream {
    /// Create a new empty stream.
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(256),
        }
    }

    /// Append raw bytes to the internal buffer.
    pub fn push(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Try to extract one complete packet from the front of the buffer.
    /// Returns `Some(Packet)` if a full packet was available, `None` if more data is needed.
    /// Corrupted data (invalid direction/checksum) is discarded by advancing 1 byte and retrying.
    pub fn next_packet(&mut self) -> Option<Packet> {
        loop {
            if self.buffer.len() < 10 {
                return None;
            }

            // Only peek at length if the first byte looks like a valid direction header.
            let first = self.buffer[0];
            if (first == 0x08 || first == 0x09) && self.buffer.len() >= 9 {
                let declared_len = u16::from_le_bytes([self.buffer[7], self.buffer[8]]) as usize;
                if declared_len >= 10 && self.buffer.len() < declared_len {
                    // Valid-looking header but not enough data yet.
                    return None;
                }
            }

            match Packet::parse_from_stream(&self.buffer) {
                Ok((remaining, packet)) => {
                    let consumed = self.buffer.len() - remaining.len();
                    self.buffer.drain(..consumed);
                    return Some(packet);
                }
                Err(ProtocolError::Incomplete) => {
                    return None;
                }
                Err(_) => {
                    // Invalid packet at current position — skip 1 byte and retry.
                    self.buffer.drain(..1);
                }
            }
        }
    }

    /// Number of buffered bytes not yet consumed.
    pub fn buffered(&self) -> usize {
        self.buffer.len()
    }

    /// Clear the internal buffer (e.g. on reconnect).
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for PacketStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_packet_in_one_push() {
        let pkt = Packet::outbound([0x01, 0x01], vec![0xAA, 0xBB]);
        let mut stream = PacketStream::new();
        stream.push(&pkt.to_bytes());
        assert_eq!(stream.next_packet(), Some(pkt));
        assert_eq!(stream.buffered(), 0);
    }

    #[test]
    fn test_fragmented_delivery() {
        let pkt = Packet::outbound([0x01, 0x01], vec![0x11, 0x22, 0x33, 0x44, 0x55]);
        let bytes = pkt.to_bytes();

        let mut stream = PacketStream::new();
        stream.push(&bytes[..5]);
        assert_eq!(stream.next_packet(), None);
        stream.push(&bytes[5..]);
        assert_eq!(stream.next_packet(), Some(pkt));
        assert_eq!(stream.buffered(), 0);
    }

    #[test]
    fn test_two_packets_concatenated() {
        let pkt1 = Packet::inbound([0x01, 0x01], vec![0xAA]);
        let pkt2 = Packet::inbound([0x06, 0x01], vec![0xBB, 0xCC]);

        let mut bytes = pkt1.to_bytes();
        bytes.extend_from_slice(&pkt2.to_bytes());

        let mut stream = PacketStream::new();
        stream.push(&bytes);
        assert_eq!(stream.next_packet(), Some(pkt1));
        assert_eq!(stream.next_packet(), Some(pkt2));
        assert_eq!(stream.next_packet(), None);
    }

    #[test]
    fn test_garbage_prefix_is_skipped() {
        let pkt = Packet::inbound([0x01, 0x01], vec![0xDD]);
        let mut bytes = vec![0xFF, 0xFE, 0xFD]; // garbage
        bytes.extend_from_slice(&pkt.to_bytes());

        let mut stream = PacketStream::new();
        stream.push(&bytes);
        assert_eq!(stream.next_packet(), Some(pkt));
    }

    #[test]
    fn test_byte_by_byte_delivery() {
        let pkt = Packet::outbound([0x02, 0x01], vec![0x01, 0x02]);
        let bytes = pkt.to_bytes();

        let mut stream = PacketStream::new();
        for (i, &b) in bytes.iter().enumerate() {
            stream.push(&[b]);
            if i < bytes.len() - 1 {
                assert_eq!(stream.next_packet(), None);
            }
        }
        assert_eq!(stream.next_packet(), Some(pkt));
    }

    #[test]
    fn test_clear_resets_buffer() {
        let mut stream = PacketStream::new();
        stream.push(&[0x08, 0xEE, 0x00, 0x00, 0x00]);
        assert_eq!(stream.buffered(), 5);
        stream.clear();
        assert_eq!(stream.buffered(), 0);
    }

    #[test]
    fn test_empty_stream_returns_none() {
        let mut stream = PacketStream::new();
        assert_eq!(stream.next_packet(), None);
    }
}
