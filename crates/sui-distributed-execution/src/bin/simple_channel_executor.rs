use clap::*;
use std::path::PathBuf;
use std::sync::Arc;
use sui_config::{Config, NodeConfig};
use sui_distributed_execution::{
    seqn_worker,
    exec_worker,
    dash_store::DashMemoryBackedStore,
    mutex_store::MutexedMemoryBackedStore, types::{SailfishMessage, Transaction},
};
use sui_types::multiaddr::Multiaddr;
use tokio::sync::mpsc;

const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--dirty", "--exclude", "*"],
            fallback = ""
        );

        if version.is_empty() {
            panic!("unable to query git revision");
        }
        version
    }
};
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION);

const DEFAULT_CHANNEL_SIZE:usize = 1024;


#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version = VERSION)]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,

    /// Specifies the watermark up to which I will download checkpoints
    #[clap(long)]
    download: Option<u64>,

    /// Specifies the watermark up to which I will execute checkpoints
    #[clap(long)]
    execute: Option<u64>,

    #[clap(long, help = "Specify address to listen on")]
    listen_address: Option<Multiaddr>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let args = Args::parse();
    let config = NodeConfig::load(&args.config_path).unwrap();
    let genesis = Arc::new(config.genesis().expect("Could not load genesis"));
    let mut sw_state = seqn_worker::SequenceWorkerState::new(&config).await;
    let metrics1 = sw_state.metrics.clone();
    let metrics2 = sw_state.metrics.clone();
    let store1 = DashMemoryBackedStore::new(); // use the mutexed store for concurrency control
    let mut ew_state1 = exec_worker::ExecutionWorkerState::new(store1);
    ew_state1.init_store(&genesis);
    let store2 = DashMemoryBackedStore::new(); // use the mutexed store for concurrency control
    let mut ew_state2 = exec_worker::ExecutionWorkerState::new(store2);
    ew_state2.init_store(&genesis);


    // ==== Measure Sequence Worker (w/o EW) ====

    let (sw_sender, mut rcv) = mpsc::channel(DEFAULT_CHANNEL_SIZE);

    let dropper = tokio::spawn(async move {
        while let Some(msg) = rcv.recv().await {
            drop(msg);
        }
    });

    sw_state.run(
        config.clone(), 
        args.download, 
        args.execute,
        sw_sender, 
        None, 
    ).await;
    dropper.await.unwrap();


    // ==== Run Sequence Worker and Queues Manager ====

    let (sw_sender, mut rcv) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let (manager_sender, mut drop_rcv) = mpsc::channel(DEFAULT_CHANNEL_SIZE);

    let dropper = tokio::spawn(async move {
        while let Some(msg) = drop_rcv.recv().await {
            drop(msg);
        }
    });

    let qw = tokio::spawn(async move {
        let mut manager = exec_worker::QueuesManager::new(manager_sender);
        while let Some(msg) = rcv.recv().await {
            if let SailfishMessage::Transaction {
                tx, 
                tx_effects, 
                checkpoint_seq,
            } = msg {
                let full_tx = Transaction{tx, ground_truth_effects: tx_effects, checkpoint_seq};
                if full_tx.is_epoch_change() {
                    // don't queue to manager
                } else {
                    manager.queue_tx(full_tx.clone()).await;
                    manager.clean_up(&full_tx).await;
                }
            }
        }
    });

    sw_state.run(
        config.clone(),
        args.download,
        args.execute,
        sw_sender,
        None,
    ).await;
    qw.await.unwrap();
    dropper.await.unwrap();


    // ==== Run Both (EW + SW) ====
    // Results from here are used by the EW below to substitute the missing SW.

    // Channel from sw to ew
    let (sw_sender, mut cs_receiver) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let (cs_sender1, sw_receiver1) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let (cs_sender2, mut sw_receiver2) = mpsc::channel(100_000_000);
    // Channel from ew to sw
    let (ew_sender1, ew_receiver1) = mpsc::channel(DEFAULT_CHANNEL_SIZE);

    // Run Sequence Worker
    let sw_handler = tokio::spawn(async move {
        sw_state.run(
            config.clone(),
            args.download,
            args.execute,
            sw_sender,
            Some(ew_receiver1),
        ).await;
    });

    // This task copies each message to a channel for the 2nd Execution Worker
    // before passing it on to the channel for the 1st Execution Worker.
    let channel_splitter = tokio::spawn(async move {
        while let Some(msg) = cs_receiver.recv().await {
            cs_sender1.send(msg.clone()).await.expect("send failed");
            let permit = cs_sender2.try_reserve().expect("channel full");
            permit.send(msg);
        }
    });

    // Run Execution Worker
    if let Some(watermark) = args.execute {
        ew_state1.run(
            metrics1,
            watermark,
            sw_receiver1,
            Some(ew_sender1)
        ).await;
    }

    // Wait for workers to terminate
    sw_handler.await.expect("sw failed");
    channel_splitter.await.expect("splitter failed");


    // ==== Measure Execution Worker (w/o SW) ====
    // This uses the results from the exeuction above (SW + EW) stored in an mpsc channel.
    // Run 2nd Execution Worker

    let (fw_sender, fw_receiver) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let (ew_sender, mut ew_receiver) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    
    let forwarder = tokio::spawn(async move {
        // start epoch 0 manually
        let msg = sw_receiver2.recv().await.expect("EpochStart for epoch 0 should exist");
        fw_sender.send(msg).await.unwrap();

        while let Some(msg) = sw_receiver2.recv().await {
            if let &SailfishMessage::EpochStart { .. } = &msg {
                if let SailfishMessage::EpochEnd { .. } = ew_receiver.recv().await.unwrap() {
                    // do nothing
                } else {
                    eprintln!("unexpected msg");
                }
            }
            fw_sender.send(msg).await.unwrap();
        }
    });

    if let Some(watermark) = args.execute {
        ew_state2.run(
            metrics2,
            watermark,
            fw_receiver,
            Some(ew_sender)
        ).await;
    }
    forwarder.await.unwrap();
}
