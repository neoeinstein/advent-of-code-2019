use super::{AsyncExecutable, ExecutionError, Memory, Word};
use num_traits::ToPrimitive;
use tokio::{
    io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct AsciiTerminal {
    tx: Sender<Word>,
    rx: Receiver<Word>,
    exe: AsyncExecutable,
    predefined_input: &'static [&'static str],
}

impl AsciiTerminal {
    pub fn new(program: Memory) -> Self {
        let mut exe = AsyncExecutable::from(program);
        let input = channel(20);
        let output = channel(20);
        exe.pipe_inputs_from(input.1);
        exe.pipe_outputs_to(output.0);
        Self {
            tx: input.0,
            rx: output.1,
            exe,
            predefined_input: &[],
        }
    }

    pub fn with_input(program: Memory, predefined_input: &'static [&'static str]) -> Self {
        let mut exe = AsyncExecutable::from(program);
        let input = channel(20);
        let output = channel(20);
        exe.pipe_inputs_from(input.1);
        exe.pipe_outputs_to(output.0);
        Self {
            tx: input.0,
            rx: output.1,
            exe,
            predefined_input
        }
    }

    pub async fn execute(self) -> Result<Memory, ExecutionError> {
        tokio::spawn(run_input(self.tx, self.predefined_input));
        let out = tokio::spawn(run_output(self.rx));
        let join = tokio::spawn(self.exe.execute());
        let r = join.await.unwrap();
        let _ = out.await;
        r
    }
}

async fn run_input(mut input: Sender<Word>, predefined_input: &'static [&'static str]) -> tokio::io::Result<()> {
    for line in predefined_input {
        for ch in line.bytes() {
            let _ = input.send(ch as i64).await;
        }
        let _ = input.send(b'\n' as i64).await;
    }

    let mut i = BufReader::new(tokio::io::stdin());
    let mut buf = String::new();
    loop {
        let c = i.read_line(&mut buf).await?;
        if c == 0 {
            break;
        }
        for ch in buf.bytes() {
            let _ = input.send(ch as i64).await;
        }
        buf.clear();
    }
    Ok(())
}

async fn run_output(mut output: Receiver<Word>) -> tokio::io::Result<()> {
    let mut o = tokio::io::stdout();
    while let Some(w) = output.recv().await {
        if let Some(ch) = w.to_u8() {
            o.write(&[ch]).await?;
        } else {
            let data = format!("Non-ASCII value received: {}\n", w);
            o.write(data.as_bytes()).await?;
        }
    }
    Ok(())
}

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
