//! # Day 23: Category Six
//!
//! The droids have finished repairing as much of the ship as they can. Their
//! report indicates that this was a Category 6 disaster - not because it was
//! that bad, but because it destroyed the stockpile of Category 6 network
//! cables as well as most of the ship's network infrastructure.
//!
//! You'll need to rebuild the network from scratch.
//!
//! The computers on the network are standard Intcode computers that communicate
//! by sending packets to each other. There are 50 of them in total, each
//! running a copy of the same Network Interface Controller (NIC) software (your
//! puzzle input). The computers have network addresses 0 through 49; when each
//! computer boots up, it will request its network address via a single input
//! instruction. Be sure to give each computer a unique network address.
//!
//! Once a computer has received its network address, it will begin doing work
//! and communicating over the network by sending and receiving packets. All
//! packets contain two values named X and Y. Packets sent to a computer are
//! queued by the recipient and read in the order they are received.
//!
//! To send a packet to another computer, the NIC will use three output
//! instructions that provide the destination address of the packet followed by
//! its X and Y values. For example, three output instructions that provide the
//! values `10, 20, 30` would send a packet with X=20 and Y=30 to the computer
//! with address 10.
//!
//! To receive a packet from another computer, the NIC will use an input
//! instruction. If the incoming packet queue is empty, provide `-1`. Otherwise,
//! provide the X value of the next packet; the computer will then use a second
//! input instruction to receive the Y value for the same packet. Once both
//! values of the packet are read in this way, the packet is removed from the
//! queue.
//!
//! Note that these input and output instructions never block. Specifically,
//! output instructions do not wait for the sent packet to be received - the
//! computer might send multiple packets before receiving any. Similarly, input
//! instructions do not wait for a packet to arrive - if no packet is waiting,
//! input instructions should receive `-1`.
//!
//! Boot up all 50 computers and attach them to your network. What is the Y
//! value of the first packet sent to address `255`?
//!
//! ## Part Two
//!
//! Packets sent to address `255` are handled by a device called a NAT (Not
//! Always Transmitting). The NAT is responsible for managing power consumption
//! of the network by blocking certain packets and watching for idle periods in
//! the computers.
//!
//! If a packet would be sent to address 255, the NAT receives it instead. The
//! NAT remembers only the last packet it receives; that is, the data in each
//! packet it receives overwrites the NAT's packet memory with the new packet's
//! X and Y values.
//!
//! The NAT also monitors all computers on the network. If all computers have
//! empty incoming packet queues and are continuously trying to receive packets
//! without sending packets, the network is considered idle.
//!
//! Once the network is idle, the NAT sends only the last packet it received to
//! address 0; this will cause the computers on the network to resume activity.
//! In this way, the NAT can throttle power consumption of the network when the
//! ship needs power in other areas.
//!
//! Monitor packets released to the computer at address `0` by the NAT. What is
//! the first Y value delivered by the NAT to the computer at address 0 twice in
//! a row?

use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-23");

#[derive(Clone, Copy, Debug)]
struct Packet {
    address: intcode::Word,
    data: PacketData,
}

#[derive(Clone, Copy, Debug, Default)]
struct PacketData {
    x: intcode::Word,
    y: intcode::Word,
}

#[derive(Debug)]
struct NetworkRouter {
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
    members: Vec<Sender<PacketData>>,
    watchers: Vec<tokio::sync::watch::Receiver<bool>>,
}

impl NetworkRouter {
    fn new() -> Self {
        let (tx, rx) = channel(1);
        Self {
            tx,
            rx,
            members: Vec::new(),
            watchers: Vec::new(),
        }
    }

    async fn attach_client<S>(
        &mut self,
        mut exe: intcode::AsyncExecutable<S>,
    ) -> Result<intcode::AsyncExecutable<FromNetworkTranslator>> {
        let id = self.members.len() as intcode::Word;
        let (tx, net_translator) = ToNetworkTranslator::new(id, &*self);
        exe.pipe_outputs_to(tx);
        tokio::spawn(net_translator.execute());

        let net_in = FromNetworkTranslator::new(id);

        self.members.push(net_in.tx());
        self.watchers.push(net_in.idle());

        let exe = exe.input_stream(net_in);

        Ok(exe)
    }

    async fn execute_nat(mut self) -> Result<intcode::Word> {
        let mut last_sent: Option<PacketData> = None;
        let mut nat_packet: Option<PacketData> = None;
        loop {
            if let Ok(pkt) = self.rx.try_recv() {
                log::debug!("Router packet received: {:?}", pkt);
                if pkt.address == 255 {
                    nat_packet = Some(pkt.data);
                } else {
                    self.members[pkt.address as usize].send(pkt.data).await?;
                }
                continue;
            } else if let Some(pkt) = nat_packet.take() {
                log::trace!("Maybe idle");
                if self.watchers.iter().all(|w| *w.borrow()) {
                    if let Some(last) = last_sent {
                        if last.y == pkt.y {
                            return Ok(last.y);
                        }
                    }
                    last_sent = Some(pkt);
                    self.members[0].send(pkt).await?;
                    continue;
                }
            }
            tokio::task::yield_now().await;
        }
    }

