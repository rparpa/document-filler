use reqwest::Result;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    let template_path = "https://www.formulaires.service-public.fr/gf/cerfa_14952.do";

    let tmp_dir = Builder::new()
        .prefix("document-filler")
        .tempdir()
        .expect("Error creating temporary directory");

    let mut response = reqwest::get(template_path).await?.bytes().await?;

    let fname = "procuration_template.pdf";
    println!("file to download: '{}'", fname);
    let destname = tmp_dir.path().join(fname);
    println!("will be located under: '{:?}'", destname);
    let mut dest = File::create(destname).expect("File not created");
    dest.write_all(&mut response)
        .expect("Error copying to file");

    println!("Copy file worked");

    thread::sleep(time::Duration::from_secs(300));

    Ok(())
}
