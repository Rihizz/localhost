// server.rs

use std::os::unix::io::{AsRawFd, RawFd};
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use libc::{epoll_create1, epoll_ctl, epoll_wait, epoll_event, EPOLLIN, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLLOpt};

const MAX_EVENTS: i32 = 10;

fn create_epoll_instance() -> RawFd {
    unsafe { epoll_create1(0) }
}

fn add_fd_to_epoll(epoll_fd: RawFd, fd: RawFd) {
    let mut event = epoll_event {
        events: EPOLLIN as u32,
        u64: fd as u64,
    };

    unsafe {
        epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd, &mut event);
    }
}

fn remove_fd_from_epoll(epoll_fd: RawFd, fd: RawFd) {
    unsafe {
        epoll_ctl(epoll_fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut());
    }
}

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Could not bind to address");
    listener.set_nonblocking(true).expect("Could not set non-blocking");

    let epoll_fd = create_epoll_instance();
    add_fd_to_epoll(epoll_fd, listener.as_raw_fd());

    let mut events = [epoll_event { events: 0, u64: 0 }; MAX_EVENTS as usize];

    let mut connections: HashMap<RawFd, TcpStream> = HashMap::new();

    loop {
        let num_events = unsafe {
            epoll_wait(epoll_fd, events.as_mut_ptr(), MAX_EVENTS, -1)
        };

        for i in 0..num_events {
            let event_fd = events[i as usize].u64 as RawFd;

            if event_fd == listener.as_raw_fd() {
                // We have a new connection
                match listener.accept() {
                    Ok((stream, _)) => {
                        stream.set_nonblocking(true).expect("Could not set non-blocking");
                        add_fd_to_epoll(epoll_fd, stream.as_raw_fd());
                        connections.insert(stream.as_raw_fd(), stream);
                    },
                    Err(e) => eprintln!("Failed to accept connection: {}", e),
                }
            } else {
                // We have data to read
                handle_read(event_fd, &mut connections);
            }
        }
    }
}

fn handle_read(fd: RawFd, connections: &mut HashMap<RawFd, TcpStream>) {
    // Read from the connection and handle it
    // If the connection is closed, remove it from the epoll instance and HashMap
}
