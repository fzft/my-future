use lazy_static::lazy_static;
use std::error::Error;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    io::Read,
    net::{AddrParseError, SocketAddr, TcpListener, TcpStream},
    os::fd::AsRawFd,
    str::FromStr,
};

// brokracbiodihear@mail.com
// uR8PegVFjB
use clap::{Arg, Parser};

use crate::{executor::Executor, poll::EventId};

mod executor;
mod poll;
mod reactor;

lazy_static! {
    static ref EXECUTOR: Mutex<executor::Executor> = Mutex::new(executor::Executor::new());
    static ref REACTOR: Mutex<reactor::Reactor> = Mutex::new(reactor::Reactor::new());
    static ref CONTEXTS: Mutex<HashMap<EventId, Request>> = Mutex::new(HashMap::new());
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::from_args()?;
    let listener = TcpListener::bind(url)?;
    listener.set_nonblocking(true)?;

    println!("server listen on {}", url);

    let listener_fd = listener.as_raw_fd();

    let (sender, receiver) = std::sync::mpsc::channel();

    let _ = match REACTOR.lock() {
        Ok(mut re) => re.run(sender),
        Err(e) => panic!("error running reactor, {}", e),
    };

    REACTOR
        .lock()
        .expect("get locker")
        .read_interest(listener_fd, 100)?;

    listener_cb(listener, 100);

    while let Ok(event_id) = receiver.recv() {
        println!("recv {}", event_id);
        EXECUTOR
            .lock()
            .expect("get executor lock")
            .run(event_id as EventId)
    }

    Ok(())
}

fn listener_cb(listener: TcpListener, event_id: EventId) {
    let mut exec_locker = EXECUTOR.lock().expect("get executor locker");
    exec_locker.await_keep(event_id, move |exec| match listener.accept() {
        Ok((stream, addr)) => {
            let event_id: EventId = rand::random();
            stream.set_nonblocking(true).expect("set non blocking");
            println!(
                "new client: {}, new event id: {}, new raw_fd: {}",
                addr,
                event_id,
                stream.as_raw_fd()
            );
            REACTOR
                .lock()
                .expect("get reactor locker")
                .read_interest(stream.as_raw_fd(), event_id)
                .expect("can set read interest");
            CONTEXTS
                .lock()
                .expect("get context locker")
                .insert(event_id, Request::new(stream));
            exec.await_keep(event_id, move |_| {
                if let Some(r) = CONTEXTS
                    .lock()
                    .expect("get context locker")
                    .get_mut(&event_id)
                {
                    r.read_cb()
                }
            })
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
        Err(e) => panic!("{}", e),
    })
}

struct Request {
    stream: TcpStream,
    buffer: [u8; 4096],
}

impl Request {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: [0u8; 4096],
        }
    }

    fn read_cb(&mut self) {
        loop {
            match self.stream.read(&mut self.buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    if let Ok(data) = std::str::from_utf8(&self.buffer[..n]) {
                        println!("get data: {}", data);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => panic!("read cb get error: {}", e)
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Url {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value = "8000")]
    port: String,
}

impl Url {
    fn from_args() -> Result<SocketAddr, AddrParseError> {
        let url = Url::parse();
        let addr = format!("{}:{}", url.ip, url.port);
        SocketAddr::from_str(&addr)
    }
}
