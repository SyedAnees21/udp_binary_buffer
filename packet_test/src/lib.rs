#[cfg(test)]
mod tests {
    use packet::*;

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

        let pack = PlayerStatePacket{
            id: 1,
            position: Point2D { x: 600., y: 975.5 },
            angle: 27.6
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

        let pack = LaserPointsPacket{
            id: 1,
            point1: Point2D { x: 600., y: 975.5 },
            point2:Point2D { x: 1200., y: 1155.6 }
        };

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

        let pack = PlayerInfoPacket{
            name: "Anees".to_string(),
            fighter: 4
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
