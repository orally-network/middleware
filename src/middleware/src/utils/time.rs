use std::{sync::Arc, time::Duration};

use crate::log;

pub async fn sleep(dur: Duration) {
    let notify = Arc::new(tokio::sync::Notify::new());
    let notifyer = notify.clone();

    log!("Sleeping for {}ms", dur.as_millis());
    ic_cdk_timers::set_timer(dur, move || {
        notifyer.notify_one();
    });

    notify.notified().await;
    log!("Sleeping finished");
}
