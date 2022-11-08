
const QUAT_COMPONENT_RANGE: (f32,f32) = (-0.70717,0.70717);
const QUAT_PRECISION_BITS: u32 = 21;
const QUAT_PRECISION_BITS_OFFSET: u64 = 32-QUAT_PRECISION_BITS as u64;
const INDEX_BITS: u32 = 3;       //actually 3 from 0..2
const QUAT_INDEX_BITS_OFFSET: u64 = 3*QUAT_PRECISION_BITS_OFFSET + BIT_OFFSET as u64;
const BIT_OFFSET: u32 = 1;

pub fn quat_compression() {
    let mut quat: Vec<f32>= Vec::new();

    quat.push(0.5);
    quat.push(0.4);
    quat.push(0.632);
    quat.push(0.435);

    println!("{:?}", quat.iter()); 

    let mut largest = 0.;
    let mut index = 0;

    for value in quat.clone().into_iter().enumerate() {
        assert!(
            value.1 <= QUAT_COMPONENT_RANGE.1 &&
            value.1 >= QUAT_COMPONENT_RANGE.0,
            "Quat component: {}  at {} not in range -0.70717..0.70717",
            value.1,
            value.0
        );
        
        if value.1 > largest {
            largest = value.1;
            index = value.0;
        }
    }
    quat.remove(index);

    println!("Uncompressed: ");

    for component in quat.iter(){
        print!("{component}: ");
        let uncompressed_binary = component.to_be_bytes();
        for byte in uncompressed_binary.iter() {
            print!("{:b}", byte);    
        }
        print!("\n");
    }

    let compressed = compress_quat(&quat, index);

    println!("\nCompressed: ");

    let compressed_binary = compressed.to_be_bytes();
    for byte in compressed_binary.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");

    let received_quat = uncompress_quat(compressed);
    println!("\nreceived quaternion: {:?}", received_quat.iter()); 

}

fn compress_quat(quaternion: &Vec<f32>, index: usize) -> u64 {
    
    let mut double: u64 = 0;
    let mut i: u64 = 0;
    
    for component in quaternion.iter() {
        let mut temp: u32 = 0;
        let quat_comp_bytes = (*component).to_be_bytes();        
        temp = u32::from_be_bytes(quat_comp_bytes);
        temp = temp >> QUAT_PRECISION_BITS;
        double |= (temp as u64) << i*QUAT_PRECISION_BITS_OFFSET;
        i += 1;
    }
    double |= (index as u64) << QUAT_INDEX_BITS_OFFSET;
    double
}

fn uncompress_quat(double: u64) -> Vec<f32> {
    
    let mut upper_half: u32 = 0;
    let mut lower_half: u32 = 0;
    let mut quat_comp1: u32 = 0;
    let mut quat_comp2: u32 = 0;
    let mut quat_comp3: u32 = 0;
    let mut index: u32 = 0;

    let mut quat = Vec::new();

    upper_half |= (double as u32 ) >> 0;

    println!("\nUpper Half: ");

    let half_word = upper_half.to_be_bytes();
    for byte in half_word.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");

    
    println!("\nLower Half: ");
    
    lower_half |= (double  >> 32) as u32;
    
    let lower_half_bytes = lower_half.to_be_bytes();
    for byte in lower_half_bytes.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");
    
    quat_comp1 |= upper_half << QUAT_PRECISION_BITS;

    upper_half &= !(quat_comp1 >> QUAT_PRECISION_BITS);
    upper_half = upper_half >> QUAT_PRECISION_BITS_OFFSET;

    quat_comp2 |= upper_half << QUAT_PRECISION_BITS;

    upper_half &= !(quat_comp1 >> QUAT_PRECISION_BITS);
    upper_half = upper_half >> QUAT_PRECISION_BITS_OFFSET;

    quat_comp3 |= lower_half << 31 | upper_half << QUAT_PRECISION_BITS;
    
    lower_half = lower_half >> 2;

    index |= lower_half;

    if index == 0 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let num = 1. - (w*w + y*y + z*z);
        let x = num.sqrt();
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 1 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let num = 1. - (x*x + z*z + w*w);
        let y = num.sqrt();
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 2 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let num = 1. - (x*x + y*y + w*w);
        let z = num.sqrt();
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 3 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        let num = 1. - (x*x + y*y + z*z);
        let w = num.sqrt();
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    quat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn quat_compression_test() {
        quat_compression();
    }
}