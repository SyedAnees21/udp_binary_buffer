use core::panic;
use std::{ptr, collections::HashMap};

pub const MAX_ELEMENTS: usize = 50;
pub const MAX_BUFFER_SIZE: usize = 1200;

pub trait ReadWrite {
    fn write_to_buffer(&self, _buffer: &mut Buffer){}

    fn read_from_buffer(_buffer: &mut Buffer)-> Data {
        let data = Data::disconnect { packet: DisconnectPacket };
        data
    } 
}

#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub struct Dashboardinfo {
    pub score: f32,
    pub health: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InTypes {
    LEFT_CLICK,
    RIGHT_CLICK,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    DISCONNECT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaserPoints {
    pub point: Point2D, 
    pub dir: Point2D,
    pub id: usize
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfo {
    pub name: String,
    pub fighter: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub id: i32,
    pub position: Point2D, //TODO: later we have to use Vec2 in galactic fighters
    pub angle: f32
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Data{
    disconnect    {packet: DisconnectPacket},
    spawn_Laser   {packet: SpawnLaserPacket},
    spawn_player  {packet: SpawnPlayerPacket},
    Init_Connect  {packet: InitConnectPacket},
    movement      {packet: MovementPacket},
    rotate        {packet: RotatePacket},
    fire          {packet: FirePacket},
    dashboard     {packet: DashboardinfoPacket},
    playerstate   {packet: PlayerStatePacket},
    laserpoints   {packet: LaserPointsPacket},
    playerinfo    {packet: PlayerInfoPacket},
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum PacketType {   // packets ids
    DISCONNECT =1,
    SPAWN_LASER,
    SPAWN_PLAYER,
    INIT_CONNECT,
    MOVEMENT,
    ROTATE,
    FIRE,
    DASHBOARD,
    PLAYERSTATE,
    LASER_POINTS,
    PlayerInfo
}

#[derive(Debug, Clone, Copy)]
pub struct DisconnectPacket;

impl ReadWrite for DisconnectPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::DISCONNECT as u8);
    }

    fn read_from_buffer( _buffer: &mut Buffer)-> Data {
        let pack = DisconnectPacket;

        let data = Data::disconnect{ packet:pack };
        data
    }
    
}


#[derive(Debug, Clone, Copy)]
pub struct SpawnLaserPacket{
    pub point: Point2D
}

impl ReadWrite for SpawnLaserPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::SPAWN_LASER as u8);

        let mut bytes = self.point.x.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
        
        bytes = self.point.y.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let mut bytes: [u8;4] = [0;4];
        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let x = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let y = f32::from_le_bytes(bytes);

        let pack = SpawnLaserPacket{
            point: Point2D { x, y, }
        };

        let data = Data::spawn_Laser{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementPacket {
    pub in_type: InTypes
}

impl ReadWrite for MovementPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::MOVEMENT as u8);
        buffer.write_bytes(self.in_type as u8)
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let intype = match buffer.read_bytes(){
            2 => {InTypes::UP},
            3 => {InTypes::DOWN},
            4 => {InTypes::LEFT},
            5 => {InTypes::RIGHT},
            _ => {panic!("Expected keyboard Input, invalid input received")}
        };
        let pack = MovementPacket { in_type: intype };

        let data = Data::movement { packet: pack };
        data
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotatePacket{
    pub in_type: InTypes,
    pub point: Point2D
}

impl ReadWrite for RotatePacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::ROTATE as u8);
        buffer.write_bytes(self.in_type as u8);

        let mut bytes = self.point.x.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
        
        bytes = self.point.y.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let intype = match buffer.read_bytes(){
            0 => {InTypes::LEFT_CLICK},
            _ => {panic!("Invalid input received, expected mouse left click")}
        };
        
        let mut bytes: [u8;4] = [0;4];
        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let x = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let y = f32::from_le_bytes(bytes);

        let pack = RotatePacket{
            in_type: intype,
            point: Point2D { x, y, }
        };

        let data = Data::rotate{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirePacket {
    pub in_type: InTypes
}

impl ReadWrite for FirePacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::FIRE as u8);
        buffer.write_bytes(self.in_type as u8)
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let intype = match buffer.read_bytes(){
            1 => {InTypes::RIGHT_CLICK},
            _ => {panic!("Invalid input received, expected Mouse Right Click")}
        };
        let pack = FirePacket { in_type: intype };

        let data = Data::fire{ packet: pack };
        data
    }
}

