use std::env;
use std::io::Read;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::RawFd;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let socket_fd: RawFd = args[1].parse().unwrap();
    println!("[Rust] Connecting to fd socket: {}", socket_fd);

    unsafe {
        let mut socket = UnixStream::from_raw_fd(socket_fd);
        let mut buffer = String::new();
        socket.write_fmt(format_args!("Hello parent"))?;
        socket.read_to_string(&mut buffer)?;
        println!("[Rust] Read: {}", buffer);
    }

    Ok(())
}
