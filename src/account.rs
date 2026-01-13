use crate::Neko;
use std::{
    io::{self, BufRead, Write},
    process,
};

impl Neko {
    pub fn login(&mut self) -> anyhow::Result<()> {
        let link = "https://nekoweb.org/api";

        println!("hit enter to open {link} in your browser");
        println!("generate an API key from there and paste it here");
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let line = line?;

            if line.is_empty() {
                open::that(link)?;
            } else {
                self.config.api_key = line;
                confy::store(&self.config_folder, Some(&*self.config_file), &self.config)?;
                println!("saved!");
                break;
            }
        }

        Ok(())
    }

    pub fn logout(&mut self) -> anyhow::Result<()> {
        print!("are you sure you want to log out? [Y/n] ");
        io::stdout().flush()?;

        for line in io::stdin().lock().lines() {
            let line = line?.to_lowercase();

            if line == "n" {
                println!("not logged out");
                process::exit(0);
            } else {
                self.config.api_key = String::new();
                confy::store(&self.config_folder, Some(&*self.config_file), &self.config)?;
                println!("logged out, goodbye!");
                break;
            }
        }

        Ok(())
    }
}
