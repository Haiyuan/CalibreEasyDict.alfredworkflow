# URL Converter Service Setup Guide

This project sets up a local HTTP server that listens for requests of the form `http://localhost:8080/?text={word}` and converts them into `easydict://query?text={word}`, which is then opened by the default web browser. The service is configured to run at startup on MacOS.

### Project Directory Structure

```
url_converter/
├── Cargo.toml
├── SETUP_RUST.md
├── src/
│   └── main.rs
├── get_mouse_position.scpt
└── get_mouse_position.py
```

### Cargo.toml

```toml
[package]
name = "url_converter"
version = "0.1.0"
edition = "2018"

[dependencies]
dirs = "4.0"
url = "2.2"
webbrowser = "0.5"
urlencoding = "2.1"
```

### SETUP_RUST.md

```markdown
# SETUP_RUST.md

## Project Setup and Testing

The following steps describe how to set up and test the Rust project to run in the background and start automatically upon user login.

### 1. Build the Project

First, ensure the project is built correctly:

```bash
cd /Users/yourusername/url_converter
cargo build --release
```

### 2. Create Launch Agent Configuration File

Create a file in the `~/Library/LaunchAgents` directory, for example, `com.user.urlconverter.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.urlconverter</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/yourusername/url_converter/target/release/easydict_server</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/urlconverter.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/urlconverter.err</string>
</dict>
</plist>
```

### 3. Load and Start the Launch Agent

Load and start the Launch Agent using the following command:

```bash
launchctl load ~/Library/LaunchAgents/com.user.urlconverter.plist
```

### 4. Verify the Program is Running

Check if the program is running using the following command:

```bash
ps aux | grep easydict_server
```

### 5. Stop the Launch Agent

Stop and unload the Launch Agent using the following command:

```bash
launchctl unload ~/Library/LaunchAgents/com.user.urlconverter.plist
```

With these steps, you can ensure your Rust project starts automatically upon user login and runs in the background. You can easily start and stop it using `launchctl` commands.

## Troubleshooting

- If the program does not start correctly, check the log files `/tmp/urlconverter.log` and `/tmp/urlconverter.err` for error messages.
- Ensure the `com.user.urlconverter.plist` file path and the script path are correct.
- Ensure all relevant files and directories have the correct permissions.

### Common Issues

#### 1. Log File is Empty or Missing

- Check if the paths in the `.plist` file are correct.
- Ensure the program has permission to write to the log files.

#### 2. Program Not Running in the Background

- Ensure the command in the `.plist` file is correct and the script is executable.
- Ensure the `&` symbol is used to run the program in the background.

#### 3. Launch Agent Not Starting Automatically

- Ensure the `.plist` file is placed in the `~/Library/LaunchAgents` directory.
- Ensure the `.plist` file has the correct permissions:
  ```bash
  chmod 644 ~/Library/LaunchAgents/com.user.urlconverter.plist
  ```

#### 4. Unable to Load or Unload Launch Agent

- Ensure you are using the correct commands to load or unload:
  ```bash
  launchctl load ~/Library/LaunchAgents/com.user.urlconverter.plist
  launchctl unload ~/Library/LaunchAgents/com.user.urlconverter.plist
  ```

### References

- [launchctl Manual](https://www.manpagez.com/man/1/launchctl/)
- [Creating Launch Daemons and Agents](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html)
```

### src/main.rs

