use std::{net::UdpSocket};
use packet::*;
use ::Buffer::*;

fn main() {
    let mut buffer = Buffer::new();
    
    let mut position_list : Vec<SpawnLaserPacket> = Vec::new();
    let mut disconnect_list : Vec<DisconnectPacket> = Vec::new();
    let mut player_spawn_list : Vec<SpawnPlayerPacket> = Vec::new();
    let mut connection_list : Vec<InitConnectPacket> = Vec::new();

    let socket = UdpSocket::bind("127.0.0.1:3000").expect("Could not bind socket");
    socket.connect("127.0.0.1:3000").expect("Could not connect to server");

    loop{
        socket.recv(&mut buffer.data).expect("Unable to receive data");
        
        //getting packets list from buffer received!        
        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        for pack in recv_packets.iter() {
            {
                let packet = pack.clone();
                match packet.packet_type {
                    PacketType::SPAWN_LASER => {
                        if let Data::spawn_Laser { packet } = packet.packet_data {
                            position_list.push(packet)
                        }
                    },
                    PacketType::DISCONNECT => {
                        if let Data::disconnect { packet } = packet.packet_data {
                            disconnect_list.push(packet)
                        }
                    },
                    PacketType::SPAWN_PLAYER => {
                        if let Data::spawn_player { packet } = packet.packet_data {
                            player_spawn_list.push(packet)
                        }
                    },
                    PacketType::INIT_CONNECT => {
                        if let Data::Init_Connect { packet } = packet.packet_data {
                            connection_list.push(packet)
                        }
                    },
                    _ => {continue;}
                }
            }
        }

        println!(
            "{:#?}\n{:#?}\n{:#?}\n{:#?}",
            position_list.last(), connection_list.last(),
            player_spawn_list.last(), disconnect_list.last()
        );
        break;
    }
}