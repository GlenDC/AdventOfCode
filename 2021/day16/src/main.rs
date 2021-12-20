use anyhow::{anyhow, Result};

fn main() {
    println!("++++++++++++");
    println!("PART#1");
    println!("++++++++++++");

    part1(TEST_INPUT_A);
    part1(TEST_INPUT_B);
    part1(TEST_INPUT_C);
    part1(TEST_INPUT_D);

    println!("-----");

    part1(INPUT);

    println!("++++++++++++");
    println!("PART#2");
    println!("++++++++++++");

    part2(TEST_INPUT_A);
    part2(TEST_INPUT_B);
    part2(TEST_INPUT_C);
    part2(TEST_INPUT_D);

    println!("-----");

    part2(INPUT);
}

fn part1(input: &str) {
    let decoder: Decoder = input.into();
    let packet = decoder.read_packet().unwrap();
    let version_sum: u32 = packet.iter().map(|packet| packet.version).sum();
    println!("{}", version_sum);
}

fn part2(input: &str) {
    let decoder: Decoder = input.into();
    let packet = decoder.read_packet().unwrap();
    let result = packet.eval().unwrap();
    println!("{}", result);
}

struct Packet {
    pub version: u32,
    pub data: PacketData,
}

enum PacketData {
    Literal(u64),           // ID=4
    Operator(OperatorData), // ID!=4
}

struct OperatorData {
    pub packet_id: u32,
    pub sub_packets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq)]
enum LenghtTypeID {
    Zero,
    One,
}

impl LenghtTypeID {
    pub fn new(b: u8) -> Result<LenghtTypeID> {
        match b {
            0 => Ok(LenghtTypeID::Zero),
            1 => Ok(LenghtTypeID::One),
            _ => Err(anyhow!("unexpected bit received: {}", b)),
        }
    }
}

impl Packet {
    pub fn iter(&self) -> PacketIter {
        PacketIter {
            packets: vec![self],
        }
    }

