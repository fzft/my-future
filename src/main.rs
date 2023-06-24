use lazy_static::lazy_static;
use std::{net::{TcpListener, SocketAddr, AddrParseError, TcpStream}, str::FromStr, os::fd::AsRawFd, io::Read,};
use std::sync::Mutex;
use std::error::Error;

// brokracbiodihear@mail.com
// uR8PegVFjB
use clap::{Parser, Arg};

mod executor;
mod reactor;
mod poll;

lazy_static! {
    static ref EXECUTOR: Mutex<executor::Executor> = Mutex::new(executor::Executor::new());
    static ref REACTOR: Mutex<reactor::Reactor> = Mutex::new(reactor::Reactor::new());
}

fn main()-> Result<(), Box<dyn Error>> {
    let url = Url::from_args()?;
    let listener = TcpListener::bind(url)?;
    listener.set_nonblocking(true)?;

    let listener_fd = listener.as_raw_fd();

    let (sender, receiver) = std::sync::mpsc::channel();
    
    let _ = match REACTOR.lock() {
        Ok(mut re) => re.run(sender),
        Err(e) => panic!("error running reactor, {}", e),
    };

    REACTOR.lock().expect("get locker").read_interest(listener_fd)?;
    match listener.accept() {
        Ok((stream, addr)) => {
            stream.set_nonblocking(true)?;
            REACTOR.lock().expect("get locker").read_interest(stream.as_raw_fd())?;
            
        },
        Err(e) => panic!("could not accept client: {}", e)

    }

    let r = Request::new(stream)

    while let Ok(_) = receiver.recv() {
        
    }

struct Request {
    stream: TcpStream,
}


impl Request {

    fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    fn read_cb(&mut self) {
        let mut buf = [0u8;4096];
        match self.stream.read(&mut buf) {
            Ok(_) => {
                if let Ok(data) = std::str::from_utf8(&buf) {
                    println!("get data: {}", data);
                }
            },
            Err(e) => {}
        }
        
    }
}


    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Url {

    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value = "8000" )]
    port: String
}

impl Url {
    fn from_args() -> Result<SocketAddr, AddrParseError> {
        let url = Url::parse();
        let addr = format!("{}:{}", url.ip, url.port);
        SocketAddr::from_str(&addr)
    }
}


