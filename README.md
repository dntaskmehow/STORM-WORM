<div align="center">

# STORM WORM

<img src="https://readme-typing-svg.demolab.com?font=Orbitron&size=28&pause=1000&color=CC00FF&center=true&vCenter=true&width=700&lines=Researcher+dontaskmehow;Supply-Chain+Worm+Framework" alt="DOMONO banner" />

[![Status](https://img.shields.io/badge/Status-Research-red)](https://github.com/dntaskmehow/DOMONO)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20IoT-00F5FF)](https://github.com/dntaskmehow/DOMONO)
[![Stack](https://img.shields.io/badge/Stack-Go%20%2B%20Rust-FFA500)](https://github.com/dntaskmehow/DOMONO)

</div>

`STORM-WORM` is a zero-day supply-chain worm framework written in Rust, designed to model self-replicating malicious behavior at the kernel boundary through a stealth-focused, persistence-heavy execution chain.

The project is framed around a supply-chain compromise model involving self-spreading execution, kernel-level access, eBPF-based rootkit activity, and stealthy credential collection in hostile environments.

----------------------------------------

##  THREAT OVERVIEW 

This framework is engineered to explore:

- supply-chain propagation patterns
- self-replication and lateral spread
- kernel-level persistence strategies
- eBPF rootkit-style stealth techniques
- information theft and credential harvesting
- evasive execution in constrained environments

### Capability profile

- Primary language: Rust
- Execution model: Self-replicating worm
- Persistence: Kernel-level / stealth-oriented
- Control plane: rootkit / evasive execution primitives
- Data theft: infostealer behavior
- Attack vector: supply-chain compromise / post-infection propagation

----------------------------------------

##  CORE CAPABILITIES 

### 1. Self-Replication
Designed to spread across trust boundaries through a supply-chain compromise model, enabling recursive propagation and chain amplification.

### 2. Kernel-Level Access
Explores privileged execution paths and post-compromise persistence at the operating system boundary.

### 3. eBPF Rootkit Behavior
Focuses on stealthy execution through eBPF-based instrumentation, low-level visibility manipulation, and tamper-resistant runtime control.

### 4. Infostealer Core
Captures system and user data with a focus on credential material, session artifacts, and environment intelligence.

### 5. Supply-Chain Weaponization
Targets trust dependencies and upstream delivery chains to maximize propagation and exploit reach.

----------------------------------------

##  OPERATIONAL FLOW 

1. Establish initial foothold through a supply-chain path
2. Execute self-replication logic
3. Elevate runtime privileges where possible
4. Deploy rootkit / stealth layer
5. Harvest credentials and system intelligence
6. Propagate to additional dependency or trust surfaces
7. Maintain persistence and stealth across the host

----------------------------------------

##  INSTALL & RUN 

### Requirements

- Rust toolchain
- Linux environment for kernel / eBPF testing
- Authorized lab or controlled testing environment only

### Build

```bash
cargo build --release
```
Run

```bash
./target/release/storm_worm
```
Optional debug build:

```bash
cargo run
```

---

*DISCLAIMER* ⚠️

**This repository is presented for security research, and defensive analysis in authorized environments only. Use responsibly and in compliance with applicable laws and organizational policy.**
STORM-WORM is a Rust-based supply-chain worm framework centered on stealth persistence, kernel-level access, and eBPF rootkit behavior for research and defensive analysis scenarios.
---

<img src="https://readme-typing-svg.demolab.com?font=Orbitron&size=28&pause=1000&color=CC00FF&center=true&vCenter=true&width=700&lines=Researcher+dontaskmehow;Supply-Chain+Worm+Framework" alt="DOMONO banner" />
