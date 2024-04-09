use std::time::Duration;

use log::debug;
use tokio::{
    sync::{mpsc, watch},
    time::sleep,
};

use crate::Error;

pub(crate) async fn run_sync_rbac(
    restart_s: mpsc::Sender<()>,
    mut stop_r: watch::Receiver<()>,
) -> Result<(), Error> {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(10)) => restart_s.send(()).await?,
            _ = stop_r.changed() => {
                debug!("received stop signal");
                return Ok(())
            }
        }
    }
}
