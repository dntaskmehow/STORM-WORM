# STORM WORM

A focused research archive for studying supply-chain trust failures, dependency poisoning patterns, and defensive controls in a controlled lab environment.

## Overview

This folder contains a small set of sample artifacts that model a fictional multi-stage supply-chain attack chain:

- a Rust-based implant simulation
- a Go-based command-and-control listener
- an npm package poisoning example

The material is organized as a lab reference for security analysis, threat modeling, and defensive review. It is intended only for isolated, authorized research and should never be deployed outside a controlled environment.

## Folder Contents

### 1) storm_worm.rs
A Rust-based simulation of an implant that demonstrates:

- environment reconnaissance
- credential collection patterns
- command-and-control exfiltration behavior
- replication-style supply-chain behavior
- persistence-style looping execution patterns

This file is intended to support malware-analysis study and defensive detection work in a restricted lab context.

### 2) c2_server.go
A lightweight Go listener that models a simple exfiltration endpoint. It accepts data payloads and logs incoming records for analysis.

This file represents the control-plane side of a simulated attack chain and helps illustrate how exfiltrated data is received and handled.

### 3) npm_poisoner.js
A Node.js script that creates and publishes a malicious package metadata structure designed to simulate dependency poisoning. It shows how trust assumptions in package ecosystems can be abused in unsafe or misconfigured environments.

This artifact is useful for studying package integrity, installation-time execution, and supply-chain defense controls.

## Intended Use

This archive is best used for:

- threat-model walkthroughs
- malware triage and behavioral analysis
- dependency trust review
- defensive controls evaluation
- educational red-team / blue-team lab work

## Safety and Scope

This project is provided as a controlled research artifact only.

It must not be used for:

- real-world attacks
- credential theft or exfiltration against third parties
- unauthorized access or production deployment
- public exploitation guidance
- any activity outside an approved lab environment

## Research Framing

The purpose is not to enable operational misuse, but to understand:

- how trust boundaries collapse in supply chains
- how package metadata and install hooks can become abuse vectors
- how detection and prevention controls can be strengthened
- how defensive engineering can reduce risk across dependency flows

## Suggested Review Flow

1. Start with the Rust implant and identify the behavioral stages.
2. Review the exfiltration model in the Go server.
3. Examine the npm package-poisoning example and its trust assumptions.
4. Map the findings to defensive controls: signing, verification, allowlists, sandboxing, and provenance checks.
5. Validate conclusions only in an isolated, authorized lab.

## Status

Storm Worm research archive initialized.

Prepared for controlled defensive security review, malware analysis, and isolated lab evaluation only.

---

For authorized security research only. No unauthorized targeting, no real-world deployment, and no misuse outside a governed testing environment.
