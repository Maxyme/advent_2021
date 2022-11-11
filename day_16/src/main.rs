use std::cmp::{max, min};
use std::fs;
use std::str::Chars;

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Not found"),
    }
}

// Read a packet and return subpackets string
fn read_packet_version_sum(packet: &mut Chars) -> usize {
    let version_bits: String = packet.take(3).collect();
    let mut version = usize::from_str_radix(&version_bits, 2).unwrap();
    let type_id_bits: String = packet.take(3).collect();
    let type_id = usize::from_str_radix(&type_id_bits, 2).unwrap();

    match type_id {
        4 => {
            // literal packet type, read multiples of 4 bits until starts with 0
            let mut literal_bits = String::new();
            loop {
                let group: String = packet.take(5).collect();
                // Todo should next mutate this group?
                literal_bits.extend(group.chars().skip(1).take(4));
                let first_bit = group.chars().next().unwrap();
                if first_bit == '0' {
                    // Last group
                    break;
                }
            }
            // Not needed for this part
            let _ = usize::from_str_radix(literal_bits.as_str(), 2).unwrap();
        }
        _ => {
            let length_type_bit = packet.next().unwrap();
            match length_type_bit {
                '0' => {
                    // the next 15 bits are a number that represents the total length
                    // in bits of the sub-packets contained by this packet
                    let packet_length_bits: String = packet.take(15).collect();
                    let sub_packets_length = usize::from_str_radix(&packet_length_bits, 2).unwrap();
                    let sub_packets: String = packet.take(sub_packets_length).collect();
                    // do while length is leftover
                    let mut chars = sub_packets.chars();
                    loop {
                        if !chars.clone().any(|x| x == '1') {
                            // 0 padding
                            break;
                        }
                        version += read_packet_version_sum(&mut chars);
                    }
                }
                _ => {
                    // the next 11 bits are a number that represents the number
                    // in bits of sub-packets immediately contained by this packet
                    let packets_count_bits: String = packet.take(11).collect();
                    let packets_count = usize::from_str_radix(&packets_count_bits, 2).unwrap();
                    let mut i = 0;
                    while i < packets_count {
                        version += read_packet_version_sum(packet);
                        i += 1;
                    }
                }
            }
        }
    }
    version
}

fn expression_operation(type_id: usize, packet_1: usize, packet_2: usize) -> usize {
    match type_id {
        0 => packet_1 + packet_2,
        1 => packet_1 * packet_2,
        2 => min(packet_1, packet_2),
        3 => max(packet_1, packet_2),
        5 => {
            if packet_1 > packet_2 {
                return 1;
            }
            0
        }
        6 => {
            if packet_1 < packet_2 {
                return 1;
            }
            0
        }
        7 => {
            if packet_1 == packet_2 {
                return 1;
            }
            0
        }
        _ => panic!(""),
    }
}

fn read_packet_expression(packet: &mut Chars) -> usize {
    // Skip 3 bits for the version
    packet.take(3).for_each(drop);

    let type_id_bits: String = packet.take(3).collect();
    let type_id = usize::from_str_radix(&type_id_bits, 2).unwrap();

    return match type_id {
        4 => {
            // literal packet type, read multiples of 4 bits until starts with 0
            let mut literal_bits = String::new();
            loop {
                let group: String = packet.take(5).collect();
                // Todo should next mutate this group?
                literal_bits.extend(group.chars().skip(1).take(4));
                let first_bit = group.chars().next().unwrap();
                if first_bit == '0' {
                    // Last group
                    break;
                }
            }
            let literal = usize::from_str_radix(literal_bits.as_str(), 2).unwrap();
            literal
        }
        _ => {
            // Todo use None/Some
            let mut expression: Option<usize> = None;
            let length_type_bit = packet.next().unwrap();
            match length_type_bit {
                '0' => {
                    // the next 15 bits are a number that represents the total length
                    // in bits of the sub-packets contained by this packet
                    let packet_length_bits: String = packet.take(15).collect();
                    let sub_packets_length = usize::from_str_radix(&packet_length_bits, 2).unwrap();
                    let sub_packets: String = packet.take(sub_packets_length).collect();
                    // do while length is leftover
                    let mut chars = sub_packets.chars();
                    loop {
                        if !chars.clone().any(|x| x == '1') {
                            // skip 0 padding
                            break;
                        }
                        let new_value = read_packet_expression(&mut chars);
                        if expression == None {
                            expression = Some(new_value);
                        } else {
                            expression = Some(expression_operation(
                                type_id,
                                expression.unwrap(),
                                new_value,
                            ));
                        }
                    }
                }
                _ => {
                    // the next 11 bits are a number that represents the number
                    // in bits of sub-packets immediately contained by this packet
                    let packets_count_bits: String = packet.take(11).collect();
                    let packets_count = usize::from_str_radix(&packets_count_bits, 2).unwrap();
                    let mut i = 0;
                    while i < packets_count {
                        let new_value = read_packet_expression(packet);
                        if expression == None {
                            expression = Some(new_value);
                        } else {
                            expression = Some(expression_operation(
                                type_id,
                                expression.unwrap(),
                                new_value,
                            ));
                        }
                        i += 1;
                    }
                }
            }
            expression.unwrap()
        }
    };
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let binary_str: String = input.chars().map(to_binary).collect();

    // Part 1: Get the sum of the version numbers
    let version = read_packet_version_sum(&mut binary_str.chars());
    println!("Part 1: {:?}", version);

    // Part 1: Get the packet expression
    let expression = read_packet_expression(&mut binary_str.chars());
    println!("Part 2: {:?}", expression);
}
