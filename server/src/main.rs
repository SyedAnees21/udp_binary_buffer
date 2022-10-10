use std::{net::UdpSocket, fmt, ptr, thread, time::Duration};


const MAX_ELEMENTS: usize = 50;
const MAX_BUFFER_SIZE: usize = 500;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug, Clone, Copy)]
struct Players {
    num_elements: usize,
    elements :[u8;MAX_ELEMENTS]
}

#[derive(Debug, Clone, Copy)]
struct Actions {
    fire_state:bool,
    a: u8,
    b: u8,
} 

#[derive(Debug, Clone)]
enum PacketType {   // packets ids
    Position = 1,
    Players = 2,
    Actions = 3
}

#[derive( Debug, Clone)]
struct Packet {
    packet_type: PacketType,
    packet_data: Data
}

#[derive( Clone, Copy)]
pub union Data {
    a: Position,
    b: Players,
    c: Actions
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
    data: [u8;MAX_BUFFER_SIZE],
    ptr: *mut u8,    //Pointer to buffer data
    size: usize,     //Size of buffer data bytes
    index: usize     //index of next data byte in memory
}

fn main() {
    //Creating a new buffer
    let mut buffer = Buffer::new();

    let socket = UdpSocket::bind("0.0.0.0:3000").expect("Could not bind the socket");

    let mut position = Position {
        x: 25,
        y: 25,
        z: 25,
    };

    let mut players = Players {
        num_elements: 7,
        elements: [10;MAX_ELEMENTS]
    };

    let mut i = 10;
    // let packet_c = Actions {
    //     fire_state: false,
    //     a: 10,
    //     b: 21
    // };

    // let pack_c_send = Packet {
    //     packet_type: PacketType::Actions,
    //     packet_data: Data { c: packet_c }
    // };
    // pack_c_send.to_buffer(&mut buffer);
    // println!("Data in the buffer {:?}", buffer.data);

    loop {
        let position_to_send = Packet {
            packet_data: Data{ a: position },
            packet_type: PacketType::Position
        };
        position_to_send.to_buffer(&mut buffer);
        println!("Data in the buffer {:?}", buffer.data);
    
        let players_to_send = Packet {
            packet_type: PacketType::Players,
            packet_data: Data { b: players }
        };
        players_to_send.to_buffer(&mut buffer);
        println!("Data in the buffer {:?}", buffer.data);
    
        let _ = socket.send_to(&buffer.data, "127.0.0.1:3000").expect("unable to send");

        position.x += 1;
        position.y += 1;
        position.z += 1;

        i += 1;
        players.elements.fill(i);

        thread::sleep(Duration::from_secs(2));
        buffer.reset();
    }
    
}


//Read/Write func. for packet A
impl ReadWrite for Position {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        // buffer.reset();
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

//Read/Write func. for packet B
impl ReadWrite for Players {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        if  self.num_elements <= MAX_BUFFER_SIZE {
            // buffer.reset();
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
            // buffer.index += i;
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

//Read/Write func. for packet C
impl ReadWrite for Actions {
    fn write_to_buffer(&self, buffer: &mut Buffer) {
        // buffer.reset();
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

pub trait ReadWrite {
    fn write_to_buffer(&self, _buffer: &mut Buffer){}

    fn read_from_buffer(_buffer: &mut Buffer)-> Data {
        let data = Data {
            a: Position { x: 0, y: 0, z: 0 }
        };
        data
    } 
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

    fn set_index(&mut self) {
        self.ptr = &mut self.data[0];
        self.index = 0;
    }

    fn reset(&mut self) {
        self.data.fill(0);
        self.ptr = &mut self.data[0];
        self.index = 0;
    }

    fn write_bytes(&mut self, value:u8) {
        unsafe
        {
            assert!(self.index + 1 <= self.size);
            let ptr  = self.ptr.add(self.index);
            *ptr = value;
            self.index += 1;
        }
    }

    fn read_bytes(&mut self) -> u8 {
        unsafe
        {
            let ptr = self.ptr.add(self.index);
            let value = *ptr;
            self.index += 1;
            value
        }
    }
}

//Packet Functionality
impl Packet {
    fn to_buffer(self, buffer: &mut Buffer) {
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

    fn packet_from_buffer(buffer: &mut Buffer) -> Option<Packet> {
        // buffer.set_index();
        let p_type = buffer.read_bytes();

        match p_type {
            1 => {
                let pack = Packet {
                    packet_type: PacketType::Position,
                    packet_data: Position::read_from_buffer(buffer)
                };
                // buffer.reset();
                Some(pack)
            }
            2 => {
                let pack = Packet {
                    packet_type: PacketType::Players,
                    packet_data: Players::read_from_buffer(buffer)
                };
                // buffer.reset();
                Some(pack)
            }
            3 => {
                let pack = Packet {
                    packet_type: PacketType::Actions,
                    packet_data: Actions::read_from_buffer(buffer)
                };
                // buffer.reset();
                Some(pack)
            }
            _ => {None}
        }
    }

    fn packets_from_buffer(buffer: &mut Buffer) -> Vec<Packet> {
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