    pub fn eval(&self) -> Result<u64> {
        let mut packets: Vec<&Packet> = self.iter().collect();
        if packets.len() == 1 {
            return match packets[0].data {
                PacketData::Literal(x) => Ok(x),
                _ => Err(anyhow!("unexpected last remaining packet")),
            };
        }

        let mut value_stack: Vec<u64> = Vec::new();
        loop {
            let packet = match packets.pop() {
                None => break,
                Some(packet) => packet,
            };
            match &packet.data {
                PacketData::Literal(x) => {
                    value_stack.push(*x);
                }
                PacketData::Operator(op_data) => match op_data.packet_id {
                    // Packets with type ID 0 are sum packets:
                    // their value is the sum of the values of their sub-packets.
                    // If they only have a single sub-packet, their value is the value of the sub-packet.
                    0 => {
                        let n = op_data.sub_packets.len();
                        if n == 0 {
                            return Err(anyhow!("failed to sum: no sub packets found"));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!("failed to sum: value stack out of range"));
                        }
                        let sum: u64 = value_stack.drain(value_stack.len()-n..).sum();
                        value_stack.push(sum);
                    }
                    // Packets with type ID 1 are product packets:
                    // their value is the result of multiplying together the values of their sub-packets.
                    // If they only have a single sub-packet, their value is the value of the sub-packet.
                    1 => {
                        let n = op_data.sub_packets.len();
                        if n == 0 {
                            return Err(anyhow!("failed to multiply: no sub packets found"));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!("failed to multiply: value stack out of range"));
                        }
                        let mut it = value_stack.drain(value_stack.len()-n..);
                        let mut p: u64 = it
                            .next()
                            .ok_or(anyhow!("failed to multiply: no sub packets found"))?;
                        p = it.fold(p, |acc, x| acc * x);
                        value_stack.push(p);
                    }
                    // Packets with type ID 2 are minimum packets:
                    // their value is the minimum of the values of their sub-packets.
                    2 => {
                        let n = op_data.sub_packets.len();
                        if n == 0 {
                            return Err(anyhow!("failed to take min value: no sub packets found"));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!(
                                "failed to take min value: value stack out of range"
                            ));
                        }
                        let min: u64 = value_stack.drain(value_stack.len()-n..)
                            .min()
                            .ok_or(anyhow!(
                                "failed to take min value: no values found on value stack"
                            ))?;
                        value_stack.push(min);
                    }
                    // Packets with type ID 3 are maximum packets:
                    // their value is the maximum of the values of their sub-packets.
                    3 => {
                        let n = op_data.sub_packets.len();
                        if n == 0 {
                            return Err(anyhow!("failed to take max value: no sub packets found"));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!(
                                "failed to take max value: value stack out of range"
                            ));
                        }
                        let max: u64 = value_stack.drain(value_stack.len()-n..)
                            .max()
                            .ok_or(anyhow!(
                                "failed to take max value: no values found on value stack"
                            ))?;
                        value_stack.push(max);
                    }
                    // Packets with type ID 5 are greater than packets:
                    // their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet;
                    // otherwise, their value is 0. These packets always have exactly two sub-packets.
                    5 => {
                        let n = op_data.sub_packets.len();
                        if n != 2 {
                            return Err(anyhow!("failed to apply greater then test: expected 2 packets, found {} packet(s)", n));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!(
                                "failed to apply greater then test: value stack out of range"
                            ));
                        }
                        let y = value_stack.pop().ok_or(anyhow!(
                            "failed to apply greater then test: value stack out of range (2/2)"
                        ))?;
                        let x = value_stack.pop().ok_or(anyhow!(
                            "failed to apply greater then test: value stack out of range (1/2)"
                        ))?;
                        let v = (x > y) as u64;
                        value_stack.push(v);
                    }
                    // Packets with type ID 6 are less than packets:
                    // their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet;
                    // otherwise, their value is 0. These packets always have exactly two sub-packets.
                    6 => {
                        let n = op_data.sub_packets.len();
                        if n != 2 {
                            return Err(anyhow!("failed to apply less then test: expected 2 packets, found {} packet(s)", n));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!(
                                "failed to apply less then test: value stack out of range"
                            ));
                        }
                        let y = value_stack.pop().ok_or(anyhow!(
                            "failed to apply less then test: value stack out of range (2/2)"
                        ))?;
                        let x = value_stack.pop().ok_or(anyhow!(
                            "failed to apply less then test: value stack out of range (1/2)"
                        ))?;
                        let v = (x < y) as u64;
                        value_stack.push(v);
                    }
                    // Packets with type ID 7 are equal to packets:
                    // their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet;
                    // otherwise, their value is 0. These packets always have exactly two sub-packets.
                    7 => {
                        let n = op_data.sub_packets.len();
                        if n != 2 {
                            return Err(anyhow!("failed to apply equal to test: expected 2 packets, found {} packet(s)", n));
                        }
                        if n > value_stack.len() {
                            return Err(anyhow!(
                                "failed to apply equal to test: value stack out of range"
                            ));
                        }
                        let y = value_stack.pop().ok_or(anyhow!(
                            "failed to apply equal to test: value stack out of range (2/2)"
                        ))?;
                        let x = value_stack.pop().ok_or(anyhow!(
                            "failed to apply equal to test: value stack out of range (1/2)"
                        ))?;
                        let v = (x == y) as u64;
                        value_stack.push(v);
                    }
                    // anything else is unexpected
                    _ => {
                        return Err(anyhow!(
                            "unexpected operator packet id {}",
                            op_data.packet_id
                        ))
                    }
                },
            }
        }
        if value_stack.len() > 1 {
            return Err(anyhow!("unexpected value stack state at end of eval: {:?}", value_stack));
        }
        Ok(value_stack[0])
    }
}

struct PacketIter<'a> {
    packets: Vec<&'a Packet>,
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = &'a Packet;

    fn next(&mut self) -> Option<&'a Packet> {
        let packet = match self.packets.pop() {
            None => return None,
            Some(packet) => packet,
        };
        match &packet.data {
            PacketData::Literal(_) => (),
            PacketData::Operator(data) => {
                self.packets.extend(&data.sub_packets);
            }
        };
        Some(packet)
    }
}

struct Decoder<'a> {
    it: HexToBinIter<'a>,
}

