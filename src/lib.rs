use std::{os::fd::RawFd, io::Error, collections::{HashMap, HashSet}};


#[repr(i32)]
pub enum EpollControlOpts {
    Add = libc::EPOLL_CTL_ADD,
    Mod = libc::EPOLL_CTL_MOD,
    Del = libc::EPOLL_CTL_DEL
}

pub enum Interest {
    Readable,
    Writeable
}

pub struct Epoll {
    epoll_fd: i32,
}

pub trait Source {
    
    fn register(&self, i: Interest) ;

    fn deregister(&self, i: Interest);
}

impl Epoll {
    pub fn create_epoll() -> Result<Self, Error> {
        let epoll_fd = unsafe {
            libc::epoll_create1(0)
        }; 
        let flags: i32 = unsafe { libc::fcntl(epoll_fd, libc::F_GETFD) };
        if flags == -1 {
            return Err(Error::last_os_error());
        }

        let new_flags = flags | libc::FD_CLOEXEC;
        if unsafe { libc::fcntl(epoll_fd, libc::F_SETFD, new_flags) } == -1 {
            return Err(Error::last_os_error());
        }
        Ok(Epoll{ epoll_fd})
    }
}


pub struct Registry {
    epoll_fd: i32,
    source: HashMap<RawFd, HashSet<Interest>>
}

impl Registry {
    fn new(epoll_fd: i32) -> Self {
        Self { epoll_fd, source: HashMap::new() }
    }

    fn register_read(&self) {}

    fn register_write(&self) {}

    fn remove_interests(&self) {}
}

pub struct Reactor {
    registry: Registry
    
}


impl Reactor {

    // Add read event to poll with fd
    fn read_interest(&self, fd: RawFd) {
        self.registry.register_read();
    }

    // Add write event to poll with fd
    fn write_interest(&self, fd: RawFd) {
        self.registry.register_write()
    }

    // Remove the event with fd
    fn close(&self, fd: RawFd) {
        self.registry.remove_interests()
    }
}




