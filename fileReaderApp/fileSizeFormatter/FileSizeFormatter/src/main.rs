use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn Unformat_size(size: u64, sizeSuffix: &str) -> u64
{
        let storage = match sizeSuffix.to_lowercase().as_str() {
        "kb"=> size*1000,
        "mb"=> size*1_000_000,
        "gb"=> size*1_000_000_000,
        _=> size,
    };

    storage
}

fn format_size(filesize: &FileSize) -> String {
    match filesize {
        FileSize::Bytes(bytes) => format!("bytes: {} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("kilobytes: {} kilobytes", kb),
        FileSize::Megabytes(mb) => format!("megabytes: {} megabytes", mb),
        FileSize::Gigabytes(gb) => format!("gigabytes: {} gigabytes", gb),
    }
}

fn all_sizes(size: u64) -> Vec<FileSize> {
    vec![
        FileSize::Bytes(size),
        FileSize::Kilobytes(size as f64 / 1_000.0),
        FileSize::Megabytes(size as f64 / 1_000_000.0),
        FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    ]
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let number = args[1].parse::<u64>().unwrap(); 
    //ToDO: add exception handling 
    let sufix = &args[2];

    let resultBytes = Unformat_size(number, sufix);

    let result_list = all_sizes(resultBytes);

    for fs in &result_list {
        println!("{}", format_size(fs));
    }

}