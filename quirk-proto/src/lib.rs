mod frames;
pub use frames::*;
mod stream;
use std::sync::{Arc, Mutex};
pub use stream::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnSide {
    Client = 0,
    Server = 1,
}

impl ConnSide {
    pub fn swap(&self) -> Self {
        match self {
            Self::Client => Self::Server,
            Self::Server => Self::Client,
        }
    }
}

#[derive(Debug, Default)]
struct Streams {
    bi: Vec<Arc<RWStream>>,
    read: Vec<Arc<RStream>>,
    write: Vec<Arc<WStream>>,
}

pub struct Conn {
    side: ConnSide,
    streams: Streams,
}

impl Conn {
    pub fn new() -> Self {
        Self {
            side: ConnSide::Client,
            streams: Streams::default(),
        }
    }

    fn get_next_id(
        streams: Vec<Arc<dyn Stream>>,
        directionality: StreamDirectionality,
        initiator: ConnSide,
    ) -> u64 {
        match streams.last() {
            Some(last) => {
                let last_id = last.get_id();
                let flags = last_id & 0b11; // 2 least significant bits are initiator and directionality flags, so keep them
                let next_id = (last_id >> 2) + 1; // Get unique part of last ID and increment it
                (next_id << 2) | flags // Shift the unique part back up and put the flags back
            }
            None => ((directionality as u64) << 1) | (initiator as u64),
        }
    }

    pub fn new_bi_stream(&mut self) -> Arc<RWStream> {
        let stream = Arc::new(RWStream {
            id: Self::get_next_id(
                self.streams
                    .bi
                    .iter()
                    .cloned()
                    .map(|x| x as Arc<dyn Stream>)
                    .collect(),
                StreamDirectionality::Bi,
                self.side,
            ),
            recv_buffer: Arc::new(Mutex::new(Vec::new())),
            recv_state: RecvState::Recv,
            send_state: SendState::Ready,
        });
        self.streams.bi.push(stream.clone());
        stream
    }

    pub fn new_uni_stream(&mut self) -> Arc<WStream> {
        let stream = Arc::new(WStream {
            id: Self::get_next_id(
                self.streams
                    .write
                    .iter()
                    .cloned()
                    .map(|x| x as Arc<dyn Stream>)
                    .collect(),
                StreamDirectionality::Uni,
                self.side,
            ),
            state: SendState::Ready,
        });
        self.streams.write.push(stream.clone());
        stream
    }
}

pub struct Server {
    conns: Vec<Conn>,
}

impl Server {
    pub fn new() -> Self {
        Self { conns: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_id_generation() {
        let mut conn = Conn::new();
        let stream1 = conn.new_bi_stream();
        let stream2 = conn.new_bi_stream();
        let stream3 = conn.new_uni_stream();

        assert_eq!(stream1.id, 0b0000);
        assert_eq!(stream2.id, 0b0100);
        assert_eq!(stream3.id, 0b0010);
    }
}
