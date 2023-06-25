use std::{io::Error, collections::{HashMap, HashSet}, os::fd::RawFd};

pub type EventId = usize;

#[derive(PartialEq, Hash, Eq)]
pub enum Interest {
    READ,
    WRITE,
}

pub struct Poll {
    epoll_fd: i32,
}

impl Poll {
    pub fn create_epoll() -> Result<Self, Error> {
        let epoll_fd = unsafe { libc::epoll_create1(0) };
        let flags: i32 = unsafe { libc::fcntl(epoll_fd, libc::F_GETFD) };
        if flags == -1 {
            return Err(Error::last_os_error());
        }

        let new_flags = flags | libc::FD_CLOEXEC;
        if unsafe { libc::fcntl(epoll_fd, libc::F_SETFD, new_flags) } == -1 {
            return Err(Error::last_os_error());
        }
        Ok(Poll { epoll_fd })
    }

    pub fn get_fd(&self) -> i32 {
        self.epoll_fd
    }

    pub fn poll(&self, events: &mut Vec<libc::epoll_event>) {
        events.clear();
        let nfds = unsafe {
            libc::epoll_wait(self.epoll_fd, events.as_mut_ptr(), 1024, 1000)
        };

        unsafe{events.set_len(nfds as usize)} 
    }
}

pub struct Registry {
    epoll_fd: i32,
    epoll_set: HashMap<RawFd, HashSet<Interest>>,
}

impl Registry {
    pub fn new(epoll_fd: i32) -> Self {
        Self {
            epoll_fd,
            epoll_set: HashMap::new(),
        }
    }

    pub fn register_read(&mut self, fd: RawFd, event_id: EventId) -> Result<(), Error> {
       self.register_interest(fd, event_id, Interest::READ)
    }

    pub fn register_write(&mut self, fd: RawFd, event_id: EventId) -> Result<(), Error> {
        self.register_interest(fd, event_id, Interest::WRITE)
    }

    pub fn remove_interests(&mut self, fd: RawFd) -> Result<(), Error> {

        let result = unsafe {
            libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_DEL, fd, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(std::io::Error::last_os_error());
        }

        self.epoll_set.remove(&fd);
        unsafe{libc::close(fd)};
        Ok(())
    }


    fn register_interest(&mut self, fd: RawFd, event_id: EventId, interest: Interest) -> Result<(), Error> {

        let interests = self.epoll_set.entry(fd).or_insert(HashSet::new());

        // Calculate the current epoll events we are interested in.
        let mut new_events = if interests.contains(&Interest::READ) {
            libc::EPOLLIN
        } else {
            0
        };
    
        if interests.contains(&Interest::WRITE) {
            new_events |= libc::EPOLLOUT;
        }
    
        // Add the new interest.
        match interest {
            Interest::READ => {
                interests.insert(Interest::READ);
                new_events |= libc::EPOLLIN;
            },
            Interest::WRITE => {
                interests.insert(Interest::WRITE);
                new_events |= libc::EPOLLOUT;
            },
        }
    
        let operation = if interests.len() == 1 {
            libc::EPOLL_CTL_ADD
        } else {
            libc::EPOLL_CTL_MOD
        };
    
        let mut event = libc::epoll_event {
            events: new_events as u32,
            u64: event_id as u64,
        };
    
        let result = unsafe { libc::epoll_ctl(self.epoll_fd, operation, fd, &mut event) };
    
        if result == -1 {
            return Err(std::io::Error::last_os_error());
        }
    
        Ok(())
    }

}