use std::{net::UdpSocket, time::Duration, thread};
use packet::*;
use ::Buffer::*;

fn main() {
    //Creating a new buffer
    let mut buffer = Buffer::new();

    let socket = UdpSocket::bind("0.0.0.0:3000").expect("Could not bind the socket");

    let mut position = Point2D {
        x: 5.25,
        y: -3.3,
    };

    let player_spawn = SpawnPlayerPacket {
        in_type: InTypes::RIGHT,
        point: position,
        player_name: String::from("Rajit, Akaam, Kasun"),
        fighter_type: 0
    };

    let connect_request = InitConnectPacket{
        name: String::from("Anees"),
        unique_index: 7,
        other_players_indices: vec![1,4,5,8],
        no_players: 5,
        arena_dim: Point2D { x: 1200., y: 1200. },
        dashboard_info: Dashboardinfo { score: 0., health: 5. }
    };

    let i = 0.5;

    loop {
        let spawn_laser_packet = SpawnLaserPacket {
            point: position
        };

        let packet1_to_send = Packet {
            packet_type: PacketType::SPAWN_LASER,
            packet_data: Data::spawn_Laser{ packet: spawn_laser_packet },
        };
        packet1_to_send.to_buffer(&mut buffer);

        let packet3_to_send = Packet{
            packet_type: PacketType::SPAWN_PLAYER,
            packet_data: Data::spawn_player { packet: player_spawn.clone()}
        };
        packet3_to_send.to_buffer(&mut buffer);

        let packet4_to_send = Packet {
            packet_type: PacketType::INIT_CONNECT,
            packet_data: Data::Init_Connect { packet: connect_request.clone() }
        };
        packet4_to_send.to_buffer(&mut buffer);

        println!("Data in the buffer {:?}", buffer.data);
    
        let _ = socket.send_to(&buffer.data, "127.0.0.1:3000").expect("unable to send");

        position.x += i;
        position.y += i;

        thread::sleep(Duration::from_secs(2));
        buffer.reset();
        break;
    }
    
}
