use std::sync::{Arc, Mutex};

use crate::StreamFrame;

pub enum StreamDirectionality {
    Bi = 0,
    Uni = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum SendState {
    Ready,
    Send,
    DataSent,
    DataRecieved,

    ResetSent,
    ResetRecieved,
}

#[derive(Debug, Clone, Copy)]
pub enum RecvState {
    Recv,
    SizeKnown,
    DataRecieved,
    DataRead,

    ResetRecieved,
    ResetRead,
}

pub trait Stream {
    fn get_directionality() -> StreamDirectionality
    where
        Self: Sized;
    fn get_id(&self) -> u64;
}
pub trait Recv {
    fn get_buffer(&self) -> Arc<Mutex<Vec<u8>>>;
    fn get_state(&self) -> RecvState;

    // NOTE: data may be recieved out of order, in which case it must be buffered
    fn handle_stream_frame(&self, _frame: StreamFrame) {}
}
pub trait Send {
    fn get_state(&self) -> SendState;
}

#[derive(Debug)]
pub struct RWStream {
    pub id: u64,
    pub recv_buffer: Arc<Mutex<Vec<u8>>>,
    pub recv_state: RecvState,
    pub send_state: SendState,
}
impl Stream for RWStream {
    fn get_directionality() -> StreamDirectionality {
        StreamDirectionality::Bi
    }

    fn get_id(&self) -> u64 {
        self.id
    }
}
impl Recv for RWStream {
    fn get_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
        self.recv_buffer.clone()
    }

    fn get_state(&self) -> RecvState {
        self.recv_state
    }
}
impl Send for RWStream {
    fn get_state(&self) -> SendState {
        self.send_state
    }
}

#[derive(Debug)]
pub struct RStream {
    pub id: u64,
    pub buffer: Arc<Mutex<Vec<u8>>>,
    pub state: RecvState,
}
impl Stream for RStream {
    fn get_directionality() -> StreamDirectionality {
        StreamDirectionality::Uni
    }

    fn get_id(&self) -> u64 {
        self.id
    }
}
impl Recv for RStream {
    fn get_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
        self.buffer.clone()
    }

    fn get_state(&self) -> RecvState {
        self.state
    }
}

#[derive(Debug)]
pub struct WStream {
    pub id: u64,
    pub state: SendState,
}
impl Stream for WStream {
    fn get_directionality() -> StreamDirectionality {
        StreamDirectionality::Uni
    }

    fn get_id(&self) -> u64 {
        self.id
    }
}
impl Send for WStream {
    fn get_state(&self) -> SendState {
        self.state
    }
}
