use std::{
    fs,
    process::{ExitStatus, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug, error, trace};
use tokio::{
    process::{Child, Command},
    sync::{oneshot, watch},
};

use crate::Error;

const LOGS_DIR: &str = "logs";

#[derive(Clone, Copy, Default)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Status {
    pub(crate) exit_status: ExitStatus,
}

pub(crate) struct ChildProcess {
    kill_s: oneshot::Sender<()>,
}

impl ChildProcess {
    pub(crate) async fn spawn(
        cmd: &str,
        args: &[&str],
        status_s: watch::Sender<Status>,
    ) -> Result<Self, Error> {
        let mut cmd = Command::new(cmd);

        let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        fs::create_dir_all(LOGS_DIR)?;
        let stdout_file = fs::File::create_new(format!("{}/{}.stdout.txt", LOGS_DIR, start_time))?;
        let stderr_file = fs::File::create_new(format!("{}/{}.stderr.txt", LOGS_DIR, start_time))?;
        cmd.stdout::<Stdio>(stdout_file.into());
        cmd.stderr::<Stdio>(stderr_file.into());

        let child = cmd.args(args).kill_on_drop(true).spawn()?;

        let (kill_s, kill_r) = oneshot::channel::<()>();

        tokio::spawn(async move {
            if let Err(e) = Self::wait(child, start_time, status_s, kill_r).await {
                error!("failed to wait for child process: {start_time}: {e}")
            }
        });

        Ok(Self { kill_s })
    }

    pub(crate) async fn kill(self) -> Result<(), Error> {
        if self.kill_s.is_closed() {
            return Ok(());
        }
        Ok(self.kill_s.send(()).map_err(|_| "failed to send signal to kill child".to_string())?)
    }

    async fn wait(
        mut child: Child,
        start_time: u64,
        status_s: watch::Sender<Status>,
        kill_r: oneshot::Receiver<()>,
    ) -> Result<(), Error> {
        debug!("child process started time is: {start_time}");
        tokio::select! {
            status = child.wait() => {
                let status = status?;
                process_status(status, start_time, status_s)?;
            },
            _ = kill_r => {
                trace!("received signal to kill child: {start_time}");
                child.start_kill()?;
                let status = child.wait().await?;
                process_status(status, start_time, status_s)?;
                trace!("child was signalled to kill: {start_time}");
            }
        };
        Ok(())
    }
}

fn process_status(
    status: ExitStatus,
    start_time: u64,
    status_s: watch::Sender<Status>,
) -> Result<(), Error> {
    debug!("child ({start_time}) is stopped, status is {:?}", status);
    status_s
        .send(Status {
            exit_status: status,
        })
        .map_err(|_| "failed to send result status".to_string())?;
    Ok(())
}

pub(crate) async fn status(mut status_r: watch::Receiver<Status>) -> Result<Status, Error> {
    status_r.changed().await?;
    Ok(*status_r.borrow())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::LevelFilter;
    use tokio::{sync::oneshot, time::sleep};

    use super::*;

    #[tokio::test]
    #[ignore = "run it manually or write proper check for logs file"]
    async fn child_process_all() {
        eprintln!(); // insert first \n before testing logs
        env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).init();
        let (status_s, status_r) = watch::channel(Status::default());
        let child_process = ChildProcess::spawn(
            "sh",
            &["-c", "while true; do echo \"$(date +%s)\"; sleep 1; done"],
            status_s,
        )
        .await
        .unwrap();
        let (done_s, done_r) = oneshot::channel::<()>();
        tokio::spawn(async move {
            let status = status(status_r).await.unwrap();
            eprintln!("Status from test: {:?}", status.exit_status);
            done_s.send(()).unwrap();
        });
        sleep(Duration::from_secs(3)).await;
        child_process.kill().await.unwrap();
        done_r.await.unwrap();
    }
}
