use anyhow::anyhow;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use wasmtime_wasi::{HostOutputStream, StdoutStream, StreamError, Subscribe};

#[derive(Debug, Clone)]
pub struct MemoryOutputPipe {
    capacity: usize,
    buffer: Arc<Mutex<bytes::BytesMut>>,
    tx: Arc<Mutex<UnboundedSender<String>>>,
    id: uuid::Uuid,
    print_guest_output: bool,
}

impl MemoryOutputPipe {
    pub fn new(capacity: usize, tx: UnboundedSender<String>, print_guest_output: bool,id: uuid::Uuid) -> Self {
        MemoryOutputPipe {
            capacity,
            buffer: Arc::new(Mutex::new(bytes::BytesMut::new())),
            tx: Arc::new(Mutex::new(tx)),
            id,
            print_guest_output,
        }
    }

    pub fn contents(&self) -> bytes::Bytes {
        self.buffer.lock().unwrap().clone().freeze()
    }

    pub fn contents_string(&self) -> String {
        String::from_utf8(self.buffer.lock().unwrap().clone().freeze().to_vec()).unwrap()
    }

    /// Meant only for debugging
    pub async fn poll_contents(&self) {
        let buffer = self.buffer.clone();
        let spawn = tokio::spawn(async move {
            println!("start polling");
            loop {
                let contents = buffer.lock().unwrap().clone().freeze();
                if !contents.is_empty() {
                    println!(
                        "in poll: {:?}",
                        String::from_utf8(contents.to_vec()).unwrap()
                    );
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        });
        spawn.await.unwrap();
    }

    pub fn try_into_inner(self) -> Option<bytes::BytesMut> {
        std::sync::Arc::into_inner(self.buffer).map(|m| m.into_inner().unwrap())
    }
}

impl HostOutputStream for MemoryOutputPipe {
    fn write(&mut self, bytes: Bytes) -> Result<(), StreamError> {
        let mut buf = self.buffer.lock().unwrap();
        if bytes.len() > self.capacity - buf.len() {
            return Err(StreamError::Trap(anyhow!(
                "write beyond capacity of MemoryOutputPipe"
            )));
        }
        buf.extend_from_slice(bytes.as_ref());

        let new_content = String::from_utf8(bytes.to_vec()).unwrap();

        if self.print_guest_output {
            println!("\x1b[32mpython {}:\x1b[0m {}", self.id, new_content);
        }

        // send stdout/stderror on to the receiver
        let tx = self.tx.lock().unwrap();
        tx.send(new_content).unwrap();
        drop(tx);

        buf.clear();

        // Always ready for writing
        Ok(())
    }

    fn flush(&mut self) -> Result<(), StreamError> {
        let mut buf = self.buffer.lock().unwrap();
        if !buf.is_empty() {
            let new_content = String::from_utf8(buf.clone().freeze().to_vec()).unwrap(); 

            if self.print_guest_output {
                println!("\x1b[32mpython {}:\x1b[0m {}", self.id, new_content);
            }
            
            let tx = self.tx.lock().unwrap();
            tx.send(new_content).unwrap();
            drop(tx);

            buf.clear();
        }
        Ok(())
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        let consumed = self.buffer.lock().unwrap().len();
        if consumed < self.capacity {
            Ok(self.capacity - consumed)
        } else {
            // Since the buffer is full, no more bytes will ever be written
            Err(StreamError::Closed)
        }
    }
}

impl StdoutStream for MemoryOutputPipe {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(self.clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
impl Subscribe for MemoryOutputPipe {
    async fn ready(&mut self) {}
}
