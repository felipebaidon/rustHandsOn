use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn Unformat_size(size: u64, sizeSuffix: string) -> u64
{
    let multiplier = match sizeSuffix {
        "KB"=> 1000,
        "MB" => 1_000_000,
        "GB" => 1_000_000_000,
        _=> 1,
    };

    let storage = size * multiplier;

}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn main() {
    let result = format_size(6888837399);
    println!("{}", result);

    let args: Vec<String> = env::args().collect();

    let number = args[1].parse::<i32>().unwrap(); 
    //ToDO: add exception handling 
    let sufix = args[2];

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("{} {}", args[1], args[2]);
}