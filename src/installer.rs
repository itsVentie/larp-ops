use anyhow::{bail, Context, Result};
use futures_util::StreamExt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

pub async fn install_module(module_name: &str) -> Result<PathBuf> {
    let (url, filename) = match module_name {
        "evtx_dump" => (
            "https://github.com/WithSecureLabs/chainsaw/releases/download/v2.0.0/chainsaw_x86_64-pc-windows-msvc.zip",
            "evtx_dump.exe",
        ),
        "nmap" => (
            "https://nmap.org/dist/nmap-7.94-win32.zip",
            "nmap.exe",
        ),
        _ => bail!("Unknown module '{}'. Available modules: evtx_dump, nmap", module_name),
    };

    let tools_dir = PathBuf::from("tools");
    if !tools_dir.exists() {
        create_dir_all(&tools_dir)?;
    }

    let target_path = tools_dir.join(filename);
    println!("[*] Downloading module '{}' from {}", module_name, url);

    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to send request to {}", url))?;

    if !response.status().is_success() {
        bail!("Failed to download module: HTTP {}", response.status());
    }

    let mut file = File::create(&target_path)
        .with_context(|| format!("Failed to create file at {:?}", target_path))?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    println!(
        "[+] Module '{}' successfully installed to {:?}",
        module_name, target_path
    );
    Ok(target_path)
}
