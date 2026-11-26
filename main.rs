use std::process::exit;


fn bigint_init(input: &str) -> Vec<String> {
    if input.is_empty() {
        eprintln!("input is empty");
        exit(1);
    }

    for character in input.chars() {
        if !character.is_ascii_hexdigit() {
            eprintln!("input contains non-hex character");
            exit(1);
        }
    }

    if input.len() > 512 {
        eprintln!("input greater than 2048 bits");
        exit(1);
    }

    let mut hex_digits: Vec<char> = input.chars().collect();
    if hex_digits.len() % 2 != 0 {
        hex_digits.insert(0, '0');
    }

    //vector to store the hex bytes
    let mut hex_bytes = Vec::new();
    let mut index = 0;

    while index < hex_digits.len() {
        //does not store 0x
        if hex_digits[index] != '0' || hex_digits[index+1] != 'x'{
            let byte = format!("{}{}", hex_digits[index], hex_digits[index + 1]);
            hex_bytes.push(byte);
        }
        index += 2;
    }
    //make least to most significant digit order
    hex_bytes.reverse();
    return hex_bytes
}

fn bigint_to_string(input: &[String]) -> String {
    let mut result = String::new();
    let mut index = input.len();
    let mut all_zero = true;
    while index > 0 {
        index -= 1;
        let byte = &input[index];

        if !byte.starts_with("00"){
            all_zero = false;
        }

        if byte.starts_with("0x") {
            //omit '0x'
        } else if all_zero && byte.starts_with("00"){
            // omit leading zeros
        } else {
            result.push_str(byte);
        }
    }
    if all_zero{
        return "0".to_string();
    } else{
        return result.to_lowercase();
    }
}

fn bigint_multiply(input1: &[String], input2: &[String]) -> Result<Vec<String>, String> {
    //convert hex to u16
    let mut digits1 = Vec::new();
    for byte in input1 {
        let digit = u8::from_str_radix(byte, 16).map_err(|error| error.to_string())?;
        digits1.push(u16::from(digit));
    }

    //turn hex to u16
    let mut digits2 = Vec::new();
    for byte in input2 {
        let digit = u8::from_str_radix(byte, 16).map_err(|error| error.to_string())?;
        digits2.push(u16::from(digit));
    }

    let mut result = multiply_hex(&digits1, &digits2);
    while result.last() == Some(&0) {
        result.pop();
    }

    if result.len() > 256 {
        return Err("result greater than 2048 bits".to_string());
    }
    //convert back to hex
    let mut result_hex = Vec::new();
    for byte in result {
        result_hex.push(format!("{byte:02x}"));
    }
    Ok(result_hex)
}

fn multiply_hex(bigint1: &[u16], bigint2: &[u16]) -> Vec<u16> {
    let length = bigint1.len() + bigint2.len();
    let mut result = vec![0; length];

    for i in 0..bigint1.len() {
        let mut carry = 0;
        for j in 0..bigint2.len() {
            let k = i + j;
            let value = (bigint1[i] * bigint2[j]) + result[k] + carry;
            // store the new byte
            result[k] = value % 256; 
            //get the carry
            carry = value / 256; 
        }
        result[i + bigint2.len()] = carry;
    }
    return result;
}

fn main() {
    println!("main ran!")
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