impl<'a> Decoder<'a> {
    pub fn read_packet(mut self) -> Result<Packet> {
        let mut stack = Vec::new();
        self.read_initial_packet(&mut stack)?;

        loop {
            let packet = match stack.pop() {
                None => return Err(anyhow!("unexpected end of decode stack")),
                Some(state) => match state {
                    DecoderState::Literal(version) => Some(self.read_literal(version)?),
                    DecoderState::Operator(op_data) => {
                        self.read_operator_data(&mut stack, op_data)?
                    }
                },
            };
            let packet = match packet {
                None => continue,
                Some(packet) => packet,
            };
            match stack.pop() {
                // finished
                None => {
                    let remain = self.it.filter(|b| *b != 0).next();
                    if remain.is_some() {
                        return Err(anyhow!(
                            "cannot return packet: decoder's iterator ended on invalid padding"
                        ));
                    }
                    return Ok(packet);
                }
                Some(DecoderState::Operator(op_data)) => {
                    let pos = self.it.position();
                    let n = op_data.n
                        - match op_data.length_type_id {
                            // 'n' represents the total size of sub packets contained within
                            LenghtTypeID::Zero => (pos - op_data.last_pos) as u32,
                            // 'n' represent the number of sub-packets immediately contained by this packet
                            LenghtTypeID::One => 1,
                        };
                    let mut sub_packets = op_data.sub_packets;
                    sub_packets.push(packet);
                    stack.push(DecoderState::Operator(OperatorDecodeStateData {
                        version: op_data.version,
                        packet_id: op_data.packet_id,
                        length_type_id: op_data.length_type_id,
                        sub_packets: sub_packets,
                        n,
                        last_pos: pos,
                    }));
                }
                _ => return Err(anyhow!("unexpected stack state")),
            }
        }
    }

    fn read_initial_packet(&mut self, stack: &mut Vec<DecoderState>) -> Result<()> {
        let version = self.read_u32(3)?;
        let packet_id = self.read_u32(3)?;

        if packet_id == 4 {
            stack.push(DecoderState::Literal(version));
        } else {
            let length_type_id =
                LenghtTypeID::new(self.it.next().ok_or(anyhow!("unexpected EOF"))?)?;
            let n = match length_type_id {
                // next 15 bits are a numbers that represents the total size of sub packets contained within
                LenghtTypeID::Zero => self.read_u32(15)?,
                // next 11 bits are a number that represent the number of sub-packets immediately contained by this packet
                LenghtTypeID::One => self.read_u32(11)?,
            };
            stack.push(DecoderState::Operator(OperatorDecodeStateData {
                version,
                packet_id,
                length_type_id,
                sub_packets: Vec::new(),
                n,
                last_pos: self.it.position(),
            }));
        }

        Ok(())
    }

    fn read_operator_data(
        &mut self,
        stack: &mut Vec<DecoderState>,
        op_data: OperatorDecodeStateData,
    ) -> Result<Option<Packet>> {
        if op_data.n == 0 {
            return Ok(Some(Packet {
                version: op_data.version,
                data: PacketData::Operator(OperatorData {
                    packet_id: op_data.packet_id,
                    sub_packets: op_data.sub_packets,
                }),
            }));
        }

        // push op_data back to stack... not yet finished
        stack.push(DecoderState::Operator(op_data));

        let version = self.read_u32(3)?;
        let packet_id = self.read_u32(3)?;

        if packet_id == 4 {
            stack.push(DecoderState::Literal(version));
        } else {
            let length_type_id =
                LenghtTypeID::new(self.it.next().ok_or(anyhow!("unexpected EOF"))?)?;
            let n = match length_type_id {
                LenghtTypeID::Zero => self.read_u32(15)?,
                LenghtTypeID::One => self.read_u32(11)?,
            };
            stack.push(DecoderState::Operator(OperatorDecodeStateData {
                version,
                packet_id,
                length_type_id,
                sub_packets: Vec::new(),
                n,
                last_pos: self.it.position(),
            }));
        }

        Ok(None)
    }

    fn read_literal(&mut self, version: u32) -> Result<Packet> {
        let mut bits_buffer = Vec::new();
        let mut read_more = true;
        while read_more {
            read_more = match self.it.next() {
                None => return Err(anyhow!("unexpected EOF")),
                Some(b) => b != 0,
            };
            for _ in 0..4 {
                bits_buffer.push(self.it.next().ok_or(anyhow!("unexpected EOF"))?);
            }
        }
        let mut x: u64 = 0;
        for (idx, bit) in bits_buffer.iter().enumerate() {
            x |= (*bit as u64) << (bits_buffer.len() - idx - 1)
        }
        Ok(Packet {
            version,
            data: PacketData::Literal(x),
        })
    }

