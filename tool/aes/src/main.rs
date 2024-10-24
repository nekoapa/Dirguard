extern crate crypto;

use clap::ArgAction;
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
    //    let mut output = File::create(output_file).unwrap();

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
        /*   output
        .write_all(write_buffer.take_read_buffer().take_remaining())
        .unwrap();*/
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
        .subcommand_required(true)
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file to decrypt")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file for decrypted data")
                        .required(true),
                )
                .arg(
                    Arg::new("key")
                        .short('k')
                        .long("key")
                        .value_name("KEY")
                        .help("Sets the encryption key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file to encrypt")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file for encrypted data")
                        .required(true),
                )
                .arg(
                    Arg::new("key")
                        .short('k')
                        .long("key")
                        .value_name("KEY")
                        .help("Sets the encryption key")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("decrypt", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let output = sub_matches.get_one::<String>("output").unwrap();
            let key = sub_matches.get_one::<String>("key").unwrap().as_bytes();
            match decrypt_data(input, key, &[0; 16]) {
                Ok(decrypted_data) => {
                    let mut file = File::create(output).expect("Unable to create decrypted file");
                    file.write_all(&decrypted_data)
                        .expect("Unable to write decrypted data");
                }
                Err(e) => {
                    println!("Error decrypting file: {:?}", e);
                }
            }
            println!("Decryption completed.");
        }
        Some(("encrypt", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let output = sub_matches.get_one::<String>("output").unwrap();
            let key = sub_matches.get_one::<String>("key").unwrap().as_bytes();
            encrypt_data(input, output, key);
            println!("Encryption completed.");
        }
        _ => unreachable!(), // clap will handle incorrect usage if subcommand_required is set
    }
}
