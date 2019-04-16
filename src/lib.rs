use redux::model::*;

pub fn encode(data: &Vec<u8>, symbol: usize, frequency: usize, code: usize) -> Vec<u8> {
    let mut cursor = std::io::Cursor::new(data);
    let mut result : Vec<u8> = Vec::new();

    let params = Parameters::new(symbol, frequency, code).unwrap();
    let model = AdaptiveTreeModel::new(params);

    let size = redux::compress(&mut cursor, &mut result, model).unwrap();
    // println!("{:?}", size);
    result
}

pub fn decode(data: &Vec<u8>,  symbol: usize, frequency: usize, code: usize) -> Vec<u8> {
    let mut cursor = std::io::Cursor::new(data);
    let mut result : Vec<u8> = Vec::new();

    let params = Parameters::new(symbol, frequency, code).unwrap();
    let model = AdaptiveTreeModel::new(params);

    let size = redux::decompress(&mut cursor, &mut result, model).unwrap();
    // println!("{:?}", size);
    result
}