    fn read_u32(&mut self, bits: usize) -> Result<u32> {
        let mut x: u32 = 0;
        for i in 0..bits {
            let bit = self.it.next().ok_or(anyhow!("unexpected EOF"))? as u32;
            x |= bit << bits - i - 1;
        }
        Ok(x)
    }
}

enum DecoderState {
    Literal(u32),
    Operator(OperatorDecodeStateData),
}

struct OperatorDecodeStateData {
    pub version: u32,
    pub packet_id: u32,
    pub length_type_id: LenghtTypeID,
    pub sub_packets: Vec<Packet>,
    pub n: u32,
    pub last_pos: usize,
}

impl<'a> From<&'a str> for Decoder<'a> {
    fn from(s: &'a str) -> Decoder<'a> {
        Decoder { it: s.into() }
    }
}

struct HexToBinIter<'a> {
    bytes_it: std::slice::Iter<'a, u8>,
    nibble: Option<NibbleToBinIter>,
    pos: usize,
}

impl<'a> HexToBinIter<'a> {
    pub fn new(s: &'a str) -> HexToBinIter<'a> {
        let bytes_it = s.as_bytes().iter();
        HexToBinIter {
            bytes_it,
            nibble: None,
            pos: 0,
        }
    }

    pub fn position(&'a self) -> usize {
        self.pos
    }
}

impl<'a> Iterator for HexToBinIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match &mut self.nibble {
            None => match self.bytes_it.next() {
                None => None,
                Some(b) => {
                    self.nibble = Some(NibbleToBinIter::from_byte(*b).unwrap());
                    self.next()
                }
            },
            Some(nibble_it) => match nibble_it.next() {
                None => {
                    self.nibble = None;
                    self.next()
                }
                Some(bit) => {
                    self.pos += 1;
                    Some(bit)
                }
            },
        }
    }
}

impl<'a> From<&'a str> for HexToBinIter<'a> {
    fn from(s: &'a str) -> HexToBinIter<'a> {
        HexToBinIter::new(s)
    }
}

struct NibbleToBinIter {
    n: u8,
    m: u8,
}

impl NibbleToBinIter {
    pub fn new(n: u8) -> Result<NibbleToBinIter> {
        if n >= 16 {
            return Err(anyhow!("n takes more than 4 bits (Out of Range)"));
        }

        Ok(NibbleToBinIter { n, m: 8 })
    }

    pub fn from_byte(b: u8) -> Result<NibbleToBinIter> {
        match b {
            b'0'..=b'9' => NibbleToBinIter::new(b - 48),
            b'A'..=b'F' => NibbleToBinIter::new(b - 55),
            _ => Err(anyhow!("unexpected byte {}", b)),
        }
    }
}

impl Iterator for NibbleToBinIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.m == 0 {
            return None;
        }
        let out = (self.n & self.m != 0) as u8;
        self.m >>= 1;
        Some(out)
    }
}

const TEST_INPUT_A: &'static str = "8A004A801A8002F478";
const TEST_INPUT_B: &'static str = "620080001611562C8802118E34";
const TEST_INPUT_C: &'static str = "C0015000016115A2E0802F182340";
const TEST_INPUT_D: &'static str = "A0016C880162017C3686B18A3D4780";

