use std::time::Duration;

use log::debug;
use tokio::{
    sync::{mpsc, watch},
    time::timeout,
};

use crate::{
    child_process::{ChildProcess, Status},
    Error,
};

pub(crate) async fn run_staex_mcc(
    mut restart_r: mpsc::Receiver<()>,
    mut stop_r: watch::Receiver<()>,
) -> Result<(), Error> {
    loop {
        tokio::select! {
            _ = restart_r.recv() => restart_staex_mcc().await?,
            _ = stop_r.changed() => return Ok(())
        }
    }
}

async fn restart_staex_mcc() -> Result<(), Error> {
    debug!("received a signal to restart staex mcc");
    let (status_s, mut status_r) = watch::channel(Status::default());
    let _child_process = ChildProcess::spawn("systemctl", &["restart", "mcc"], status_s)?;
    timeout(Duration::from_secs(10), status_r.changed()).await??;
    let status = *status_r.borrow();
    debug!("exit status after restart staex mcc: {}", status.exit_status);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::LevelFilter;
    use tokio::{
        sync::{mpsc, oneshot, watch},
        time::{sleep, timeout},
    };

    use crate::staex_mcc::run_staex_mcc;

    #[tokio::test]
    #[ignore = "start manually or write result checking"]
    async fn run_staex_mcc_all() {
        eprintln!(); // insert first \n before testing logs
        env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).init();
        let (restart_s, restart_r) = mpsc::channel::<()>(1);
        let (stop_s, stop_r) = watch::channel(());
        let (done_s, done_r) = oneshot::channel::<()>();
        tokio::spawn(async move {
            run_staex_mcc(restart_r, stop_r).await.unwrap();
            done_s.send(()).unwrap();
        });
        sleep(Duration::from_secs(1)).await;
        restart_s.send_timeout((), Duration::from_secs(1)).await.unwrap();
        sleep(Duration::from_secs(3)).await;
        stop_s.send(()).unwrap();
        timeout(Duration::from_secs(10), done_r).await.unwrap().unwrap();
    }
}
