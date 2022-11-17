// const F32_PRECISION_BITS: u32 = 13;

// const F32_R_SHIFT_POS: u32 = 32-F32_PRECISION_BITS;
// const F32_L_SHIFT_POS: u32 = 32-F32_PRECISION_BITS;
// const F32_PRECISION_BITS_OFFSET: u64 = F32_PRECISION_BITS as u64;
// const F32_UPPER_REM_BITS: u32 = 32-2*F32_PRECISION_BITS;
// const F32_UPPER_REM_POS: u32 = F32_PRECISION_BITS+F32_UPPER_REM_BITS;
// const F32_LOWER_REM_POS: u32 = 32-(F32_PRECISION_BITS-F32_UPPER_REM_BITS);

const INDEX_BITS: u32 = 2;
// const F32_INDEX_BITS_OFFSET: u64 = 3*F32_PRECISION_BITS as u64;
// const F32_INDEX_POS: u32 = 3*F32_PRECISION_BITS -32;
// const SIGN_BIT_POS: u64 = F32_INDEX_BITS_OFFSET + INDEX_BITS as u64;

pub fn quat_compression() {
    let mut quat: Vec<f32>= Vec::new();

    quat.push(-0.000000469);
    quat.push(-0.196);
    quat.push(-0.00000703);
    quat.push(-0.98);

    println!("{:?}", quat.iter()); 

    let mut to_drop = quat.first().unwrap().clone();
    let mut index = 0;
    let mut signed = false;

    // let mut quaternion = quat.iter_mut().map(|a| *a*1000.).collect::<Vec<f32>>();
    // println!("{:?}", quaternion.iter()); 


    for value in quat.clone().into_iter().enumerate() {
        
        if value.1 > 0. {
            if value.1 > to_drop {
                to_drop = value.1;
                index = value.0;
            }
        }

        if value.1 < 0. {
            if value.1 < to_drop {
                to_drop = value.1;
                index = value.0;
            }
        }
    }

    if to_drop < 0. {
        signed = true;
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

    let compressed = compress_quat(&quat, index, signed, 13);

    println!("\nCompressed: ");

    let bytes = compressed.to_be_bytes();
    
    for i in 2..bytes.len() {
        println!("{}", bytes[i]);    
    }

    let compressed_binary = compressed.to_be_bytes();
    for byte in compressed_binary.iter() {
        print!("{:b}", byte);    
    }
    // println!("{}", compressed_binary.len());
    // print!("\n");

    let double = u64::from_be_bytes(compressed_binary);
    let received_quat = uncompress_quat(double,13);
    println!("\nreceived quaternion: {:?}", received_quat.iter()); 

}

fn compress_quat(quaternion: &Vec<f32>, index: usize, sign: bool, precision_bits: u32) -> u64 {
    
    let mut double: u64 = 0;
    let mut i: u64 = 0;
    println!("{sign}");

    let f32_r_shift_pos: u32 = 32 - precision_bits;
    let f32_precision_bits_offset: u64 = precision_bits as u64;
    let f32_index_bits_offset = 3*precision_bits as u64;
    let sign_bit_pos = f32_index_bits_offset + INDEX_BITS as u64;

    for component in quaternion.iter() {
        let quat_comp_bytes = (*component).to_be_bytes();        
        let mut temp = u32::from_be_bytes(quat_comp_bytes);
        temp = temp >> f32_r_shift_pos;
        double |= (temp as u64) << i*f32_precision_bits_offset;
        i += 1;
    }

    if sign {
        double |= (index as u64) << f32_index_bits_offset | 1 << sign_bit_pos;
    }else {
        double |= (index as u64) << f32_index_bits_offset;
    }
    // for component in quaternion.iter() {
    //     let quat_comp_bytes = (*component).to_be_bytes();        
    //     let mut temp = u32::from_be_bytes(quat_comp_bytes);
    //     temp = temp >> F32_R_SHIFT_POS;
    //     double |= (temp as u64) << i*F32_PRECISION_BITS_OFFSET;
    //     i += 1;
    // }

    // if sign {
    //     double |= (index as u64) << F32_INDEX_BITS_OFFSET | 1 << SIGN_BIT_POS;
    // }else {
    //     double |= (index as u64) << F32_INDEX_BITS_OFFSET;
    // }
    double
}

fn uncompress_quat(double: u64, precision_bits: u32) -> Vec<f32> {
    
    let mut upper_half: u32 = 0;
    let mut lower_half: u32 = 0;
    let mut quat_comp1: u32 = 0;
    let mut quat_comp2: u32 = 0;
    let mut quat_comp3: u32 = 0;
    let mut index: u32 = 0x03;
    let mut sign: u32 = 0x04;

    let f32_r_shift_pos: u32 = 32 - precision_bits;
    let f32_l_shift_pos = 32 - precision_bits;
    let f32_precision_bits_offset: u64 = precision_bits as u64;
    let f32_upper_rem_bits = 32 - 2*precision_bits;
    let f32_upper_rem_pos = precision_bits + f32_upper_rem_bits;
    let f32_lower_rem_pos = 32 - (precision_bits-f32_upper_rem_bits);
    let f32_index_pos: u32 = 3*precision_bits -32;

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
    
    quat_comp1 |= upper_half << f32_l_shift_pos;

    upper_half &= !(quat_comp1 >> f32_r_shift_pos );
    upper_half = upper_half >> f32_precision_bits_offset;
    
    quat_comp2 |= upper_half << f32_l_shift_pos;
    
    upper_half &= !(quat_comp1 >> f32_r_shift_pos);
    upper_half = upper_half >> f32_precision_bits_offset;
    println!("{:b}", upper_half);

    quat_comp3 |= lower_half << f32_lower_rem_pos | upper_half << f32_upper_rem_pos;
    
    lower_half = lower_half >> f32_index_pos;
    // quat_comp1 |= upper_half << F32_L_SHIFT_POS;

    // upper_half &= !(quat_comp1 >> F32_R_SHIFT_POS );
    // upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    
    // quat_comp2 |= upper_half << F32_L_SHIFT_POS;
    
    // upper_half &= !(quat_comp1 >> F32_R_SHIFT_POS);
    // upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    // println!("{:b}", upper_half);

    // quat_comp3 |= lower_half << F32_LOWER_REM_POS | upper_half << F32_UPPER_REM_POS;
    
    // lower_half = lower_half >> F32_INDEX_POS;

    index &= lower_half;
    sign &= lower_half;

    println!("{index} {sign}");

    if index == 0 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let num = 1. - (w*w + y*y + z*z);
        let mut x = num.sqrt();
        
        if sign == 4 {
            x = -x;
        }
    
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
        let mut y = num.sqrt();

        if sign == 4 {
            y = -y;
        }
    
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
        let mut z = num.sqrt();
        
        if sign == 4 {
            z = -z;
        }

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

        let num = 1. - (x*x + z*z + y*y);
        let mut w = num.sqrt();
    
        if sign == 4 {
            w = -w;
        }

        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    quat
}

pub fn trans_compress(translation: [f32;3], precision_bits: u32)-> u64 {
    let mut double: u64 = 0;
    for i in 0..translation.len(){
        bitpack_f32(&mut double, translation[i], i as u64, precision_bits);
    }
    double
}

pub fn bitpack_f32 (double: &mut u64, mut num: f32, pos: u64, precision_bits: u32){
    let f32_r_shift_pos: u32 = 32 - precision_bits;
    let f32_precision_bits_offset: u64 = precision_bits as u64;
    let f32_index_bits_offset = 3*precision_bits as u64;
    let sign_bit_pos = f32_index_bits_offset + INDEX_BITS as u64;

    num = num*100.;
    let quat_comp_bytes = num.to_be_bytes();        
    let mut temp = u32::from_be_bytes(quat_comp_bytes);
    temp = temp >> f32_r_shift_pos;
    *double |= (temp as u64) << pos*f32_precision_bits_offset;
    // temp = temp >> F32_R_SHIFT_POS;
    // *double |= (temp as u64) << pos*F32_PRECISION_BITS_OFFSET;
}

pub fn uncompress_translation (double:u64, precision_bits: u32) -> [f32;3] {
    let mut upper_half: u32 = 0;
    let mut lower_half: u32 = 0;
    let mut comp1: u32 = 0;
    let mut comp2: u32 = 0;
    let mut comp3: u32 = 0;
    let mut vector: [f32;3] = [0.;3];

    let f32_r_shift_pos: u32 = 32 - precision_bits;
    let f32_l_shift_pos = 32 - precision_bits;
    let f32_precision_bits_offset: u64 = precision_bits as u64;
    let f32_upper_rem_bits = 32 - 2*precision_bits;
    let f32_upper_rem_pos = precision_bits + f32_upper_rem_bits;
    let f32_lower_rem_pos = 32 - (precision_bits-f32_upper_rem_bits);
    let f32_index_pos: u32 = 3*precision_bits -32;

    upper_half |= (double as u32 ) >> 0;
    lower_half |= (double  >> 32) as u32;

    comp1 |= upper_half << f32_l_shift_pos;

    upper_half &= !(comp1 >> f32_r_shift_pos );
    upper_half = upper_half >> f32_precision_bits_offset;
    
    comp2 |= upper_half << f32_l_shift_pos;
    
    upper_half &= !(comp1 >> f32_r_shift_pos);
    upper_half = upper_half >> f32_precision_bits_offset;
    println!("{:b}", upper_half);

    comp3 |= lower_half << f32_lower_rem_pos | upper_half << f32_upper_rem_pos;
    
    // comp1 |= upper_half << F32_L_SHIFT_POS;

    // upper_half &= !(comp1 >> F32_R_SHIFT_POS );
    // upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    
    // comp2 |= upper_half << F32_L_SHIFT_POS;
    
    // upper_half &= !(comp1 >> F32_R_SHIFT_POS);
    // upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    // println!("{:b}", upper_half);

    // comp3 |= lower_half << F32_LOWER_REM_POS | upper_half << F32_UPPER_REM_POS;

    let mut component_bytes = comp1.to_be_bytes();

    let x = f32::from_be_bytes(component_bytes);

    component_bytes = comp2.to_be_bytes();
    let y = f32::from_be_bytes(component_bytes);

    component_bytes = comp3.to_be_bytes();
    let z = f32::from_be_bytes(component_bytes);

    vector[0] = x/100.;
    vector[1] = y/100.;
    vector[2] = z/100.;

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn quat_compression_test() {
        quat_compression();

        let mut num: u32 = 0;
        println!("{:b}",num);
        num |= 7 << 0;
        println!("{:b}",num);

        num &= !(1<<1);
        println!("{:b}",num);
    }

    #[test]
    pub fn translation_compression_test() {
        let translation:[f32;3] = [-99.6, -88.41, 55.72];
        let compressed_translation = trans_compress(translation, 16);
        println!("Compressed Translation: ");
        println!("{:b}", compressed_translation);

        // for byte in compressed_translation.to_le_bytes().iter() {
        //     println!("{:b}",byte);
        // }

        let translation = uncompress_translation(compressed_translation,16);
        println!("{:?}",translation);
    }
}