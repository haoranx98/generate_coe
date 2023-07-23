use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};

fn read_4bytes_and_write(input_file: &mut File, output_file: &mut File, flag: bool) -> Result<()> {
    let mut buffer = [0u8; 4];
    let bytes_read = input_file.read(&mut buffer)?;
    let mut content = buffer.to_vec();

    match bytes_read {
        0 => {
            content[0] = 0;
            content[1] = 0;
            content[2] = 0;
            content[3] = 0;
        }
        1 => {
            content[1] = 0;
            content[2] = 0;
            content[3] = 0;
        }
        2 => {
            content[2] = 0;
            content[3] = 0;
        }
        3 => {
            content[2] = 0;
            content[3] = 0;
        }
        4 => {}
        _ => {}
    }

    let hex_string = content
        .iter()
        .rev()
        .map(|num| format!("{:02x}", num))
        .collect::<Vec<String>>()
        .join("");
    output_file.write_all(hex_string.as_bytes())?;
    if flag == true {
        output_file.write_all(";".as_bytes())?;
    } else {
        output_file.write_all(",".as_bytes())?;
        output_file.write_all(b"\n")?;
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 {
        let depth: i32 = args[3].parse().expect("Failed to parse");
        match File::open(&args[1]) {
            Ok(mut input) => match File::create(&args[2]) {
                Ok(mut output) => {
                    let _ = output.write_all("memory_initialization_radix = 16;".as_bytes());
                    let _ = output.write_all(b"\n");
                    let _ = output.write_all("memory_initialization_vector =".as_bytes());
                    let _ = output.write_all(b"\n");
                    for num in 1..depth + 1 {
                        if num == depth {
                            match read_4bytes_and_write(&mut input, &mut output, true) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error reading/writing file: {}", e);
                                    break;
                                }
                            }
                        } else {
                            match read_4bytes_and_write(&mut input, &mut output, false) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error reading/writing file: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error opening input file: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error opening input file: {}", e);
            }
        }
    }
}
