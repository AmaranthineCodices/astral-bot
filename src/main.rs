/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use color_eyre::eyre::{Report, Result, WrapErr};

async fn launch(mut shutdown_rx: tokio::sync::mpsc::Receiver<()>) {
    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                break
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("Unable to install eyre hooks")?;
    dotenv::dotenv().ok();

    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(launch(shutdown_rx));

    match tokio::signal::ctrl_c().await {
        Ok(()) => shutdown_tx.send(()).await?,
        Err(err) => return Err(Report::new(err).wrap_err("Unable to listen to Ctrl-C handler")),
    }

    Ok(())
}
