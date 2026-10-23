use crate::cli::{IsTerminal, StdinStream, StdoutStream};
use crate::p2;
use tokio::io::{AsyncRead, AsyncWrite};
use wasmtime_wasi_io::streams::{InputStream, OutputStream};

use flux_rs::attrs::*;

// Implementation for p2::pipe::MemoryInputPipe
impl IsTerminal for p2::pipe::MemoryInputPipe {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdinStream for p2::pipe::MemoryInputPipe {
    #[trusted]
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(self.clone())
    }
    #[trusted]
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(self.clone())
    }
}

// Implementation for p2::pipe::MemoryOutputPipe
impl IsTerminal for p2::pipe::MemoryOutputPipe {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdoutStream for p2::pipe::MemoryOutputPipe {
    #[trusted]
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(self.clone())
    }
    #[trusted]
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(self.clone())
    }
}
