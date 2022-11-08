
const QUAT_COMPONENT_RANGE: (f32,f32) = (-0.70717,0.70717);
const QUAT_PRECISION_BITS: u32 = 21;
const QUAT_PRECISION_BITS_OFFSET: u64 = 32-QUAT_PRECISION_BITS as u64;
const INDEX_BITS: u32 = 3; //actually 3 from 0..2
const QUAT_INDEX_BITS_OFFSET: u64 = 3*QUAT_PRECISION_BITS_OFFSET + BIT_OFFSET as u64;
const BIT_OFFSET: u32 = 1;
const COMPRESSED_QUAT_BITS: u64 = 60;

pub fn quat_compression() {
    let mut quat: Vec<f32>= Vec::new();

    quat.push(0.2);
    quat.push(0.4);
    quat.push(0.435);
    quat.push(0.5);

    println!("{:?}", quat.iter()); 
    println!("Total size of original quat: {} bytes\n", 4*quat.len());

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
    println!("largest quat component {largest} at index {index}\n");
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
    println!("Total size after largest value dropped: {} bytes", 4*quat.len());

    let compressed = compress_quat(&quat, index);

    println!("\nCompressed: ");

    let compressed_binary = compressed.to_be_bytes();
    for byte in compressed_binary.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");
    println!("Total size after quat compression: {} bytes", compressed_binary.len());

    uncompress_quat(compressed);

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

fn uncompress_quat(mut double: u64) {
    
    let mut upper_half: u32 = 0;
    let mut lower_half: u32 = 0;
    let mut quat_comp1: u32 = 0;
    let mut quat_comp2: u32 = 0;
    let mut quat_comp3: u32 = 0;
    let mut index: u32 = 0;

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
    println!("{:b}", quat_comp1);

    upper_half &= !(quat_comp1 >> QUAT_PRECISION_BITS);
    upper_half = upper_half >> QUAT_PRECISION_BITS_OFFSET;
    println!("{:b}", upper_half);

    quat_comp2 |= upper_half << QUAT_PRECISION_BITS;
    println!("{:b}", quat_comp2);

    upper_half &= !(quat_comp1 >> QUAT_PRECISION_BITS);
    upper_half = upper_half >> QUAT_PRECISION_BITS_OFFSET;
    println!("{:b}", upper_half);

    quat_comp3 |= lower_half << 31 | upper_half << QUAT_PRECISION_BITS;
    println!("{:b}", quat_comp3);
    
    lower_half = lower_half >> 2;
    println!("{:b}", lower_half);

    index |= lower_half;
    println!("{}", index);

    lower_half = lower_half >> INDEX_BITS;
    println!("{:b}", lower_half);

    if index == 3 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);
        println!("{}",x);

        component_bytes = quat_comp2.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);
        println!("{}",y);

        component_bytes = quat_comp3.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);
        println!("{}",z);

        let num = 1. - (x*x + y*y + z*z);
        let w = num.sqrt();
        println!("{}",w);

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn quat_compression_test() {
        quat_compression();
    }
}