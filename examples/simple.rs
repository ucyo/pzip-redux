use pzip_redux;

fn main() {
    let data : Vec<u8> = vec![
        2,2,2,2,2,2,2,2,52,52,2,2,2,2,2,
        2,2,2,52,52,2,2,2,2,2,2,2,2,52,
        52,2,2,2,2,2,2,2,2,52,52,123
    ];
    let (symbol, freq, code) = (8, 10, 12);

    let encoded = pzip_redux::encode(&data, symbol, freq, code);
    let decoded = pzip_redux::decode(&encoded, symbol, freq, code);

    println!("+++ Adaptive Range Encoder +++")
    for val in encoded.iter() {
        println!("{:08b}", val)
    }
    println!("{:?} {}", encoded, encoded.len());
    println!("{:?} {}", decoded, decoded.len());
}
