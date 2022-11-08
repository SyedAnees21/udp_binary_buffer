#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use packet::*;
    use ::Buffer::*;


    #[test]
    fn movement_packet_test() {
        let mut buffer = Buffer::new();

        let movement = MovementPacket{
            in_type: InTypes::DOWN
        };

        let packet_to_send = Packet{
            packet_type: PacketType::MOVEMENT,
            packet_data: Data::movement { packet: movement }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let movement_recieved;
        
        if let Data::movement { packet } = recieved_packet.packet_data {
            movement_recieved = packet;
            assert_eq!(movement, movement_recieved);
        }

    }

    #[test]
    fn fire_packet_test() {
        let mut buffer = Buffer::new();

        let fire = FirePacket{
            in_type: InTypes::RIGHT_CLICK
        };

        let packet_to_send = Packet{
            packet_type: PacketType::FIRE,
            packet_data: Data::fire{ packet: fire }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let fire_recieved;
        
        if let Data::fire{ packet } = recieved_packet.packet_data {
            fire_recieved = packet;
            assert_eq!(fire, fire_recieved);
        }

    }

    #[test]
    fn rotate_packet_test() {
        let mut buffer = Buffer::new();

        let rotation = RotatePacket{
            in_type: InTypes::LEFT_CLICK,
            point: Point2D { x:45.5, y: 55. }
        };

        let packet_to_send = Packet{
            packet_type: PacketType::ROTATE,
            packet_data: Data::rotate { packet: rotation }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let rotation_recieved;
        
        if let Data::rotate{ packet } = recieved_packet.packet_data {
            rotation_recieved = packet;
            assert_eq!(rotation, rotation_recieved);
        }

    }

    #[test]
    fn dashboard_packet_test() {
        let mut buffer = Buffer::new();

        let dash = DashboardinfoPacket{
            dashboard_info: Dashboardinfo { score: 5., health: 100. }
        };

        let packet_to_send = Packet{
            packet_type: PacketType::DASHBOARD,
            packet_data: Data::dashboard{ packet: dash }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let dash_info_recieved;
        
        if let Data::dashboard{ packet } = recieved_packet.packet_data {
            dash_info_recieved = packet;
            assert_eq!(dash, dash_info_recieved);
        }
    }

    #[test]
    fn playerstate_packet_test() {
        let mut buffer = Buffer::new();
        let mut p_state_vec = Vec::new();
        
        let mut id = 1;
        let mut position= Point2D { x: 600., y: 975.5 };
        let mut angle= 27.6;
        
        for _ in 0..5 {
            let pstate= PlayerState {
                id,
                position,
                angle
            };
            p_state_vec.push(pstate);

            id += 1;
            position.x += 1.;
            position.y += 1.;
            angle += 1.;

        }
        let pack = PlayerStatePacket {
            player_states: p_state_vec
        };
        
        let packet_to_send = Packet{
            packet_type: PacketType::PLAYERSTATE,
            packet_data: Data::playerstate{ packet: pack.clone() }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let pack_recieved;
        
        if let Data::playerstate{ packet } = recieved_packet.packet_data {
            pack_recieved = packet;
            assert_eq!(pack, pack_recieved);
        }
    }
    

    #[test]
    fn laserpoints_packet_test() {
        let mut buffer = Buffer::new();

        let mut point = Point2D {x:2.25, y:35.6};
        let mut dir = Point2D {x: -36., y:-45.5};
        let mut id = 0 as usize;

        let mut laser_points_vec = Vec::new();

        for _ in 0..10 {
            let laser_points = LaserPoints {
                point,
                dir,
                id
            };

            laser_points_vec.push(laser_points);
            point.x += 1.;
            point.y += 1.;
            dir.x += 1.;
            dir.y += 1.;
            id += 1;
        }

        let pack = LaserPointsPacket { laser_points: laser_points_vec };

        let packet_to_send = Packet{
            packet_type: PacketType::LASER_POINTS,
            packet_data: Data::laserpoints{ packet: pack.clone()}
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let pack_recieved;
        
        if let Data::laserpoints{ packet } = recieved_packet.packet_data {
            pack_recieved = packet;
            assert_eq!(pack, pack_recieved);
        }
    }

    #[test]
    fn playerinfo_packet_test() {
        let mut buffer = Buffer::new();
        let mut info_map = HashMap::new();

        let playerinfo1 = PlayerInfo {
            name: "Anees".to_string(),
            fighter: 2
        };
        info_map.insert(0, playerinfo1);

        let playerinfo2 = PlayerInfo {
            name: "Akaam".to_string(),
            fighter: 1
        };
        info_map.insert(1, playerinfo2);

        let pack = PlayerInfoPacket{
            player_info_map: info_map
        };

        let packet_to_send = Packet{
            packet_type: PacketType::PlayerInfo,
            packet_data: Data::playerinfo{ packet: pack.clone() }
        };
        packet_to_send.to_buffer(&mut buffer);

        let recv_packets = Packet::packets_from_buffer(&mut buffer);

        let recieved_packet = recv_packets.iter().next().unwrap().clone();

        let pack_recieved;
        
        if let Data::playerinfo{ packet } = recieved_packet.packet_data {
            pack_recieved = packet;
            assert_eq!(pack, pack_recieved);
        }
    }
    
}
