use std::time::Duration;

use log::{debug, error};
use peaq_client::SignerClient;
use tokio::{
    sync::{mpsc, watch},
    time::{sleep, timeout},
};

use crate::Error;

pub(crate) async fn run_sync_rbac(
    peaq_client: SignerClient,
    restart_s: mpsc::Sender<()>,
    mut stop_r: watch::Receiver<()>,
) -> Result<(), Error> {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(60)) => {
                match timeout(Duration::from_secs(10), sync_rbac(peaq_client.clone(), restart_s.clone())).await {
                    Ok(res) => {
                        if let Err(e) = res {
                            error!("failed to sync rbac: {e}")
                        }
                    },
                    Err(e) => error!("failed to sync rbac; 10s timeout: {e}")
                }
            },
            _ = stop_r.changed() => {
                debug!("received stop signal");
                return Ok(())
            }
        }
    }
}

async fn sync_rbac(_peaq_client: SignerClient, restart_s: mpsc::Sender<()>) -> Result<(), Error> {
    // todo: get events from blocks like indexer and if see new rbac event -> update config -> restart mcc
    // todo: we need to do the same with rbac revocation
    Ok(restart_s.send(()).await?)
}
