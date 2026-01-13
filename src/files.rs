use crate::Neko;

impl Neko {
    pub async fn rename(&self, from: String, to: String) -> anyhow::Result<()> {
        let res = self.client.rename(&from, &to).await?;
        println!("{}", res.text().await?);
        Ok(())
    }

    pub async fn list(&self, dir: String) -> anyhow::Result<()> {
        let files = self.client.list(&dir).await?;

        files
            .iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .for_each(|(i, file)| {
                if file.dir {
                    print!("\x1b[1;34m{}\x1b[0m/", file.name.display());
                } else {
                    print!("{}", file.name.display());
                }

                print!("  ");

                if i % 5 == 0 {
                    print!("\n");
                }
            });

        if files.len() % 5 != 0 {
            println!();
        }

        Ok(())
    }

    pub async fn touch(&self, path: String) -> anyhow::Result<()> {
        let res = self.client.create_file(&path).await?;
        println!("{}", res.text().await?);
        Ok(())
    }

    pub async fn mkdir(&self, path: String) -> anyhow::Result<()> {
        let res = self.client.create_folder(&path).await?;
        println!("{}", res.text().await?);
        Ok(())
    }

    pub async fn remove(&self, path: String) -> anyhow::Result<()> {
        let res = self.client.delete(path).await?;
        println!("{}", res.text().await?);
        Ok(())
    }
}
