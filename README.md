# DDNS Updater

A simple Rust-based Dynamic DNS (DDNS) updater that fetches the external IP address and updates the corresponding DNS record via Cloudflare's API. The program runs as a service on an Ubuntu system and checks for IP changes every minute. If a change is detected, it updates the DNS record.

## Features

- Automatically fetches the current external IP address.
- Updates Cloudflare DNS records only when the IP address changes.
- Runs continuously as a service using systemd.
- Logs changes and errors to the console.

## Requirements

- Rust
- Cargo package manager
- A Cloudflare API token with DNS edit permissions

## Installation

### 1. Clone the Repository
```bash
git clone https://github.com/ken890728/ddns_updater.git
cd ddns_updater
```

### 2. Configure Environment Variables
Create a `.env` file in the project directory with the following content:
```env
CLOUDFLARE_API_TOKEN=your_cloudflare_api_token
CLOUDFLARE_ZONE_ID=your_zone_id
CLOUDFLARE_RECORD_ID=your_record_id
DNS_RECORD_NAME=your_domain_name
```

### 3. Build the Project
Run the following commands to build the project in release mode:
```bash
cargo build --release
```
The compiled binary will be located in `./target/release/ddns-updater`.

## Usage

### Run Manually
To test the program, you can run it directly:
```bash
./target/release/ddns_updater
```

### Install as a Service (Linux)
To ensure the program runs automatically, create a `systemd` service file.

1. Create a service file at `/etc/systemd/system/ddns_updater.service`:
   ```ini
   [Unit]
   Description=DDNS Updater Service
   After=network.target

   [Service]
   ExecStart=/path/to/ddns_updater/target/release/ddns_updater
   WorkingDirectory=/path/to/ddns_updater
   Restart=always
   EnvironmentFile=/path/to/ddns_updater/.env

   [Install]
   WantedBy=multi-user.target
   ```

2. Reload `systemd` and enable the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable ddns_updater
   sudo systemctl start ddns_updater
   ```

3. Check the service status:
   ```bash
   sudo systemctl status ddns_updater
   ```

### Logs
The program logs updates and errors to the console. If running as a service, logs can be viewed with:
```bash
journalctl -u ddns_updater -f
```

## Project Structure

- `src/main.rs`: The main program logic, including IP fetching and DNS updates.
- `.env`: Environment variables for configuration.
- `Cargo.toml`: Dependency and metadata configuration for the Rust project.

## Dependencies

- [`reqwest`](https://docs.rs/reqwest): For making HTTP requests.
- [`tokio`](https://docs.rs/tokio): For asynchronous execution and timers.
- [`serde`](https://docs.rs/serde): For JSON serialization/deserialization.
- [`dotenv`](https://docs.rs/dotenv): For loading environment variables from a file.

## Notes

- Ensure your Cloudflare API token has sufficient permissions to edit DNS records.
- The program only updates the DNS record if the external IP changes, reducing unnecessary API calls.

## Credits
This project was developed with the assistance of ChatGPT for code generation, optimization, and documentation.

## License

This project is licensed under the MIT License. See `LICENSE` for details.
