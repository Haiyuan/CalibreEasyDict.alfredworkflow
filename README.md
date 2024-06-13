# EasyDictHelper.alfredworkflow

A workflow for Alfred that allows you to switch between specified special applications using keyboard shortcuts and restore the mouse position. Additionally, it includes custom alerts and hotkey bindings through Hammerspoon. The project also sets up a local HTTP server for converting text queries to EasyDict URL schemes, facilitating translation services.

## Table of Contents

- [EasyDictHelper.alfredworkflow](#easydicthelperalfredworkflow)
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
- Python environment with Quartz installed (`pyobjc-framework-Quartz`).
- Hammerspoon for additional hotkey bindings and alerts.
- Virtual environment for Python at `~/myenv/bin/python`.
- EasyDict installed via Homebrew:
    ```sh
    brew install --cask Easydict
    ```

### Steps

1. **Clone the repository:**
    ```sh
    git clone https://github.com/Haiyuan/EasyDictHelper.alfredworkflow.git
    cd EasyDictHelper.alfredworkflow
    ```

2. **Set up the Python virtual environment:**
    ```sh
    python3 -m venv ~/myenv
    source ~/myenv/bin/activate
    ```

3. **Install the necessary dependencies:**
    ```sh
    pip install -r requirements.txt
    ```
    The `requirements.txt` file should contain:
    ```plaintext
    pyobjc-framework-Quartz==6.2
    ```

4. **Import the workflow into Alfred:**
    - Open Alfred Preferences.
    - Go to the "Workflows" tab.
    - Drag and drop the downloaded `EasyDictHelper.alfredworkflow` file into the workflow list.

5. **Set up Hammerspoon:**
    - Install [Hammerspoon](https://www.hammerspoon.org/).
    - Copy the provided Hammerspoon configuration to your Hammerspoon config file (`~/.hammerspoon/init.lua`).

6. **Set up the local HTTP server for URL conversion:**
    - Install Homebrew if not already installed:
      ```sh
      /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
      ```
    - Install Python via Homebrew:
      ```sh
      brew install python
      ```

7. **Create and configure the Python script:**
    - Create a directory for the script:
      ```sh
      mkdir -p ~/url_converter
      ```
    - Create and edit the `url_converter.py` file in the `~/url_converter` directory with the following content:
      ```python
      import http.server
      import urllib.parse
      import webbrowser
      import subprocess
      import time

      PERFORM_KEY_PRESS_SCRIPT = """
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
      """

      class RequestHandler(http.server.BaseHTTPRequestHandler):
          def do_GET(self):
              parsed_path = urllib.parse.urlparse(self.path)
              query = urllib.parse.parse_qs(parsed_path.query)
              word = query.get('text', [''])[0]

              if not word:
                  self.send_response(400)
                  self.send_header('Content-type', 'text/html')
                  self.end_headers()
                  self.wfile.write(b'Missing query text.')
                  return

              encoded_word = urllib.parse.quote(word)
              self.record_current_window()
              easydict_url = f"easydict://query?text={encoded_word}"
              webbrowser.open(easydict_url)
              time.sleep(1)

              if len(word.split()) >= 2:
                  self.handle_special_cases()
                  time.sleep(7)

              self.switch_back_to_previous_window()

              self.send_response(200)
              self.send_header('Content-type', 'text/html')
              self.end_headers()
              self.wfile.write(b'URL has been converted and opened.')

          def record_current_window(self):
              script = PERFORM_KEY_PRESS_SCRIPT + """
      performKeyPress(true, true, true, 1)
              """
              subprocess.run(["osascript", "-e", script])

          def handle_special_cases(self):
              script = PERFORM_KEY_PRESS_SCRIPT + """
      tell application "EasyDict" to activate
      performKeyPress(true, true, false, 1)
              """
              subprocess.run(['osascript', '-e', script])

          def switch_back_to_previous_window(self):
              script = PERFORM_KEY_PRESS_SCRIPT + """
      performKeyPress(true, true, true, 15)
              """
              subprocess.run(["osascript", "-e", script])

      def run(server_class=http.server.HTTPServer, handler_class=RequestHandler):
          server_address = ('', 8082)
          httpd = server_class(server_address, handler_class)
          print('Starting http server...')
          httpd.serve_forever()

      if __name__ == "__main__":
          run()
      ```

8. **Configure Launchd to run the script at startup:**
    - Create a directory for launch agents if it doesn't exist and create the configuration file:
      ```sh
      mkdir -p ~/Library/LaunchAgents
      nano ~/Library/LaunchAgents/com.user.urlconverter.plist
      ```
    - Add the following content, replacing `yourusername` with your actual username:
      ```xml
      <?xml version="1.0" encoding="UTF-8"?>
      <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
      <plist version="1.0">
      <dict>
          <key>Label</key>
          <string>com.user.urlconverter</string>
          <key>ProgramArguments</key>
          <array>
              <string>/Users/yourusername/myenv/bin/python3</string>
              <string>/Users/yourusername/url_converter/url_converter.py</string>
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
    - Load and start the service:
      ```sh
      launchctl load ~/Library/LaunchAgents/com.user.urlconverter.plist
      ```
    - Verify the service:
      ```sh
      launchctl list | grep com.user.urlconverter
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
    - The workflow captures the mouse position before switching applications and restores it afterward using Python scripts.

4. **Hammerspoon Hotkeys:**
    - Save the current window: `Cmd + Alt + Ctrl + S`
    - Switch back to the previous window: `Cmd + Alt + Ctrl + R`

5. **Run the Python Script:**
    - Ensure the Python script is running by executing the following command in the terminal:
      ```sh
      python3 ~/url_converter/url_converter.py
      ```

6. **Use Calibre Lookup:**
    - In `Calibre`, when you select a word and use the lookup feature, the configured custom source will send a request to `http://localhost:8082/?text={word}`.
    - The local service will convert this request to `easydict://query?text={word}` and open it in the default web browser.

## Configuration

### Hammerspoon Alerts

You can customize the alert styles and hotkey bindings in your Hammerspoon configuration:

```lua
-- Custom alert styles
hs.alert.defaultStyle.strokeWidth = 0
```lua
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

The main AppleScript handles application switching, keypress simulation, and interaction with Python scripts.

```applescript
-- Embed special_apps.json data directly
set specialAppsJson to "{\"special_apps\": [\"calibre-parallel\", \"sublime_text\", \"sublime_merge\", \"Electron\"]}"
set specialApps to {"calibre-parallel", "sublime_text", "sublime_merge", "Electron"}

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

-- Get mouse position
on getMousePosition()
    set mousePositionScript to "~/myenv/bin/python -c 'import Quartz.CoreGraphics as CG; loc = CG.CGEventGetLocation(CG.CGEventCreate(None)); print(int(loc.x), int(loc.y))'"
    set mousePosition to do shell script mousePositionScript
    set AppleScript's text item delimiters to " "
    set {mousePositionX, mousePositionY} to text items of mousePosition
    return {mousePositionX, mousePositionY}
end getMousePosition

-- Restore mouse position
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

-- Execute the first main logic
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

    try
        set the clipboard to frontApp & "," & mousePositionX & "," & mousePositionY
    on error errMsg
        -- Error handling
    end try

    set clipboardContent to the clipboard
    set AppleScript's text item delimiters to ","
    set {frontApp, mousePositionX, mousePositionY} to text items of clipboardContent

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

-- Execute the second main logic
on executeSecondLogic(specialApps)
    set frontApp to getCurrentAppName()

    if frontApp is in specialApps then
        switchToApp(frontApp, 1)
    end if

    set {mousePositionX, mousePositionY} to getMousePosition()

    try
        set the clipboard to frontApp & "," & mousePositionX & "," & mousePositionY
    on error errMsg
        -- Error handling
    end try

    tell application "EasyDict" to activate
    performKeyPress(true, true, false, 1)
    delay 0.5

    set clipboardContent to the clipboard
    set AppleScript's text item delimiters to ","
    set {frontApp, mousePositionX, mousePositionY} to text items of clipboardContent

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

### Python Script for URL Conversion

The Python script for converting text queries to EasyDict URL schemes and handling mouse and window interactions:

```python
import http.server
import urllib.parse
import webbrowser
import subprocess
import time

PERFORM_KEY_PRESS_SCRIPT = """
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
"""

class RequestHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        query = urllib.parse.parse_qs(parsed_path.query)
        word = query.get('text', [''])[0]

        if not word:
            self.send_response(400)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b'Missing query text.')
            return

        encoded_word = urllib.parse.quote(word)
        self.record_current_window()
        easydict_url = f"easydict://query?text={encoded_word}"
        webbrowser.open(easydict_url)
        time.sleep(1)

        if len(word.split()) >= 2:
            self.handle_special_cases()
            time.sleep(7)

        self.switch_back_to_previous_window()

        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'URL has been converted and opened.')

    def record_current_window(self):
        script = PERFORM_KEY_PRESS_SCRIPT + """
performKeyPress(true, true, true, 1)
        """
        subprocess.run(["osascript", "-e", script])

    def handle_special_cases(self):
        script = PERFORM_KEY_PRESS_SCRIPT + """
tell application "EasyDict" to activate
performKeyPress(true, true, false, 1)
        """
        subprocess.run(['osascript', '-e', script])

    def switch_back_to_previous_window(self):
        script = PERFORM_KEY_PRESS_SCRIPT + """
performKeyPress(true, true, true, 15)
        """
        subprocess.run(["osascript", "-e", script])