#[derive(Debug, Clone)]
pub struct SpawnPlayerPacket{
    pub in_type: InTypes,
    pub point: Point2D,
    pub player_name: String,
    pub fighter_type: usize
}

impl ReadWrite for SpawnPlayerPacket{
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::SPAWN_PLAYER as u8);
        buffer.write_bytes(self.in_type as u8);

        let mut bytes = self.point.x.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
        
        bytes = self.point.y.to_le_bytes();

        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
     
        buffer.write_bytes(self.fighter_type as u8);
        
        let bytes = self.player_name.as_bytes();
        let str_bytes_len = bytes.len()  as u8;

        buffer.write_bytes(str_bytes_len);

        for byte in bytes.iter(){
            buffer.write_bytes(*byte);
        }
    }
        
    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let in_type = match buffer.read_bytes() {
            0 => {InTypes::LEFT_CLICK},
            1 => {InTypes::RIGHT_CLICK},
            2 => {InTypes::UP},
            3 => {InTypes::DOWN},
            4 => {InTypes::LEFT},
            5 => {InTypes::RIGHT},
            6 => {InTypes::DISCONNECT},
            _ => {panic!("Invalid input type")}
        };

        let mut bytes: [u8;4] = [0;4];
        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let x = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let y = f32::from_le_bytes(bytes);

        let fightertype = buffer.read_bytes() as usize;

        let str_len_byte = buffer.read_bytes();

        let (slice, range) = buffer.get_slice_range(str_len_byte);
        let playername = String::from_utf8_lossy(&slice[0..range]);
        
        let pack = SpawnPlayerPacket{
            in_type,
            point: Point2D { x, y, },
            player_name: playername.to_string(),
            fighter_type: fightertype
        };

        let data = Data::spawn_player{ packet: pack };
        data
    }
}

#[derive(Debug, Clone)]
pub struct InitConnectPacket{
    pub name: String,
    pub unique_index: usize,
    pub other_players_indices: Vec<usize>,
    pub no_players: usize,
    pub arena_dim: Point2D,
    pub dashboard_info: Dashboardinfo,
}

