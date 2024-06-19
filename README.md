# CalibreEasyDict.alfredworkflow

A workflow for Alfred that allows you to switch between specified special applications using keyboard shortcuts and restore the mouse position. Additionally, it includes custom alerts and hotkey bindings through Hammerspoon. The project also sets up a local HTTP server for converting text queries to EasyDict URL schemes, facilitating translation services, especially useful for looking up words or sentences while reading books in Calibre.

## Table of Contents

- [CalibreEasyDict.alfredworkflow](#calibreeasydictalfredworkflow)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Configuration](#configuration)
  - [Scripts](#scripts)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

## Features

- Switch between specified special applications using keyboard shortcuts.
- Restore mouse position after switching applications.
- Trigger specific actions within the special applications.
- Customizable alert styles and hotkey bindings via Hammerspoon.
- Local HTTP server to convert text queries to EasyDict URL schemes.

## Installation

### Prerequisites

- Alfred 4 or higher with the Powerpack.
- Hammerspoon for additional hotkey bindings and alerts.
- EasyDict installed via Homebrew:
    ```sh
    brew install --cask Easydict
    ```

### Steps

1. **Clone the repository:**
    ```sh
    git clone https://github.com/Haiyuan/CalibreEasyDict.alfredworkflow.git
    cd CalibreEasyDict.alfredworkflow
    ```

2. **Build the Rust project:**
    ```bash
    cd /Users/yourusername/url_converter
    cargo build --release
    ```

3. **Import the workflow into Alfred:**
    - Open Alfred Preferences.
    - Go to the "Workflows" tab.
    - Drag and drop the downloaded `CalibreEasyDict.alfredworkflow` file into the workflow list.

4. **Set up Hammerspoon:**
    - Install [Hammerspoon](https://www.hammerspoon.org/).
    - Copy the provided Hammerspoon configuration to your Hammerspoon config file (`~/.hammerspoon/init.lua`).

5. **Set up the local HTTP server for URL conversion:**
    - Install Homebrew if not already installed:
      ```sh
      /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
      ```
    - Create and configure the Rust project as per the provided `Setup Guide for EasyDict Server Project`.

6. **Configure Launchd to run the Rust server at startup:**
    - Create a LaunchAgent plist file:
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
    - Save this file as `~/Library/LaunchAgents/com.rust.easydict_server.plist`.
    - Load the LaunchAgent with the following command:
      ```bash
      launchctl load ~/Library/LaunchAgents/com.rust.easydict_server.plist
      ```

7. **Verify the service:**
    ```sh
    launchctl list | grep com.rust.easydict_server
    ```

## Usage

1. **Trigger the workflow:**
    - Use the assigned keyword in Alfred to trigger the workflow (e.g., type `switchapp`).

2. **Keyboard Shortcuts:**
    - `Cmd + Alt + Ctrl + S`: Save the current window (Hammerspoon Hotkey).
    - `Cmd + Alt + Ctrl + R`: Switch back to the previous window (Hammerspoon Hotkey).
    - `Cmd + Alt + Ctrl + D`: Select text to translate in EasyDict.
    - `Cmd + Alt + S`: Play the selected translation in EasyDict.
    - `Cmd + P`: Pin the EasyDict window.

3. **Restore Mouse Position:**
    - The workflow captures the mouse position before switching applications and restores it afterward.

4. **Hammerspoon Hotkeys:**
    - Save the current window: `Cmd + Alt + Ctrl + S`
    - Switch back to the previous window: `Cmd + Alt + Ctrl + R`

5. **Use Calibre Lookup:**
    - In `Calibre`, when you select a word and use the lookup feature, the configured custom source will send a request to `http://localhost:8082/?text={word}`.
    - The local service will convert this request to `easydict://query?text={word}` and open it in the default web browser.

## Configuration

### Hammerspoon Alerts

You can customize the alert styles and hotkey bindings in your Hammerspoon configuration:

```lua
-- Custom alert styles
hs.alert.defaultStyle.strokeWidth = 0
hs.alert.defaultStyle.textSize = 18
hs.alert.defaultStyle.fillColor = { white = 0, alpha = 0.75 }
hs.alert.defaultStyle.strokeColor = { white = 0, alpha = 0 }
hs.alert.defaultStyle.textColor = { white = 1 }
hs.alert.defaultStyle.radius = 10
hs.alert.defaultStyle.atScreenEdge = 2  -- Display at screen edge

-- Define a variable to save the previous window
previousWindow = nil

-- Define a hotkey to save the current window
hs.hotkey.bind({"cmd", "alt", "ctrl"}, "S", function()
    previousWindow = hs.window.focusedWindow()
    hs.alert.show("Current window saved")
end)

-- Define a hotkey to switch back to the previous window
hs.hotkey.bind({"cmd", "alt", "ctrl"}, "R", function()
    if previousWindow then
        previousWindow:focus()
        hs.alert.show("Switched back to previous window")
    else
        hs.alert.show("No previous window")
    end
end)
```

## Scripts

### AppleScript

The main AppleScript handles application switching, keypress simulation, and interaction with the Rust server.

```applescript
-- Embed special_apps.json data directly
set specialAppsJson to "{\"special_apps\": [\"calibre-parallel\", \"sublime_text\", \"sublime_merge\", \"Electron\"]}"
set specialApps to {"calibre-parallel", "sublime_text", "sublime_merge", "Electron"}

-- Global variables
property frontApp : ""
property mousePositionX : ""
property mousePositionY : ""

-- Get the name of the current application
on getCurrentAppName()
    tell application "System Events"
        return name of first application process whose frontmost is true
    end tell
end getCurrentAppName

-- Key press function
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

-- Switch back to the previous application
on switchToApp(appName, keyCode)
    performKeyPress(true, true, true, keyCode)
end switchToApp

-- Get the mouse position
on getMousePosition()
    set mousePositionScript to "~/myenv/bin/python -c 'import Quartz.CoreGraphics as CG; loc = CG.CGEventGetLocation(CG.CGEventCreate(None)); print(int(loc.x), int(loc.y))'"
    set mousePosition to do shell script mousePositionScript
    set AppleScript's text item delimiters to " "
    set {mousePositionX, mousePositionY} to text items of mousePosition
    return {mousePositionX, mousePositionY}
end getMousePosition

-- Restore the mouse position
on restoreMousePosition(mouseX, mouseY)
    set pythonScript to "import sys
from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, CGEventPost
import Quartz.CoreGraphics as CG

mousePosition = float(sys.argv[1])
mousePositionY = float(sys.argv[2])

ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (mousePosition, mousePositionY), 0)
CGEventPost(0, ourEvent)"
    set shellScript to "~/myenv/bin/python -c " & quoted form of pythonScript & " " & mouseX & " " & mouseY
    do shell script shellScript
end restoreMousePosition

-- Check EasyDict window
on checkEasyDictWindow()
    set appNameSubstring to "EasyDict"
    set targetProcess to ""

    tell application "System Events"
        set allProcesses to (name of every process)
    end tell

    repeat with processName in allProcesses
        if processName contains appNameSubstring then
            set targetProcess to processName
            exit repeat
        end if
    end repeat

    if targetProcess is "" then
        return 0
    else
        try
            tell application "System Events"
                set easyDictApp to first process whose name is targetProcess
                set easyDictWindow to window 1 of easyDictApp
                return 1
            end tell
        on error
            return 0
        end try
    end if
end checkEasyDictWindow

-- Execute first logic
on executeFirstLogic(specialApps)
    set frontApp to getCurrentAppName()

    if frontApp is in specialApps then
        switchToApp(frontApp, 1)
    end if

    set myGlobalVar to checkEasyDictWindow()
    performKeyPress(true, true, true, 2)

    if myGlobalVar is 0 then
        delay 0.1
        tell application "EasyDict" to activate
        delay 0.5
        performKeyPress(true, false, false, 35)
    end if

    set {mousePositionX, mousePositionY} to getMousePosition()

    -- Save global variables
    set frontApp to frontApp
    set mousePositionX to mousePositionX
    set mousePositionY to mousePositionY

    if frontApp is in specialApps then
        switchToApp(frontApp, 15)
    else
        tell application frontApp to activate
    end if

    try
        restoreMousePosition(mousePositionX, mousePositionY)
    on error errMsg number errNum
        -- Error handling
    end try
end executeFirstLogic

-- Execute second logic
on executeSecondLogic(specialApps)
    set frontApp to getCurrentAppName()

    if frontApp is in specialApps then
        switchToApp(frontApp, 1)
    end if

    set {mousePositionX, mousePositionY} to getMousePosition()

    -- Save global variables
    set frontApp to frontApp
    set mousePositionX to mousePositionX
    set mousePositionY to mousePositionY

    tell application "EasyDict" to activate
    performKeyPress(true, true, false, 1)
    delay 0.5

    if frontApp is in specialApps then
        switchToApp(frontApp, 15)
    else
        tell application frontApp to activate
    end if

    try
        restoreMousePosition(mousePositionX, mousePositionY)
    on error errMsg number errNum
        -- Error handling
    end try
end executeSecondLogic

-- Call main logic
executeFirstLogic(specialApps)
delay 3
executeSecondLogic(specialApps)
```

### Rust Script for URL Conversion

The Rust script for converting text queries to EasyDict URL schemes and handling mouse and window interactions:

```rust
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
    println!("Starting HTTP server on port 8082...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    use std::io::Read;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /?text=";
    if buffer.starts_with(get) {
        let word_start = get.len();
        let word_end = buffer.iter().position(|&r| r == b' ').unwrap_or(buffer.len());
        let word = &buffer[word_start..word_end];
        let word = String::from_utf8_lossy(word).to_string();

        if word.is_empty() {
            let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\nMissing query text.";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }

        let encoded_word = urlencoding::encode(&word);
        let easydict_url = format!("easydict://query?text={}", encoded_word);
        open::that(easydict_url).unwrap();
        thread::sleep(Duration::from_secs(1));

        if word.split_whitespace().count() >= 2 {
            handle_special_cases();
            thread::sleep(Duration::from_secs(7));
        }

        switch_back_to_previous_window();

        let response = "HTTP/1.1 200 OK\r\n\r\nURL has been converted and opened.";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_special_cases() {
    let script = r#"
    tell application "EasyDict"
        activate
    end tell
    tell application "System Events"
        key code 1 using {command down, option down}
    end tell
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("failed to execute script");
}

fn switch_back_to_previous_window() {
    let script = r#"
    tell application "System Events"
        key code 15 using {command down, option down, control down}
    end tell
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("failed to execute script");
}
```

## Contributing

We welcome contributions to improve this workflow:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Alfred](https://www.alfredapp.com/) - The macOS productivity application.
- [Hammerspoon](https://www.hammerspoon.org/) - For additional hotkey bindings and alerts.
- [Quartz](https://pypi.org/project/pyobjc-framework-Quartz/) - Used for mouse position handling in Python.
- [EasyDict](https://github.com/tisfeng/Easydict?tab=readme-ov-file#url-scheme) - For translation services.
- Special thanks to all contributors and resources used.

---

This comprehensive guide should help you set up and run the CalibreEasyDict workflow and the URL converter service. If you encounter any issues or need further assistance, please feel free to ask.