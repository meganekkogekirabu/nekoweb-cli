use std::time::{SystemTime, UNIX_EPOCH};

use chrono::DateTime;

use crate::Neko;

fn to_timestamp(time: SystemTime) -> anyhow::Result<String> {
    let duration = i64::try_from(time.duration_since(UNIX_EPOCH)?.as_secs())?;
    Ok(format!(
        "{}",
        DateTime::from_timestamp_secs(duration)
            .unwrap()
            .format("%m/%d/%Y at %H:%M")
    ))
}

impl Neko {
    pub async fn info(&self, site: Option<String>) -> anyhow::Result<()> {
        let info = self.client.get_site(site.as_deref()).await?;

        println!("\x1b[31mdomain:\x1b[0m {}", info.domain);
        println!("\x1b[32mupdates:\x1b[0m {}", info.updates);
        println!("\x1b[33mfollowers:\x1b[0m {}", info.followers);
        println!("\x1b[34mviews:\x1b[0m {}", info.views);
        println!(
            "\x1b[35mcreated on:\x1b[0m {}",
            to_timestamp(info.created_at)?
        );
        println!(
            "\x1b[36mlast updated on:\x1b[0m {}",
            to_timestamp(info.updated_at)?
        );

        Ok(())
    }
}
