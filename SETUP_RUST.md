# SETUP_RUST.md

## Project Setup and Testing

The following steps describe how to set up and test the Rust project to run in the background and start automatically upon user login.

### 1. Build the Project

First, ensure the project is built correctly:

```bash
cd /Users/yourusername/url_converter
cargo build --release
```

### 2. Configure LaunchAgent for Auto-Start

To ensure the service starts automatically on system startup, create a LaunchAgent plist file:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rust.easydict_server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/yourusername/url_converter/target/release/easydict_server</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

Save this file as `~/Library/LaunchAgents/com.rust.easydict_server.plist`.

Load the LaunchAgent with the following command:

```bash
launchctl load ~/Library/LaunchAgents/com.rust.easydict_server.plist
```

### 3. Verify the Program is Running

Check if the program is running using the following command:

```bash
ps aux | grep easydict_server
```

### 4. Stop the Server Manually

If needed, you can stop the server manually by finding its process ID and killing it:

```bash
ps aux | grep easydict_server
kill <process_id>
```

With these steps, you can ensure your Rust project starts automatically upon user login and runs in the background. You can easily start and stop it as needed.

## Troubleshooting

- If the program does not start correctly, check the log file `/tmp/urlconverter.log` for error messages.
- Ensure the paths in the script are correct.
- Ensure all relevant files and directories have the correct permissions.

### Common Issues

#### 1. Log File is Empty or Missing

- Check if the paths in the script are correct.
- Ensure the program has permission to write to the log file.

#### 2. Program Not Running in the Background

- Ensure the command in the script is correct and the script is executable.
- Ensure the `&` symbol is used to run the program in the background.

### Project Directory Structure

The simplified project directory structure after these changes is:

```
url_converter/
├── Cargo.toml
├── SETUP_RUST.md
├── src/
│   └── main.rs
├── target/
    └── release/
```

By following this guide, you should be able to set up and run your URL Converter service efficiently. If you encounter any issues, refer to the troubleshooting section for common problems and solutions.