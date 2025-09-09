use std::env;
#[derive(Debug)]

struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

fn format_size(size: u64, unit: &str) -> Sizes {
    let unit = unit.to_lowercase();

    let bytes = match unit.as_str() {
        "b" => size,
        "kb" => size * 1000,
        "mb" => size * 1000000,
        "gb" => size * 1000000000,
        _ => panic!("Unknown Unit!"),
    };
    let kbs = bytes / 1000;
    let mbs = bytes / 1000000;
    let gbs = bytes / 1000000000;
    Sizes {
        bytes: format!("{} bytes", bytes),
        kilobytes: format!("{} bytes", bytes),
        megabytes: format!("{} bytes", bytes),
        gigabytes: format!("{} bytes", bytes),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputarg: &str = &args[1];
    println!("Size is: {}.", inputarg);
    let parts: Vec<&str> = inputarg.split_whitespace().collect();
    let num: u64 = parts[0]
        .parse()
        .expect("Argument should be <number> <unit>");
    let unit: &str = parts[1];
    let result = format_size(num, unit);
    println!("{:?}", result)
}