    async fn execute(mut self) -> Result<intcode::Word> {
        while let Some(pkt) = self.rx.recv().await {
            log::debug!("Router packet received: {:?}", pkt);
            if pkt.address == 255 {
                return Ok(pkt.data.y);
            }
            self.members[pkt.address as usize].send(pkt.data).await?;
        }
        Ok(-1)
    }
}

struct ToNetworkTranslator {
    id: intcode::Word,
    from_exe_rx: Receiver<intcode::Word>,
    to_router_tx: Sender<Packet>,
}

impl ToNetworkTranslator {
    fn new(id: intcode::Word, router: &NetworkRouter) -> (Sender<intcode::Word>, Self) {
        let chan = channel(1);
        (
            chan.0,
            Self {
                id,
                from_exe_rx: chan.1,
                to_router_tx: router.tx.clone(),
            },
        )
    }

    async fn execute(self) {
        let _ = self.execute_inner().await;
    }

    async fn execute_inner(mut self) -> Option<()> {
        loop {
            log::trace!("{} Waiting for data", self.id);
            let address = self.from_exe_rx.recv().await?;
            log::trace!("{} Sent address: {}", self.id, address);
            let x = self.from_exe_rx.recv().await?;
            log::trace!("{} Sent x: {}", self.id, x);
            let y = self.from_exe_rx.recv().await?;
            log::trace!("{} Sent y: {}", self.id, y);
            let packet = Packet {
                address,
                data: PacketData { x, y },
            };
            if self.to_router_tx.send(packet).await.is_err() {
                break;
            }
            log::trace!("{} Sent full packet: {:?}", self.id, packet);
        }
        Some(())
    }
}

struct FromNetworkTranslator {
    id: intcode::Word,
    tx: Sender<PacketData>,
    packet_source: Receiver<PacketData>,
    packet_y: Option<intcode::Word>,
    idle_signal: tokio::sync::watch::Sender<bool>,
    idle_watch: tokio::sync::watch::Receiver<bool>,
    yielded: bool,
}

impl FromNetworkTranslator {
    fn new(network_id: intcode::Word) -> Self {
        let (tx, rx) = channel(1);
        let (idle_signal, idle_watch) = tokio::sync::watch::channel(false);
        Self {
            id: network_id,
            tx,
            packet_source: rx,
            packet_y: Some(network_id),
            idle_signal,
            idle_watch,
            yielded: false,
        }
    }

    fn idle(&self) -> tokio::sync::watch::Receiver<bool> {
        self.idle_watch.clone()
    }

    fn tx(&self) -> Sender<PacketData> {
        self.tx.clone()
    }
}

impl futures::stream::Stream for FromNetworkTranslator {
    type Item = intcode::Word;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let packet_y = self.packet_y;
        let pin = self.get_mut();
        if let Some(y) = packet_y {
            log::trace!("{} Receiving y: {}", pin.id, y);
            pin.packet_y = None;
            std::task::Poll::Ready(Some(y))
        } else if let std::task::Poll::Ready(Some(data)) =
            std::pin::Pin::new(&mut pin.packet_source).poll_next(ctx)
        {
            let _ = pin.idle_signal.broadcast(false);
            log::debug!("{} Received packet: {:?}", pin.id, data);
            pin.packet_y = Some(data.y);
            log::trace!("{} Receiving x: {}", pin.id, data.x);
            std::task::Poll::Ready(Some(data.x))
        } else if !pin.yielded {
            pin.yielded = true;
            log::trace!("{} Yielding", pin.id);
            ctx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            pin.yielded = false;
            let _ = pin.idle_signal.broadcast(true);
            log::trace!("{} Idle", pin.id);
            std::task::Poll::Ready(Some(-1))
        }
    }
}

async fn part1(program: &intcode::Memory) -> Result<intcode::Word> {
    let mut router = NetworkRouter::new();
    let mut futs = Vec::new();
    for _ in 0..50_u8 {
        let exe = router
            .attach_client(intcode::AsyncExecutable::from(program.clone()))
            .await?;
        let fut = exe.execute();
        futs.push(fut);
    }

    let j = router.execute();
    for f in futs {
        tokio::spawn(f);
    }
    j.await
}

async fn part2(program: &intcode::Memory) -> Result<intcode::Word> {
    let mut router = NetworkRouter::new();
    let mut futs = Vec::new();
    for _ in 0..50_u8 {
        let exe = router
            .attach_client(intcode::AsyncExecutable::from(program.clone()))
            .await?;
        let fut = exe.execute();
        futs.push(fut);
    }

    let j = router.execute_nat();
    for f in futs {
        tokio::spawn(f);
    }
    j.await
}

pub fn run() -> Result<()> {
    let program: intcode::Memory = PUZZLE_INPUT.parse()?;
    let mut runtime = tokio::runtime::Runtime::new()?;
    let result = runtime.block_on(part1(&program))?;

    println!("Packet to 255: {}", result);

    let result = runtime.block_on(part2(&program))?;

    println!("Last doubled to 0: {}", result);

    Ok(())
}