def run(server_class=http.server.HTTPServer, handler_class=RequestHandler):
    server_address = ('', 8082)
    httpd = server_class(server_address, handler_class)
    print('Starting ```python http server...')
    httpd.serve_forever()

if __name__ == "__main__":
    run()
```

### Launchd Configuration for Startup

Create a Launchd configuration file to run the Python script at startup:

1. **Create a Directory for Launch Agents**:
    ```sh
    mkdir -p ~/Library/LaunchAgents
    ```

2. **Create the Configuration File**:
    ```sh
    nano ~/Library/LaunchAgents/com.user.urlconverter.plist
    ```

3. **Add the Following Content**:
    ```xml
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>Label</key>
        <string>com.user.urlconverter</string>
        <key>ProgramArguments</key>
        <array>
            <string>/usr/local/bin/python3</string>
            <string>/Users/yourusername/url_converter/url_converter.py</string>
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
    Replace `yourusername` with your actual username.

4. **Load and Start the Service**:
    ```sh
    launchctl load ~/Library/LaunchAgents/com.user.urlconverter.plist
    ```

5. **Verify the Service**:
    ```sh
    launchctl list | grep com.user.urlconverter
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

This comprehensive guide should help you set up and run the EasyDictHelper workflow and the URL converter service. If you encounter any issues or need further assistance, please feel free to ask.