const INPUT: &'static str = "220D4B80491FE6FBDCDA61F23F1D9B763004A7C128012F9DA88CE27B000B30F4804D49CD515380352100763DC5E8EC000844338B10B667A1E60094B7BE8D600ACE774DF39DD364979F67A9AC0D1802B2A41401354F6BF1DC0627B15EC5CCC01694F5BABFC00964E93C95CF080263F0046741A740A76B704300824926693274BE7CC880267D00464852484A5F74520005D65A1EAD2334A700BA4EA41256E4BBBD8DC0999FC3A97286C20164B4FF14A93FD2947494E683E752E49B2737DF7C4080181973496509A5B9A8D37B7C300434016920D9EAEF16AEC0A4AB7DF5B1C01C933B9AAF19E1818027A00A80021F1FA0E43400043E174638572B984B066401D3E802735A4A9ECE371789685AB3E0E800725333EFFBB4B8D131A9F39ED413A1720058F339EE32052D48EC4E5EC3A6006CC2B4BE6FF3F40017A0E4D522226009CA676A7600980021F1921446700042A23C368B713CC015E007324A38DF30BB30533D001200F3E7AC33A00A4F73149558E7B98A4AACC402660803D1EA1045C1006E2CC668EC200F4568A5104802B7D004A53819327531FE607E118803B260F371D02CAEA3486050004EE3006A1E463858600F46D8531E08010987B1BE251002013445345C600B4F67617400D14F61867B39AA38018F8C05E430163C6004980126005B801CC0417080106005000CB4002D7A801AA0062007BC0019608018A004A002B880057CEF5604016827238DFDCC8048B9AF135802400087C32893120401C8D90463E280513D62991EE5CA543A6B75892CB639D503004F00353100662FC498AA00084C6485B1D25044C0139975D004A5EB5E52AC7233294006867F9EE6BA2115E47D7867458401424E354B36CDAFCAB34CBC2008BF2F2BA5CC646E57D4C62E41279E7F37961ACC015B005A5EFF884CBDFF10F9BFF438C014A007D67AE0529DED3901D9CD50B5C0108B13BAFD6070";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_decoder() {
        let test_cases = vec![
            ("D2FE28", "110100101111111000101000"),
            (
                "38006F45291200",
                "00111000000000000110111101000101001010010001001000000000",
            ),
            (
                "EE00D40C823060",
                "11101110000000001101010000001100100000100011000001100000",
            ),
        ];
        for test_case in test_cases {
            let it: HexToBinIter = test_case.0.into();
            let s = String::from_utf8(it.map(|b| b + 48).collect()).unwrap();
            assert_eq!(s, test_case.1);
        }
    }

    #[test]
    fn test_decode_literal_packet() {
        let decoder: Decoder = "D2FE28".into();
        let packet = decoder.read_packet().unwrap();
        assert_eq!(packet.version, 6);
        match packet.data {
            PacketData::Literal(x) => assert_eq!(x, 2021),
            _ => panic!("unexpected data"),
        };
    }

    #[test]
    fn test_decode_operator_packet_length_v0() {
        let decoder: Decoder = "38006F45291200".into();
        let packet = decoder.read_packet().unwrap();
        assert_eq!(packet.version, 1);
        match packet.data {
            PacketData::Literal(_) => panic!("unexpected data"),
            PacketData::Operator(op_data) => {
                assert_eq!(op_data.packet_id, 6);
                assert_eq!(op_data.sub_packets.len(), 2);
                match op_data.sub_packets[0].data {
                    PacketData::Literal(x) => assert_eq!(x, 10),
                    _ => panic!("unexpected sub packet #1 data"),
                };
                match op_data.sub_packets[1].data {
                    PacketData::Literal(x) => assert_eq!(x, 20),
                    _ => panic!("unexpected sub packet #2 data"),
                };
            }
        };
    }

    #[test]
    fn test_decode_operator_packet_length_v1() {
        let decoder: Decoder = "EE00D40C823060".into();
        let packet = decoder.read_packet().unwrap();
        assert_eq!(packet.version, 7);
        match packet.data {
            PacketData::Literal(_) => panic!("unexpected data"),
            PacketData::Operator(op_data) => {
                assert_eq!(op_data.packet_id, 3);
                assert_eq!(op_data.sub_packets.len(), 3);
                match op_data.sub_packets[0].data {
                    PacketData::Literal(x) => assert_eq!(x, 1),
                    _ => panic!("unexpected sub packet #1 data"),
                };
                match op_data.sub_packets[1].data {
                    PacketData::Literal(x) => assert_eq!(x, 2),
                    _ => panic!("unexpected sub packet #2 data"),
                };
                match op_data.sub_packets[2].data {
                    PacketData::Literal(x) => assert_eq!(x, 3),
                    _ => panic!("unexpected sub packet #3 data"),
                };
            }
        };
    }

    #[test]
    fn test_value_eval() {
        let test_cases = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for test_case in test_cases {
            let decoder: Decoder = test_case.0.into();
            let packet: Packet = decoder.read_packet().unwrap();
            let result = packet.eval().unwrap();
            assert_eq!(result, test_case.1);
        }
    }
}
