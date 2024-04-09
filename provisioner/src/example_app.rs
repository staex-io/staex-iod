use log::debug;
use tokio::sync::watch;

use crate::{
    child_process::{wait_status, ChildProcess, Status},
    Error,
};

pub(crate) async fn run_example_app(mut stop_r: watch::Receiver<()>) -> Result<(), Error> {
    loop {
        let (status_s, status_r) = watch::channel(Status::default());
        let child_process =
            ChildProcess::spawn("./example-app/example-app", &["server"], status_s)?;
        tokio::select! {
            status = wait_status(status_r.clone()) => {
                let status = status?;
                process_status(status);
                continue;
            }
            _ = stop_r.changed() => {
                child_process.kill()?;
                process_status(wait_status(status_r).await?);
                return Ok(());
            }
        }
    }
}

fn process_status(status: Status) {
    debug!("received new process status: {}", status.exit_status);
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::LevelFilter;
    use tokio::{
        sync::{oneshot, watch},
        time::{sleep, timeout},
    };

    use crate::example_app::run_example_app;

    #[tokio::test]
    #[ignore = "run it manually or write a result check"]
    async fn run_example_app_all() {
        eprintln!(); // insert first \n before testing logs
        env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).init();
        let (stop_s, stop_r) = watch::channel(());
        let (done_s, done_r) = oneshot::channel::<()>();
        tokio::spawn(async move {
            run_example_app(stop_r).await.unwrap();
            done_s.send(()).unwrap();
        });
        sleep(Duration::from_secs(3)).await;
        stop_s.send(()).unwrap();
        timeout(Duration::from_secs(10), done_r).await.unwrap().unwrap();
    }
}
