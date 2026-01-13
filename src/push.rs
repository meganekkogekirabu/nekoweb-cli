use crate::Neko;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::{self, PathBuf},
};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

impl Neko {
    pub async fn push(&self, dir: impl Into<PathBuf>) -> anyhow::Result<()> {
        let dir = path::absolute(dir.into())?;
        let tmp = env::temp_dir();
        env::set_current_dir(tmp)?;

        let mut output = File::create("nekoweb.zip")?;
        let mut zipper = ZipWriter::new(&mut output);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        for entry in WalkDir::new(&dir).min_depth(1) {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.strip_prefix(&dir).unwrap();

            if path.is_file() {
                zipper.start_file(name.to_string_lossy(), options)?;
                let mut file = File::open(path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                zipper.write_all(&buf)?;
            } else {
                zipper.add_directory(name.to_string_lossy(), options)?;
            }
        }

        zipper.finish()?;

        let stream = tokio::fs::File::open("nekoweb.zip").await?;
        self.client.import_stream(stream).await?;

        Ok(())
    }
}
