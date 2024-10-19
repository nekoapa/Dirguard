extern crate crypto;

use clap::{Arg, Command};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn decrypt_data(
    data: &str,
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut input = File::open(data).unwrap();
    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();
    //  let mut output = File::create(output_file).unwrap();

    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&input_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn encrypt_data(input_file: &str, output_file: &str, key: &[u8]) {
    // 打开源文件
    let mut input = File::open(input_file).unwrap();
    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();

    // 打开输出文件
    let mut output = File::create(output_file).unwrap();

    // 初始化AES加密器
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        &[0; 16], // IV（初始化向量）应该是一个随机值，这里仅为示例
        blockmodes::PkcsPadding,
    );

    let mut read_buffer = buffer::RefReadBuffer::new(&input_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        output
            .write_all(write_buffer.take_read_buffer().take_remaining())
            .unwrap();

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
}

fn main() {
    let matches = Command::new("Rust Crypto Tool")
        .version("1.0")
        .author("Nyaaaww")
        .about("Encrypts and decrypts files using AES encryption")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to write to")
                .required(true),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_name("KEY")
                .help("Sets the encryption key")
                .required(true),
        )
        .arg(
            Arg::new("operation")
                //  .short('op')
                .long("op")
                .long("operation")
                .value_name("OPERATION")
                .help("Sets the operation to perform (en or de)")
                .required(true)
                .value_parser(["en", "de"]),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let key = matches.get_one::<String>("key").unwrap().as_bytes();
    let operation = matches.get_one::<String>("operation").unwrap();

    if !Path::new(input_file).exists() {
        eprintln!("The input file does not exist.");
        return;
    }

    match operation.as_str() {
        "en" => {
            encrypt_data(input_file, output_file, key);
            println!("Encryption completed.");
        }
        "de" => {
            match decrypt_data(input_file, key, &[0; 16]) {
                Ok(decrypted_data) => {
                    let mut file =
                        File::create(output_file).expect("Unable to create decrypted file");
                    file.write_all(&decrypted_data)
                        .expect("Unable to write decrypted data");
                }
                Err(e) => {
                    println!("Error decrypting file: {:?}", e);
                }
            }
            println!("Decryption completed.");
        }
        _ => {
            eprintln!("Invalid operation specified. Use 'encrypt' or 'decrypt'.");
        }
    }
}
