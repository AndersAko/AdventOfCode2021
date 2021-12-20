use std::fs;
extern crate hex;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let lines: Vec<&str> = filecontents.split_terminator("\n").collect();

    for line in lines {
        let mut bits = String::from("");
        for b in hex::decode(line).unwrap() {
            bits.extend(format!("{:08b}", b).chars());
        }
        println!("{} => {}", line, bits);

        let packet = read_packet(&bits);
        println!("Parsed {:?}", packet);
        println!();
        println!("Part 1: Version sum = {:?}", sum_versions(&packet.0));
        println!();

        println!("Part2: packet value = {}", operator_value(&packet.0));
    }

}

fn sum_versions(packet: &Packet) -> i32 {
    let mut sum = packet.version;
    if let PacketData::SubPackets(sub) = &packet.data {
        for pack in sub {
            sum += sum_versions(&pack);
        }
    } 
    sum
}

fn operator_value(packet: &Packet) -> i128 {
    if let PacketData::SubPackets(subs) = &packet.data {
        match packet.type_id {
            0 => subs.iter().map(|p| operator_value(&p)).sum(),
            1 => subs.iter().fold(1, |prod,val| prod * operator_value(&val) ),
            2 => subs.iter().map(|p| operator_value(&p)).min().unwrap(),
            3 => subs.iter().map(|p| operator_value(&p)).max().unwrap(),
            5 => if operator_value(&subs[0]) > operator_value(&subs[1]) { 1 } else { 0 }
            6 => if operator_value(&subs[0]) < operator_value(&subs[1]) { 1 } else { 0 }
            7 => if operator_value(&subs[0]) == operator_value(&subs[1]) { 1 } else { 0 }
            _ => panic!()
        }
    } else if let PacketData::Literal(lit) = &packet.data {
        assert_eq!(packet.type_id, 4);
        *lit
    } else { panic!() } 
}

#[derive(Debug)]
enum PacketData {
    Literal(i128),
    SubPackets(Vec<Packet>)
}

#[derive(Debug)]
struct Packet {
    version: i32,
    type_id: i32,
    data: PacketData
}

fn read_packet(bits: &str) -> (Packet, usize) { // packet , next index
    let version = i32::from_str_radix(&bits[0..3], 2).unwrap();
    let type_id = i32::from_str_radix(&bits[3..6], 2).unwrap();
    let data: PacketData;

    // println!("Read packet {} (V: {} T: {})", bits, version, type_id);
    let mut index = 6;
    if type_id == 4 {
        let mut literal = String::from("");
        while index+5 <= bits.len() {
            let segment = &bits[index..index+5];
            literal += &segment[1..];
            index += 5;
            if &segment[0..1] == "0" { break; }
        }
        data = PacketData::Literal(i128::from_str_radix(&literal, 2).unwrap());
    } else {
        let mut sub_packets = Vec::new();
        if &bits[index..index+1] == "0" {
            index += 1;
            let length = usize::from_str_radix(&bits[index..index+15], 2).unwrap();
            index += 15;
            let read_until = index + length;
            // println!("--length type 0 = {}", length);
            loop {
                let (packet, next_index) = read_packet(&bits[index..]);
                sub_packets.push(packet);
                index += next_index;
                if index >= read_until { 
                    break;
                }
            }
        } else {
            index += 1;
            let length = usize::from_str_radix(&bits[index..index+11], 2).unwrap();
            // println!("--length type 1 = {}", length);
            index += 11;
            for i in 0..length {
                let (packet, next_index) = read_packet(&bits[index..]);
                sub_packets.push(packet);
                index += next_index;
            }
        }
        data = PacketData::SubPackets(sub_packets);
    }
    let result = (Packet{version, type_id, data}, index);
    // println!("=> {:?}", result );
    result
}
