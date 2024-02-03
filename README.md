# IP Sniffer

IP Sniffer is a simple Rust-based command-line tool for scanning open ports on a given IP address using multiple threads. This project is designed to provide a quick and efficient way to identify open ports on a target machine.

## Usage

The tool supports the following command-line arguments:

- `ip_sniffer -h`: Display help message.
- `ip_sniffer [IP_ADDRESS]`: Scan default ports using 4 threads.
- `ip_sniffer -j [THREAD_COUNT] [IP_ADDRESS]`: Scan ports using the specified number of threads.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)

### How to Compile

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/sahilwep/ip_sniffer
cd ip_sniffer
```
Compile the project using Rust's package manager, Cargo:
```bash
cargo build --release
```

### How to Run
Run the compiled executable with the desired command-line arguments:

```bash
./target/release/ip_sniffer [ARGUMENTS]
```

### Command-line Arguments
* **Default Scan**: Provide an IP address to scan default ports with 4 threads.

```bash
./target/release/ip_sniffer 192.168.1.1
```

* **Custom Thread Count**: Use the -j option to specify the number of threads.


```bash
./target/release/ip_sniffer -j 1000 192.168.1.1
```

* **Help Message**: Display the help message.

```bash
./target/release/ip_sniffer -h
```

### Example 

```bash
# Scan default ports on 192.168.1.1 with 4 threads
./target/release/ip_sniffer 192.168.1.1

# Scan ports on 192.168.1.1 with 1000 threads
./target/release/ip_sniffer -j 1000 192.168.1.1

# Display help message
./target/release/ip_sniffer -h

```

## Notes

* The tool uses a multi-threaded approach for faster port scanning.
* The maximum number of threads is capped at 65535.
* Results will be displayed, indicating open ports.

Feel free to contribute and enhance the functionality of this IP Sniffer tool. If you encounter any issues or have suggestions, please open an **issue**.