impl ReadWrite for InitConnectPacket{
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::INIT_CONNECT as u8);
        let bytes = self.name.as_bytes();
        
        let str_bytes_len = bytes.len()  as u8;
        buffer.write_bytes(str_bytes_len);

        for byte in bytes.iter(){
            buffer.write_bytes(*byte);
        }
        buffer.write_bytes(self.unique_index as u8);

        let vec_bytes_len = self.other_players_indices.len() as u8;
        buffer.write_bytes(vec_bytes_len);

        for value in self.other_players_indices.iter(){
            buffer.write_bytes(*value as u8);
        }

        buffer.write_bytes(self.no_players as u8);

        let mut bytes = self.arena_dim.x.to_le_bytes();
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
        
        bytes = self.arena_dim.y.to_le_bytes();
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }

        bytes =  self.dashboard_info.health.to_le_bytes();
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }

        bytes =  self.dashboard_info.score.to_le_bytes();
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
    }
        
    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let str_len_byte = buffer.read_bytes();

        let (slice, range) = buffer.get_slice_range(str_len_byte);
        let playername = String::from_utf8_lossy(&slice[0..range]);

        let uniq_index = buffer.read_bytes() as usize;

        let vec_len = buffer.read_bytes();
        let other_indices = buffer.get_vec(vec_len);

        let num_of_players = buffer.read_bytes() as usize;

        let mut bytes: [u8;4] = [0;4];
        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let x = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let y = f32::from_le_bytes(bytes);

        let arena_dimension = Point2D {x, y};

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let h = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let s = f32::from_le_bytes(bytes);
        
        let dash_info = Dashboardinfo {health: h, score: s};

        let pack = InitConnectPacket{
            name: playername.to_string(),
            unique_index: uniq_index,
            other_players_indices: other_indices,
            no_players: num_of_players,
            arena_dim: arena_dimension,
            dashboard_info: dash_info
        };

        let data = Data::Init_Connect{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DashboardinfoPacket {
    pub dashboard_info: Dashboardinfo
}

impl ReadWrite for DashboardinfoPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::DASHBOARD as u8);
        
        let mut bytes;
        bytes =  self.dashboard_info.health.to_le_bytes();
        
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }

        bytes =  self.dashboard_info.score.to_le_bytes();
        for byte in bytes.iter() {
            buffer.write_bytes(*byte);
        }
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let mut bytes= [0;4];

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let h = f32::from_le_bytes(bytes);

        for i in 0..4 {
            bytes[i] = buffer.read_bytes();
        }
        let s = f32::from_le_bytes(bytes);

        let pack = DashboardinfoPacket{dashboard_info: Dashboardinfo { score: s, health: h } };

        let data = Data::dashboard{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerStatePacket { //TODO vec for players state
    pub player_states: Vec<PlayerState>
}

impl ReadWrite for PlayerStatePacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::PLAYERSTATE as u8);
        
        let vec_len = self.player_states.len() as u8;
        buffer.write_bytes(vec_len);

        let mut bytes;

        for player_state in self.player_states.iter() {
            bytes =  player_state.id.to_le_bytes();
            
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  player_state.position.x.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  player_state.position.y.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  player_state.angle.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }
        }
        
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let vec_len = buffer.read_bytes() as usize;
        let mut players_state_vec = Vec::new();

        let mut bytes= [0;4];

        for _ in 0.. vec_len {
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let player_id = i32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let x = f32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let y = f32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let angle = f32::from_le_bytes(bytes);

            let player_state = PlayerState {
                id: player_id,
                position: Point2D { x, y },
                angle
            };
            players_state_vec.push(player_state);
        }
        

        let pack = PlayerStatePacket { player_states: players_state_vec};

        let data = Data::playerstate{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LaserPointsPacket { 
    pub laser_points: Vec<LaserPoints>,
}

impl ReadWrite for LaserPointsPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::LASER_POINTS as u8);
        
        let vec_len = self.laser_points.len() as u8;
        buffer.write_bytes(vec_len); 

        let mut bytes;

        for l_points in self.laser_points.iter() {
            bytes =  l_points.point.x.to_le_bytes();
        
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  l_points.point.y.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  l_points.dir.x.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            bytes =  l_points.dir.y.to_le_bytes();
            for byte in bytes.iter() {
                buffer.write_bytes(*byte);
            }

            buffer.write_bytes(l_points.id as u8);
        }
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let vec_len = buffer.read_bytes() as usize;
        let mut vec = Vec::new();

        let mut bytes= [0;4];

        for _ in 0..vec_len {

            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let x1 = f32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let y1 = f32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let x2 = f32::from_le_bytes(bytes);
    
            for i in 0..4 {
                bytes[i] = buffer.read_bytes();
            }
            let y2 = f32::from_le_bytes(bytes);
    
            let id = buffer.read_bytes() as usize;

            let l_points = LaserPoints {
                point: Point2D { x: x1, y: y1 },
                dir: Point2D { x: x2, y: y2 },
                id,
            };
            vec.push(l_points);
        }

        let pack = LaserPointsPacket{
                laser_points: vec
            };

        let data = Data::laserpoints{ packet: pack };
        data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoPacket {
    pub player_info_map: HashMap<usize, PlayerInfo>
}

impl ReadWrite for PlayerInfoPacket{
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::PlayerInfo as u8);
        
        let map_len = self.player_info_map.len() as u8;
        buffer.write_bytes(map_len);

        for player_info in self.player_info_map.iter() {

            buffer.write_bytes(*player_info.0 as u8);

            let bytes = player_info.1.name.as_bytes();
            
            let str_bytes_len = bytes.len()  as u8;

            buffer.write_bytes(str_bytes_len);

            for byte in bytes.iter(){
                buffer.write_bytes(*byte);
            }
            
            buffer.write_bytes(player_info.1.fighter as u8);
        }
    }
        
    fn read_from_buffer( buffer: &mut Buffer)-> Data {

        let mut info_map = HashMap::new();
        let map_len = buffer.read_bytes() as usize;

        for _ in 0..map_len {
            let key = buffer.read_bytes() as usize;
            let str_len_byte = buffer.read_bytes();

            let (slice, range) = buffer.get_slice_range(str_len_byte);
            let playername = String::from_utf8_lossy(&slice[0..range]);

            let fighter_type = buffer.read_bytes() as usize;

            let player_info = PlayerInfo { name: playername.to_string() , fighter: fighter_type };

            info_map.insert(key, player_info);
        }
        let pack = PlayerInfoPacket { player_info_map: info_map };

        let data = Data::playerinfo{ packet: pack };
        data
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub data: [u8;MAX_BUFFER_SIZE],
    pub ptr: *mut u8,    //Pointer to buffer data
    pub size: usize,     //Size of buffer data bytes
    pub index: usize     //index of next data byte in memory
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut b = Buffer {
            data : [0;MAX_BUFFER_SIZE],
            ptr : ptr::null_mut(),
            index : 0,
            size : MAX_BUFFER_SIZE,
        };
        b.ptr = &mut b.data[0];

        b
    }

    pub fn set_index(&mut self) {
        self.ptr = &mut self.data[0];
        self.index = 0;
    }

    pub fn reset(&mut self) {
        self.data.fill(0);
        self.ptr = &mut self.data[0];
        self.index = 0;
    }

    pub fn write_bytes(&mut self, value:u8) {
        unsafe
        {
            assert!(self.index + 1 <= self.size);
            let ptr  = self.ptr.add(self.index);
            *ptr = value;
            self.index += 1;
        }
    }

    pub fn read_bytes(&mut self) -> u8 {
        unsafe
        {
            let ptr = self.ptr.add(self.index);
            let value = *ptr;
            self.index += 1;
            value
        }
    }

    pub fn get_slice_range(&mut self, mut len: u8) -> ([u8;200],usize) {
   
        let mut slice:[u8;200]=[0;200];
        let mut i = 0;
        while len > 0 {
            let value = self.read_bytes();
            slice[i] = value;
            i += 1;
            len -= 1;
        }
        (slice, i)
    }

    pub fn get_vec(&mut self, mut len: u8)-> Vec<usize> {
        let mut vec = Vec::new();
        while len > 0 {
            let value = self.read_bytes() as usize;
            vec.push(value);
            len -= 1;
        }
        vec
    }
}

