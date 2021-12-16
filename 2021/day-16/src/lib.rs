#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

use bitreader::BitReader;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("incomplete packet bits")]
    BitsError(#[from] bitreader::BitReaderError),
    #[error("invalid number of arguments `{0}` for operation `{1:?}`")]
    ArgumentError(usize, Operation),
    #[error("invalid operator ID `{0}`")]
    OperatorError(u8),
}

enum LengthKind {
    TotalBits(u64),
    PacketCount(usize),
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    // Expecting a fifth operator here
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for Operation {
    type Error = PacketError;
    fn try_from(op_id: u8) -> Result<Self, Self::Error> {
        match op_id {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            // No OP with ID 4
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(PacketError::OperatorError(op_id)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PacketKind {
    Literal(usize),
    Operator {
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub version: u8,
    pub length: u64,
    pub kind: PacketKind,
}

impl Packet {
    /// Evaluates operator packets recursively
    ///
    /// # Errors
    ///
    /// Will return `Err` if any operators have an invalid number of arguments.
    pub fn eval(&self) -> Result<usize, PacketError> {
        Ok(match &self.kind {
            PacketKind::Literal(value) => *value,
            PacketKind::Operator { operation, packets } => {
                let packets: Vec<usize> =
                    packets.iter().map(Self::eval).collect::<Result<_, _>>()?;
                match operation {
                    Operation::Sum => packets.iter().sum(),
                    Operation::Product => packets.iter().product(),
                    Operation::Minimum | Operation::Maximum => *{
                        match operation {
                            Operation::Minimum => packets.iter().min(),
                            Operation::Maximum => packets.iter().max(),
                            _ => unreachable!(),
                        }
                    }
                    .ok_or_else(|| PacketError::ArgumentError(packets.len(), *operation))?,
                    Operation::LessThan | Operation::GreaterThan | Operation::EqualTo => {
                        if let [a, b] = &packets[..] {
                            Ok(match operation {
                                Operation::LessThan => a < b,
                                Operation::GreaterThan => a > b,
                                Operation::EqualTo => a == b,
                                _ => unreachable!(),
                            } as usize)
                        } else {
                            Err(PacketError::ArgumentError(packets.len(), *operation))
                        }?
                    }
                }
            }
        })
    }

    #[must_use]
    /// Returns number of sub-packets contained within this packet, and its packets, recursively
    pub fn packet_count(&self) -> usize {
        self.flat_packets().len() - 1
    }

    #[must_use]
    /// Returns a flattened vec containing Self and its sub-packets
    pub fn flat_packets(&self) -> Vec<&Self> {
        match &self.kind {
            PacketKind::Literal(_) => vec![self],
            PacketKind::Operator {
                operation: _,
                packets,
            } => packets
                .iter()
                .flat_map(Self::flat_packets)
                .chain(std::iter::once(self))
                .collect(),
        }
    }
}

impl<'a> TryFrom<BitReader<'a>> for Packet {
    type Error = PacketError;

    fn try_from(mut bit_reader: BitReader) -> Result<Self, Self::Error> {
        // VVV
        let version = bit_reader.read_u8(3)?;
        // TTT
        let type_id = bit_reader.read_u8(3)?;
        let kind = match type_id {
            4 => {
                let mut bits = Vec::new();
                let mut reading = true;
                // A+, B+, etc...
                while reading {
                    reading = bit_reader.read_bool()?;
                    bits.push(bit_reader.read_u8(4)?);
                }
                let value = bits
                    .into_iter()
                    .map(usize::from)
                    .reduce(|a, b| a << 4 | b)
                    .unwrap();
                PacketKind::Literal(value)
            }
            operation => {
                // I
                let length = if bit_reader.read_bool()? {
                    LengthKind::PacketCount(bit_reader.read_u16(11)? as usize)
                } else {
                    LengthKind::TotalBits(bit_reader.read_u64(15)?)
                };
                // A*, B*, etc...
                let mut packets = Vec::new();
                let mut sub_packet_reader = bit_reader.relative_reader();
                while {
                    match length {
                        LengthKind::TotalBits(n_bits) => sub_packet_reader.position() < n_bits,
                        LengthKind::PacketCount(n_packets) => packets.len() < n_packets,
                    }
                } {
                    let reader = sub_packet_reader.relative_reader();
                    let packet = Self::try_from(reader).unwrap();
                    sub_packet_reader.skip(packet.length).unwrap();
                    packets.push(packet);
                }
                bit_reader.skip(sub_packet_reader.position())?;
                let operation = Operation::try_from(operation)?;
                PacketKind::Operator { operation, packets }
            }
        };

        let length = bit_reader.position();
        Ok(Self {
            version,
            length,
            kind,
        })
    }
}

impl FromStr for Packet {
    type Err = PacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| u8::from_str_radix(&chunk.join(""), 16).unwrap())
            .collect_vec();
        let bit_reader = BitReader::new(&bytes);
        Self::try_from(bit_reader)
    }
}

#[must_use]
pub fn part_one(input: &'static str) -> usize {
    let packet = parse_input(input);
    packet
        .flat_packets()
        .into_iter()
        .map(|packet| packet.version as usize)
        .sum()
}

#[must_use]
pub fn part_two(input: &'static str) -> usize {
    let packet = parse_input(input);
    packet.eval().expect("AOC input will not be malformed")
}

fn parse_input(input: &'static str) -> Packet {
    input
        .trim()
        .parse()
        .expect("AOC input will not be malformed")
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        for (packet, result) in [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            assert_eq!(part_one(packet), result);
        }
    }

    #[test]
    fn test_part_two() {
        for (packet, result) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            assert_eq!(part_two(packet), result);
        }
    }
}
