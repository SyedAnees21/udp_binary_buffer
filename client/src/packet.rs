use std::{fmt, ptr};

pub const MAX_ELEMENTS: usize = 50;
pub const MAX_BUFFER_SIZE: usize = 500;

pub trait ReadWrite {
    fn write_to_buffer(&self, _buffer: &mut Buffer){}

    fn read_from_buffer(_buffer: &mut Buffer)-> Data {
        let data = Data {
            a: Position { x: 0, y: 0, z: 0 }
        };
        data
    } 
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
//Read/Write func. for packet A
impl ReadWrite for Position {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes(PacketType::Position as u8);
        buffer.write_bytes(self.x);
        buffer.write_bytes(self.y);
        buffer.write_bytes(self.z);
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let pack_a = Position {
            x: buffer.read_bytes(),
            y: buffer.read_bytes(),
            z: buffer.read_bytes(),
        };

        let data = Data {a: pack_a};
        data
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Players {
    pub num_elements: usize,
    pub elements :[u8;MAX_ELEMENTS]
}
//Read/Write func. for packet B
impl ReadWrite for Players {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        if  self.num_elements <= MAX_BUFFER_SIZE {
            buffer.write_bytes(PacketType::Players as u8);
            for i in 0..self.num_elements{
                buffer.write_bytes(self.elements[i]);
            }
        }else {
            println!("Data exceeds the buffer limit data size:{}", self.elements.len());
        }
    }

    fn read_from_buffer( buffer: &mut Buffer) -> Data {
        let num_elem = buffer.read_bytes() as usize;
        let mut element:[u8;MAX_ELEMENTS] = [0;MAX_ELEMENTS];
        for i in 0..num_elem {
            element[i] = buffer.read_bytes();
        }
        let pack_b = Players {
            num_elements: num_elem,
            elements: element
        };

        let data = Data {b: pack_b};

        data
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Actions {
    pub fire_state:bool,
    pub a: u8,
    pub b: u8,
}
//Read/Write func. for packet C
impl ReadWrite for Actions {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_bytes( PacketType::Actions as u8);
        buffer.write_bytes( self.fire_state.into());
        buffer.write_bytes( self.a);
        buffer.write_bytes( self.b);
    }

    fn read_from_buffer( buffer: &mut Buffer)-> Data {
        let pack_c = Actions {
            fire_state: buffer.read_bytes() != 0,
            a: buffer.read_bytes(),
            b: buffer.read_bytes(),
        };

        let data = Data {c: pack_c};

        data
    }
}


#[derive(Debug, Clone)]
pub enum PacketType {   // packets ids
    Position = 1,
    Players = 2,
    Actions = 3
}

#[derive( Debug, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    pub packet_data: Data
}
//Packet Functionality
impl Packet {
    pub fn to_buffer(self, buffer: &mut Buffer) {
        match self.packet_type {
            PacketType::Position => {
                unsafe{
                    self.packet_data.a.write_to_buffer(buffer)
                }
            }
            PacketType::Players => {
                unsafe{
                    self.packet_data.b.write_to_buffer(buffer)
                }
            }
            PacketType::Actions => {
                unsafe{
                    self.packet_data.c.write_to_buffer(buffer)
                }
            }
        }
    }

    pub fn packet_from_buffer(buffer: &mut Buffer) -> Option<Packet> {
        let p_type = buffer.read_bytes();

        match p_type {
            1 => {
                let pack = Packet {
                    packet_type: PacketType::Position,
                    packet_data: Position::read_from_buffer(buffer)
                };
                Some(pack)
            }
            2 => {
                let pack = Packet {
                    packet_type: PacketType::Players,
                    packet_data: Players::read_from_buffer(buffer)
                };
                Some(pack)
            }
            3 => {
                let pack = Packet {
                    packet_type: PacketType::Actions,
                    packet_data: Actions::read_from_buffer(buffer)
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

#[derive( Clone, Copy)]
pub union Data {
    pub a: Position,
    pub b: Players,
    pub c: Actions
}
impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe
        {
            f.debug_struct("Data")
         .field("Packet A:", &self.a)
         .field("Packet B:", &self.b)
         .field("Packet C:", &self.c)
         .finish()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub data: [u8;MAX_BUFFER_SIZE],
    pub ptr: *mut u8,    //Pointer to buffer data
    pub size: usize,     //Size of buffer data bytes
    pub index: usize     //index of next data byte in memory
}
//Buffer Functionality
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
}