#[derive( Debug, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    pub packet_data: Data
}

impl Packet {
    pub fn to_buffer(self, buffer: &mut Buffer) {
        match self.packet_type {
            PacketType::DISCONNECT => {
                if let Data::disconnect { packet }= self.packet_data{
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::SPAWN_LASER => {
                if let Data::spawn_Laser { packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::SPAWN_PLAYER => {
                if let Data::spawn_player { packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::INIT_CONNECT => {
                if let Data::Init_Connect { packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::MOVEMENT => {
                if let Data::movement{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::ROTATE => {
                if let Data::rotate{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::FIRE => {
                if let Data::fire{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::DASHBOARD => {
                if let Data::dashboard{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::PLAYERSTATE => {
                if let Data::playerstate{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::LASER_POINTS => {
                if let Data::laserpoints{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
            PacketType::PlayerInfo => {
                if let Data::playerinfo{ packet } = self.packet_data {
                    packet.write_to_buffer(buffer)
                }
            },
        }
    }

    pub fn packet_from_buffer(buffer: &mut Buffer) -> Option<Packet> {
        let p_type = buffer.read_bytes();

        match p_type {
            1 => {
                let pack = Packet {
                    packet_type: PacketType::DISCONNECT,
                    packet_data: DisconnectPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            2 => {
                let pack = Packet {
                    packet_type: PacketType::SPAWN_LASER,
                    packet_data: SpawnLaserPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            3 => {
                let pack = Packet {
                    packet_type: PacketType::SPAWN_PLAYER,
                    packet_data: SpawnPlayerPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            4 => {
                let pack = Packet {
                    packet_type: PacketType::INIT_CONNECT,
                    packet_data: InitConnectPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            5 => {
                let pack = Packet {
                    packet_type: PacketType::MOVEMENT,
                    packet_data: MovementPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            6 => {
                let pack = Packet {
                    packet_type: PacketType::ROTATE,
                    packet_data: RotatePacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            7 => {
                let pack = Packet {
                    packet_type: PacketType::FIRE,
                    packet_data: FirePacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            8 => {
                let pack = Packet {
                    packet_type: PacketType::DASHBOARD,
                    packet_data: DashboardinfoPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            9 => {
                let pack = Packet {
                    packet_type: PacketType::PLAYERSTATE,
                    packet_data: PlayerStatePacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            10 => {
                let pack = Packet {
                    packet_type: PacketType::LASER_POINTS,
                    packet_data: LaserPointsPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            11 => {
                let pack = Packet {
                    packet_type: PacketType::PlayerInfo,
                    packet_data: PlayerInfoPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            _ => {None}
        }
    }

    pub fn packets_from_buffer(buffer: &mut Buffer) -> Vec<Packet> {
        buffer.set_index();
        let mut packets_list = Vec::new();
        loop {
            if let Some(packet) = Packet::packet_from_buffer(buffer) {
                packets_list.push(packet);
            }else {
                buffer.reset();
                break;
            }
        }
        packets_list 
    }
}