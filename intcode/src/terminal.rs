use super::Word;
use num_traits::ToPrimitive;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Debug)]
pub struct TerminalOut {
    tx: Sender<Word>,
    rx: Receiver<Word>,
}

impl Default for TerminalOut {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalOut {
    pub fn new() -> Self {
        let (tx, rx) = channel(20);
        Self { tx, rx }
    }

    pub fn tx(&self) -> &Sender<Word> {
        &self.tx
    }

    pub fn tx_mut(&mut self) -> &mut Sender<Word> {
        &mut self.tx
    }

    pub async fn write_ascii_output_to_writer(
        mut self,
        mut o: impl AsyncWrite + Unpin,
    ) -> tokio::io::Result<()> {
        drop(self.tx);
        while let Some(w) = self.rx.recv().await {
            if let Some(ch) = w.to_u8() {
                o.write(&[ch]).await?;
            } else {
                let data = format!("Non-ASCII value received: {}\n", w);
                o.write(data.as_bytes()).await?;
            }
        }

        Ok(())
    }
}
