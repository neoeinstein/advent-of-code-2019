use super::{Executable, Word};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Buffer {
    last_output: Option<Word>,
    rx: Receiver<Word>,
    tx: Sender<Word>,
}

impl Buffer {
    pub fn between(source: &mut Executable, target: &mut Executable) -> Self {
        let (tx, buf_out) = channel();
        let (buf_in, rx) = channel();

        source.pipe_outputs_to(buf_in);
        target.pipe_inputs_from(buf_out);

        Buffer {
            last_output: None,
            rx,
            tx,
        }
    }

    pub fn injector(&self) -> Sender<Word> {
        self.tx.clone()
    }

    pub fn execute_in_thread(self) -> std::thread::JoinHandle<Option<Word>> {
        std::thread::spawn(move || self.execute())
    }

    pub fn execute(mut self) -> Option<Word> {
        while let Ok(value) = self.rx.recv() {
            self.last_output = Some(value);

            // Ignore if the listener has stopped listening
            let _ = self.tx.send(value);
        }

        self.last_output
    }
}
