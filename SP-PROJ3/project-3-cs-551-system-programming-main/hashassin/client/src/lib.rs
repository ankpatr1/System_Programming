use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn upload(server: &str, in_file: &str, name: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(server)?;
    let payload = fs::read(in_file)?;
    let name_bytes = name.as_bytes();

    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"upload"); // 6 bytes
    buffer.push(1); // version
    buffer.push(name_bytes.len() as u8); // name length
    buffer.extend_from_slice(name_bytes); // name
    buffer.extend_from_slice(&(payload.len() as u64).to_be_bytes()); // payload size
    buffer.extend_from_slice(&payload); // actual file

    stream.write_all(&buffer)?;
    Ok(())
}

pub fn crack(server: &str, in_file: &str) -> std::io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect(server)?;
    let payload = fs::read(in_file)?;

    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"crack\0"); // 6 bytes: includes null terminator
    buffer.push(1); // version
    buffer.extend_from_slice(&(payload.len() as u64).to_be_bytes()); // payload size
    buffer.extend_from_slice(&payload); // hash file

    stream.write_all(&buffer)?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    Ok(response)
}
