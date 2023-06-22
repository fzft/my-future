use std::{net::{TcpListener, SocketAddr}, str::FromStr, os::fd::AsRawFd,};
use anyhow::{Result, Ok};

// brokracbiodihear@mail.com
// uR8PegVFjB
use clap::{Parser, Arg};

fn main()-> Result<()> {
    let url = Url::from_args()?;
    let listener = TcpListener::bind(url)?;
    listener.set_nonblocking(true)?;

    let listener_fd = listener.as_raw_fd();
    let epoll_fd = unsafe {
        libc::epoll_create1(0)
    }; 

    




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
    fn from_args() -> Result<SocketAddr> {
        let url = Url::parse();
        let addr = format!("{}:{}", url.ip, url.port);
        SocketAddr::from_str(&addr).map_err(anyhow::Error::new)
    }
}


