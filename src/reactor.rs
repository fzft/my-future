use std::os::fd::RawFd;
use std::io::Error;

use crate::poll::{Registry, Poll};

pub struct Reactor {
    registry: Option<Registry> ,
}

impl Reactor {

    pub fn new() -> Self {
        Self { registry: None }
    }

    pub fn run(&mut self, sender: std::sync::mpsc::Sender<u64>) -> Result<(), Error> {
        let poller = Poll::create_epoll()?;
        let registry = Registry::new(poller.get_fd());
        self.registry = Some(registry);

        let mut events: Vec<libc::epoll_event> = Vec::with_capacity(1024);

        std::thread::spawn(move||{
            loop {
                poller.poll(&mut events);
                for e in &events {
                    sender.send(e.u64 ).expect("channel works");
                }
            }
        });

        Ok(())
    }


    // Add read event to poll with fd
    pub fn read_interest(&mut self, fd: RawFd) -> Result<(), Error> {
        self.registry.as_mut().unwrap().register_read(fd)
    }

    // Add write event to poll with fd
    pub fn write_interest(&mut self, fd: RawFd) -> Result<(), Error> {
        self.registry.as_mut().unwrap().register_write(fd)
    }

    // Remove the event with fd
    pub fn close(&mut self, fd: RawFd) -> Result<(), Error> {
        self.registry.as_mut().unwrap().remove_interests(fd)
    }
}