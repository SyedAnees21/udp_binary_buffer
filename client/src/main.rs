use std::{net::UdpSocket};
use packet::*;

mod packet;

fn main() {
    let mut buffer = Buffer::new();
    
    let mut position_list : Vec<Position> = Vec::new();
    let mut players_list: Vec<Players> = Vec::new();
    let mut action_list: Vec<Actions> = Vec::new();

    let socket = UdpSocket::bind("127.0.0.1:3000").expect("Could not bind socket");
    socket.connect("127.0.0.1:3000").expect("Could not connect to server");

    loop{
        socket.recv(&mut buffer.data).expect("Unable to receive data");
        
        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        for packet in recv_packets.iter() {
            unsafe{
                match packet.packet_type {
                    PacketType::Position => {position_list.push(packet.packet_data.a)},
                    PacketType::Players => {players_list.push(packet.packet_data.b)},
                    PacketType::Actions => {action_list.push(packet.packet_data.c)},
                }
            }
        }

        println!("Position: {:?} \n Players: {:?} \n Actions: {:?}", position_list.last(), players_list.last(), action_list.last());
    }
}