```rust
use std::net::TcpListener;
use std::io::{Read, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use url::Url;
use webbrowser;
use dirs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Starting http server...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        if let Some(word) = parse_word_from_request(&request) {
            let encoded_word = urlencoding::encode(&word).to_string();
            let mouse_position = get_mouse_position();
            record_current_window();
            let easydict_url = format!("easydict://query?text={}", encoded_word);
            webbrowser::open(&easydict_url).unwrap();
            sleep(Duration::from_secs(1));

            if word.split_whitespace().count() >= 2 {
                handle_special_cases();
                sleep(Duration::from_secs(7));
            }

            switch_back_to_previous_window();
            restore_mouse_position(&mouse_position);

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nURL has been converted and opened.";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let response = "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/html\r\n\r\nMissing query text.";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn parse_word_from_request(request: &str) -> Option<String> {
    let url = request.lines().next()?.split_whitespace().nth(1)?;
    let parsed_url = Url::parse(&format!("http://localhost{}", url)).ok()?;
    let query_pairs = parsed_url.query_pairs();
    for (key, value) in query_pairs {
        if key == "text" {
            return Some(value.into_owned());
        }
    }
    None
}

fn get_mouse_position() -> (i32, i32) {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let script_path = home_dir.join("url_converter/get_mouse_position.scpt");

    let output = Command::new("osascript")
        .arg(script_path)
        .output();

    match output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Mouse position raw output: {}", result); // Print raw output for debugging
            println!("Mouse position error output: {}", error); // Print error output for debugging

            if result.trim().is_empty() {
                panic!("Failed to get mouse position: empty output");
            }

            let coords: Vec<&str> = result.trim().split(',').collect();

            if coords.len() != 2 {
                panic!("Failed to get mouse position: {}", result);
            }

            let x: i32 = match coords[0].trim().parse() {
                Ok(val) => val,
                Err(e) => panic!("Failed to parse X coordinate: {:?}", e),
            };

            let y: i32 = match coords[1].trim().parse() {
                Ok(val) => val,
                Err(e) => panic!("Failed to parse Y coordinate: {:?}", e),
            };

            (x, y)
        }
        Err(e) => {
            panic!("Failed to execute AppleScript: {:?}", e);
        }
    }
}

fn record_current_window() {
    let script = r#"
        on performKeyPress(commandKey, optionKey, controlKey, keyCode)
            tell application "System Events"
                if commandKey then key down command
                if optionKey then key down option
                if controlKey then key down control
                key code keyCode
                if controlKey then key up control
                if optionKey then key up option
                if commandKey then key up command
            end tell
        end performKeyPress

        performKeyPress(true, true, true, 1)
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute AppleScript");
}

fn handle_special_cases() {
    let script = r#"
        on performKeyPress(commandKey, optionKey, controlKey, keyCode)
            tell application "System Events"
                if commandKey then key down command
                if optionKey then key down option
                if controlKey then key down control
                key code keyCode
                if controlKey then key up control
                if optionKey then key up option
                if commandKey then key up command
            end tell
        end performKeyPress

        tell application "EasyDict" to activate
        performKeyPress(true, true, false, 1)
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute AppleScript");
}

fn switch_back_to_previous_window() {
    let script = r#"
        on performKeyPress(commandKey, optionKey, controlKey, keyCode)
            tell application "System Events"
                if commandKey then key down command
                if optionKey then key down option
                if controlKey then key down control
                key code keyCode
                if controlKey then key up control
                if optionKey then key up option
                if commandKey then key up command
            end tell
        end performKeyPress

        performKeyPress(true, true, true, 15)
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute AppleScript");
}

fn restore_mouse_position(position: &(i32, i32)) {
    let script = format!(
        r#"
        on restoreMousePosition(mouseX, mouseY)
            set pythonScript to "import sys
        from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, CGEventPost
        import Quartz.CoreGraphics as CG

        mousePositionX = float(sys.argv[1])
        mousePositionY = float(sys.argv[2])

        ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (mousePositionX, mousePositionY), 0)
        CGEventPost(0, ourEvent)"
            set shellScript to "~/myenv/bin/python -c " & quoted form of pythonScript & " " & mouseX & " " & mouseY
            do shell script shellScript
        end restoreMousePosition

        restoreMousePosition({}, {})
    "#,
        position.0, position.1
    );

    Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .expect("Failed to execute AppleScript");
}
```

### get_mouse_position.scpt

```applescript
on getMousePosition()
    set mousePositionScript to "~/myenv/bin/python ~/url_converter/get_mouse_position.py"
    set mousePosition to do shell script mousePositionScript
    set AppleScript's text item delimiters to ","
    set {mousePositionX, mousePositionY} to text items of mousePosition
    return {mousePositionX, mousePositionY}
end getMousePosition

getMousePosition()
```

### get_mouse_position.py

```python
import Quartz.CoreGraphics as CG

loc = CG.CGEventGetLocation(CG.CGEventCreate(None))
print(int(loc.x), int(loc.y))
```