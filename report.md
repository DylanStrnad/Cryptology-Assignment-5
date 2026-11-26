Dylan Strnad

# Representation
I used a vector. The reason: I can create vectors with dynamic size, 
which is not possible for arrays in rust. Additionally, I store each hex byte as a string element in the vector. This is easier to understand than storing the decimal value, and useful for debugging.
# Multiplication Algorithm
* This algorithm converts the bigint hex vector into decimal form. It makes a new vector of decimal bytes. It passes the new decimal vectors to multiply_hex, and performs mulitplication of the bytes.
* multiply_hex multiplies each element in bigint1 vector with each element in bigint2 vector. It stores the new byte in the result vector. To do this, a carry bit must be created to store any overflow. k determines the position to store the new byte. Value stores the product of bigint1[i] and bigint2[j] + result at k + the carry. 
Result can only store byte elements, so it must perform value % 256 to store in the result byte vector. After, the carry is calculated by dividing by 256. This loops until the last element in bigint2 is visited, and then the carry is stored in the result vector. The length of the result vector is the length of bigint 1 + length of bigint 2. This ensures its big enough to store the result of the multiplicaiton.
# Testing and Performance Results
* Bigint initialization successful
* bigint_to_string successful
* Multiplication of bigints successful
* Correctly detects when mulitplication exceeds 2048 bits
* Detects and removes leading zeros
## My algorithms performance:
Performs mutliplcation of hex bytes 30 times, and takes the average time.
This uses cargo test --release, which is much faster than cargo test. 
* 256-bit multiplication takes: 5.049µs
* 512-bit multiplication takes: 14.101µs
* 1024-bit multiplcation takes:  39.552µs

## Pythons performance:
 Performs multiplication of hex bytes 30 times and takes the average time.
 * 256-bit multiplication takes: 3.688µs 
 * 512-bit multiplication takes: 3.709µs
 * 1024-bit multiplication takes: 5.308µs

Overall python is faster. Even though rust is a faster language, the algorithm used for bigint multiplication in python is more efficient than my algorithm. My algorithm takes about 3 times longer when the bit size is doubled. Python is only marginally slower when bit size if double, or even quadrupled.

