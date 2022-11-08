use std::ptr;

pub const MAX_BUFFER_SIZE: usize = 1200;
pub const F32_SIZE: usize = 4;
pub const I32_SIZE: usize = 4;

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

    pub fn write_byte(&mut self, value:u8) {
        unsafe
        {
            assert!(self.index + 1 <= self.size);
            let ptr  = self.ptr.add(self.index);
            *ptr = value;
            self.index += 1;
        }
    }

    pub fn read_byte(&mut self) -> u8 {
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
            let value = self.read_byte();
            slice[i] = value;
            i += 1;
            len -= 1;
        }
        (slice, i)
    }

    pub fn get_vec(&mut self, mut len: u8)-> Vec<usize> {

        let mut vec = Vec::new();
        while len > 0 {
            let value = self.read_byte() as usize;
            vec.push(value);
            len -= 1;
        }
        vec
    }

    pub fn write_f32 (&mut self, value: f32) {

        let f32_byte_array = value.to_le_bytes();
        for byte in f32_byte_array.iter() {
            self.write_byte(*byte);
        }
    }

    pub fn read_f32 (&mut self) ->f32 {
        let mut f32_byte_array = [0; F32_SIZE];
        
        for i in 0..F32_SIZE {
            f32_byte_array[i] = self.read_byte()
        }
        let value = f32::from_le_bytes(f32_byte_array);
        value
    }

    pub fn write_i32 (&mut self, value: i32) {

        let i32_byte_array = value.to_le_bytes();
        for byte in i32_byte_array.iter() {
            self.write_byte(*byte);
        }
    }

    pub fn read_i32 (&mut self) ->i32 {

        let mut i32_byte_array = [0; I32_SIZE];
        for i in 0..I32_SIZE {
            i32_byte_array[i] = self.read_byte()
        }
        let value = i32::from_le_bytes(i32_byte_array);
        value
    }
    
    pub fn write_string (&mut self, string: String) {
        
        let str_bytes_len = string.len() as u8;
        self.write_byte(str_bytes_len);

        let str_byte_array = string.as_bytes();
        for byte in str_byte_array.iter() {
            self.write_byte(*byte);
        }
    }

    pub fn read_string (&mut self) -> String {
        
        let str_bytes_len = self.read_byte() as usize;

        let mut str_byte_vec = Vec::new();
        for _ in 0..str_bytes_len {
            let byte = self.read_byte();
            str_byte_vec.push(byte);
        }
        let string = String::from_utf8(str_byte_vec).unwrap();
        string
    }
}