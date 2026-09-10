# PortsDeScan

A fast, concurrent port scanner written in Rust, built for personal usage on CTFs and Pentesting.

## Features

* **Dual Scanning Modes:**
  * **Fast Mode:** Quickly scans the first 1,000 standard ports plus a curated list of common service ports.
  * **Full Mode:** Performs a comprehensive scan across all ports from `1` to `65535`.
* **CIDR & IP Resolution:** Easily scan single IP addresses or entire subnets.
* **Concurrent Execution:** Utilizes multi-threading to speed up scanning while handling socket timeouts efficiently.

---

##  Installation & Usage

### Option 1: Download Pre-compiled Binary (Easiest)
Head over to the [Releases](https://github.com/YOUR_USERNAME/rusty_ports/releases) page, download the binary for your system, and run it:

```bash
chmod +x rusty_ports
./rusty_ports