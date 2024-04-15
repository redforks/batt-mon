use notify_rust::{Notification, Timeout};
use snafu::prelude::*;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, snafu::Whatever>;

fn send_notification(s: impl AsRef<str>) -> Result<()> {
    whatever!(
        Notification::new()
            .summary("Battery Low")
            .body(s.as_ref())
            .timeout(Timeout::Never)
            .show(),
        "Send notification"
    );
    Ok(())
}

#[snafu::report]
fn main() -> Result<()> {
    // read file /sys/class/power_supply/BAT0/status, return if its contents is Charging
    let battery_status = whatever!(
        read_to_string("/sys/class/power_supply/BAT0/status"),
        "Read battery status"
    );
    if battery_status.trim() == "Charging" {
        return Ok(());
    }

    let battery_level = whatever!(
        read_to_string("/sys/class/power_supply/BAT0/capacity"),
        "Read battery level"
    );
    // convert to u8
    let battery_level: u8 = whatever!(battery_level.trim().parse(), "Parse battery_level");
    // notify if battery level is less than 30
    if battery_level < 30 {
        send_notification(format!("Battery level is {}%", battery_level))?;
    }

    Ok(())
}
