use anyhow::{Context, bail};
use std::{env, fs, path::PathBuf, process::Command};
use wasm_bindgen_cli_support::Bindgen;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let serve = match args.as_slice() {
        [] => false,
        [arg] if arg == "--serve" => true,
        _ => bail!("usage: build_wasm_ui [--serve]"),
    };

    let host_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = host_dir
        .parent()
        .context("host directory has no workspace parent")?;
    let public_dir = workspace_dir.join("public");
    let dist_dir = public_dir.join("dist");
    let target_dir = workspace_dir.join("target/web-ui");

    build_wasm(workspace_dir, &target_dir)?;

    fs::create_dir_all(&dist_dir)
        .with_context(|| format!("failed to create {}", dist_dir.display()))?;
    let wasm_file = target_dir.join("wasm32-unknown-unknown/release/wasm_ui.wasm");
    Bindgen::new()
        .input_path(&wasm_file)
        .out_name("ui")
        .web(true)
        .context("failed to configure wasm-bindgen for web output")?
        .generate(&dist_dir)
        .context("failed to generate WebAssembly browser bindings")?;

    fs::copy(public_dir.join("index.html"), dist_dir.join("index.html"))
        .context("failed to copy the UI HTML entry point")?;

    build_web_server(workspace_dir)?;
    if serve {
        run_web_server(workspace_dir)?;
    } else {
        println!("Web UI and host server built successfully.");
    }
    Ok(())
}

fn build_wasm(workspace_dir: &std::path::Path, target_dir: &std::path::Path) -> anyhow::Result<()> {
    let rustup = env::var_os("RUSTUP").unwrap_or_else(|| "rustup".into());
    let status = Command::new(rustup)
        .args([
            "run",
            "nightly",
            "cargo",
            "build",
            "--package",
            "host",
            "--bin",
            "wasm_ui",
            "--no-default-features",
            "--features",
            "wasm-ui",
            "--target",
            "wasm32-unknown-unknown",
            "--target-dir",
        ])
        .arg(target_dir)
        .args(["--release"])
        .current_dir(workspace_dir)
        .status()
        .context("failed to start Cargo for the GPUI WebAssembly build")?;
    if !status.success() {
        bail!("GPUI WebAssembly build failed with {status}");
    }

    Ok(())
}

fn build_web_server(workspace_dir: &std::path::Path) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args([
            "build",
            "--package",
            "host",
            "--bin",
            "web",
            "--no-default-features",
            "--features",
            "web-server",
        ])
        .current_dir(workspace_dir)
        .status()
        .context("failed to start Cargo for the ntex host build")?;
    if !status.success() {
        bail!("ntex host build failed with {status}");
    }

    Ok(())
}

fn run_web_server(workspace_dir: &std::path::Path) -> anyhow::Result<()> {
    println!("Serving the Web UI at http://127.0.0.1:3000/");
    let status = Command::new("cargo")
        .args([
            "run",
            "--package",
            "host",
            "--bin",
            "web",
            "--no-default-features",
            "--features",
            "web-server",
        ])
        .current_dir(workspace_dir)
        .status()
        .context("failed to start the ntex Web UI server")?;
    if !status.success() {
        bail!("ntex Web UI server exited with {status}");
    }

    Ok(())
}
