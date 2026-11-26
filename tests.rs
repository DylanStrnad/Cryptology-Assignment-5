use super::{bigint_init, bigint_multiply, bigint_to_string, multiply_hex};
use std::time::Instant;
use std::time::Duration;



fn initialize_and_multiply(input1: &str, input2: &str) -> Vec<String> {
    //create bigints
    let bigint_1 = bigint_init(input1);
    let bigint_2 = bigint_init(input2);
    //multiply
    let result = bigint_multiply(&bigint_1, &bigint_2);
    //return bigint vector
    return result.unwrap()
}

#[test]
fn test_bigint_init(){
    let input = "ab54a9";
    let bigint = bigint_init(&input);
    assert_eq!(bigint, vec!["a9", "54", "ab"]);
}

#[test]
fn test_bigint_to_string(){
    let input = "ab54a9";
    let bigint = bigint_init(&input);
    let string = bigint_to_string(&bigint);
    assert_eq!(string, "ab54a9");
}

#[test]
fn test_zero_equals_0(){
    let input = "0";
    let bigint = bigint_init(&input);
    let string = bigint_to_string(&bigint);
    assert_eq!(string, "0");

    let input = "00000000";
    let bigint = bigint_init(&input);
    let string = bigint_to_string(&bigint);
    assert_eq!(string, "0");
}

#[test]
fn test_bigint_to_string_removes_leading_zeros(){
    let input = "00000000ab54a9";
    let bigint = bigint_init(&input);
    let string = bigint_to_string(&bigint);
    assert_eq!(string, "ab54a9");
}

#[test]
fn test_multiplication() {
    //multiply bigints, convert to string, validate results
    assert_eq!(bigint_to_string(&initialize_and_multiply("ab54", "a98c")), "717825f0");
    assert_eq!(bigint_to_string(&initialize_and_multiply("0000ab54", "a98c")), "717825f0");
    assert_eq!(bigint_to_string(&initialize_and_multiply("ab54", "00a98c")), "717825f0");
    assert_eq!(bigint_to_string(&initialize_and_multiply("ab54a98ceb1f0ad2", "ab54a98ceb1f0ad2")), "72aa3681f6e120d4a3ff1b512b511444");
    assert_eq!(bigint_to_string(&initialize_and_multiply("ab54a98ceb1f0ad2ab54a98ceb1f0ad2", "ab54a98ceb1f0ad2ab54a98ceb1f0ad2")), "72aa3681f6e120d589538855191355edbaa86d244d83495ca3ff1b512b511444");
    assert_eq!(bigint_to_string(&initialize_and_multiply("123456789abcdef0", "fedcba9876543210")), "121fa00ad77d7422236d88fe5618cf00");
    assert_eq!(bigint_to_string(&initialize_and_multiply("123456789abcdef0123456789abcdef0", "fedcba9876543210fedcba9876543210")), "121fa00ad77d742247acc9140513b74458fab20783af1222236d88fe5618cf00");
}

#[test]
fn test_multiplication_exceeds_2048_bits() {
    let bigint_a = bigint_init("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2");
    let bigint_b = bigint_init("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2");
    assert_eq!(
        bigint_multiply(&bigint_a, &bigint_b),
        Err("result greater than 2048 bits".to_string())
    );
}

#[test]
fn test_multiplication_performance_256_bit_operands() {
    let input1 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let input2 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let mut digits1 = Vec::new();
    for byte in input1 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits1.push(u16::from(digit));
    }

    //turn hex to u16
    let mut digits2 = Vec::new();
    for byte in input2 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits2.push(u16::from(digit));
    }
    let mut total_time = Duration::ZERO;
    for _i in 0..30 {
        let start = Instant::now();
        multiply_hex(&digits1, &digits2);
        total_time += start.elapsed();
    }
    let average = total_time/30;
    println!("Average execution time 256 bit: {:?}", average);
}

#[test]
fn test_multiplication_performance_512_bit_operands() {
    let input1 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let input2 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let mut digits1 = Vec::new();
    for byte in input1 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits1.push(u16::from(digit));
    }

    //turn hex to u16
    let mut digits2 = Vec::new();
    for byte in input2 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits2.push(u16::from(digit));
    }
    let mut total_time = Duration::ZERO;
    for _i in 0..30{
        let start = Instant::now();
        multiply_hex(&digits1, &digits2);
        total_time += start.elapsed();
    }
    let average = total_time/30;
    println!("Average execution time 512 bit: {:?}", average);
}

#[test]
fn test_multiplication_performance_1024_bit_operands() {
    let input1 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let input2 = bigint_init(&String::from("ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"));
    let mut digits1 = Vec::new();
    for byte in input1 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits1.push(u16::from(digit));
    }

    //turn hex to u16
    let mut digits2 = Vec::new();
    for byte in input2 {
        let digit = u8::from_str_radix(&byte, 16).expect("invalid hex");
        digits2.push(u16::from(digit));
    }

    let mut total_time = Duration::ZERO;
    for _i in 0..30{
        let start = Instant::now();
        multiply_hex(&digits1, &digits2);
        total_time += start.elapsed();
    }
    let average = total_time/30;
    println!("Average execution time 1024 bit: {:?}", average);
}