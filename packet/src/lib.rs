use core::panic;
use std::ptr;

pub const MAX_ELEMENTS: usize = 50;
pub const MAX_BUFFER_SIZE: usize = 1200;

pub trait ReadWrite {
    fn write_to_buffer(&self, _buffer: &mut Buffer){}

    fn read_from_buffer(_buffer: &mut Buffer)-> Data {
        let data = Data::disconnect { packet: DisconnectPacket };
        data
    } 
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Dashboardinfo {
    pub score: f32,
    pub health: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum InTypes {
    LEFT_CLICK,
    RIGHT_CLICK,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    DISCONNECT,
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Data{
    disconnect{packet: DisconnectPacket},
    spawn_Laser{packet: SpawnLaserPacket},
    game_play{packet: GamePlayPacket},
    spawn_player{packet: SpawnPlayerPacket},
    Init_Connect{packet: InitConnectPacket},
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum PacketType {   // packets ids
    DISCONNECT =1,
    SPAWN_LASER,
    GAMEPLAY,
    SPAWN_PLAYER,
    INIT_CONNECT,
}

#[derive(Debug, Clone, Copy)]
pub struct DisconnectPacket;

impl ReadWrite for DisconnectPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::DISCONNECT as u8);
    }

    fn read_from_buffer( _buffer: &mut Buffer)-> Data {
        let pack = DisconnectPacket;

        // let data = Data{disconnect: pack};
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
        // let data = Data {spawn_laser: pack};
        data
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GamePlayPacket{
    pub in_type: InTypes,
    pub point: Point2D,
}

impl ReadWrite for GamePlayPacket {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::GAMEPLAY as u8);
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

        let pack = GamePlayPacket{
            in_type,
            point: Point2D { x, y, }
        };

        // let data = Data {game_play: pack};
        let data = Data::game_play{ packet: pack };

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
                {
                    if let Data::disconnect { packet }= self.packet_data{
                        packet.write_to_buffer(buffer)
                    }
                }
            },
            PacketType::SPAWN_LASER => {
                {
                    if let Data::spawn_Laser { packet } = self.packet_data {
                        packet.write_to_buffer(buffer)
                    }
                }
            },
            PacketType::GAMEPLAY => {
                {
                    if let Data::game_play { packet } = self.packet_data {
                        packet.write_to_buffer(buffer)
                    }
                }
            },
            PacketType::SPAWN_PLAYER => {
                {
                    if let Data::spawn_player { packet } = self.packet_data {
                        packet.write_to_buffer(buffer)
                    }
                }
            },
            PacketType::INIT_CONNECT => {
                {
                    if let Data::Init_Connect { packet } = self.packet_data {
                        packet.write_to_buffer(buffer)
                    }
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
                    packet_type: PacketType::GAMEPLAY,
                    packet_data: GamePlayPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            4 => {
                let pack = Packet {
                    packet_type: PacketType::SPAWN_PLAYER,
                    packet_data: SpawnPlayerPacket::read_from_buffer(buffer)
                };
                Some(pack)
            }
            5 => {
                let pack = Packet {
                    packet_type: PacketType::INIT_CONNECT,
                    packet_data: InitConnectPacket::read_from_buffer(buffer)
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