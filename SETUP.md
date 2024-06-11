# URL Converter Service Setup Guide

This project sets up a local HTTP server that listens for requests of the form `http://localhost:8080/?text={word}` and converts them into `easydict://query?text={word}`, which is then opened by the default web browser. The service is configured to run at startup on MacOS.

## Prerequisites

1. **Homebrew**: If you don't have Homebrew installed, you can install it with the following command:
   ```sh
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. **Python**: Install Python via Homebrew:
   ```sh
   brew install python
   ```

## Setting Up the Service

### Step 1: Create and Configure the Python Script

1. **Create Directory and Script**:
   Create a directory for the script if it doesn't exist and move the script there.
   ```sh
   mkdir -p ~/url_converter
   ```

2. **Create the Script**:
   Create and edit `url_converter.py` file in the `~/url_converter` directory with the following content:

   ```python
   import http.server
   import urllib.parse
   import webbrowser
   import subprocess
   import time

   class RequestHandler(http.server.BaseHTTPRequestHandler):
       def do_GET(self):
           parsed_path = urllib.parse.urlparse(self.path)
           query = urllib.parse.parse_qs(parsed_path.query)
           word = query.get('text', [''])[0]

           # Check if the query text is present
           if not word:
               self.send_response(400)
               self.send_header('Content-type', 'text/html')
               self.end_headers()
               self.wfile.write(b'Missing query text.')
               return

           # Encode the word
           encoded_word = urllib.parse.quote(word)

           # Get current mouse position
           mouse_position_script = """
           on getMousePosition()
               set mousePositionScript to "~/myenv/bin/python -c 'import Quartz.CoreGraphics as CG; loc = CG.CGEventGetLocation(CG.CGEventCreate(None)); print(int(loc.x), int(loc.y))'"
               set mousePosition to do shell script mousePositionScript
               set AppleScript's text item delimiters to " "
               set {mousePositionX, mousePositionY} to text items of mousePosition
               return {mousePositionX, mousePositionY}
           end getMousePosition

           getMousePosition()
           """
           mouse_position = subprocess.check_output(['osascript', '-e', mouse_position_script])
           mouse_position = mouse_position.decode('utf-8').strip().split(' ')
           mouse_x, mouse_y = mouse_position[0], mouse_position[1]

           # AppleScript to simulate Cmd + Alt + Ctrl + S to record the current window
           record_window_script = """
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

           on switchToApp(appName, keyCode)
               performKeyPress(true, true, true, keyCode)
           end switchToApp

           switchToApp("frontApp", 1)
           """
           subprocess.run(["osascript", "-e", record_window_script])

           # Open easydict URL
           easydict_url = f"easydict://query?text={encoded_word}"
           webbrowser.open(easydict_url)

           # Wait for the dictionary window to open
           time.sleep(1)

           # AppleScript to simulate Cmd + Alt + Ctrl + R to switch back to the previous window
           switch_window_script = """
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

           on switchToApp(appName, keyCode)
               performKeyPress(true, true, true, keyCode)
           end switchToApp

           switchToApp("frontApp", 15)
           """
           subprocess.run(["osascript", "-e", switch_window_script])

           # Restore mouse position
           restore_mouse_position_script = f"""
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

           restoreMousePosition({mouse_x}, {mouse_y})
           """
           subprocess.run(['osascript', '-e', restore_mouse_position_script])

           # Send response
           self.send_response(200)
           self.send_header('Content-type', 'text/html')
           self.end_headers()
           self.wfile.write(b'URL has been converted and opened.')

   def run(server_class=http.server.HTTPServer, handler_class=RequestHandler):
       server_address = ('', 8080)
       httpd = server_class(server_address, handler_class)
       print('Starting http server...')
       httpd.serve_forever()

   if __name__ == "__main__":
       run()
   ```

### Step 2: Configure Launchd to Run the Script at Startup

1. **Create Launchd Configuration File**:
   Create a directory for launch agents if it doesn't exist and create the configuration file.
   ```sh
   mkdir -p ~/Library/LaunchAgents
   nano ~/Library/LaunchAgents/com.user.urlconverter.plist
   ```

2. **Add the Following Content**:
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

3. **Load and Start the Service**:
   ```sh
   launchctl load ~/Library/LaunchAgents/com.user.urlconverter.plist
   ```

4. **Verify the Service**:
   ```sh
   launchctl list | grep com.user.urlconverter
   ```

## Configuring Calibre to Use the Local Service

1. **Open Calibre**:
   - Launch the `Calibre` application.

2. **Access Preferences**:
   - Click on the "Preferences" button in the toolbar, or press `Ctrl+P` to open the Preferences window.

3. **Configure Lookup Plugin**:
   - In the Preferences window, find and click on "Lookup" under the "Advanced" section.
   - Under the "Lookup sources" tab, click on "Add a custom source".

4. **Add Custom Lookup Source**:
   - In the new window, fill in the following information:
     - **Name**: Enter a descriptive name, such as "Local Dictionary Service".
     - **Lookup URL**: Enter `http://localhost:8080/?text={word}`.

5. **Save Settings**:
   - Click "OK" to save the custom lookup source, then close the Preferences window.

## Usage

1. **Run the Python Script**:
   - Ensure the Python script is running by executing the following command in the terminal:
     ```sh
     python3 ~/url_converter/url_converter.py
     ```

2. **Use Calibre Lookup**:
   - In `Calibre`, when you select a word and use the lookup feature, the configured custom source will send a request to `http://localhost:8080/?text={word}`.
   - The local service will convert this request to `easydict://query?text={word}` and open it in the default web browser.

## Troubleshooting

- **Logs**: Check the logs at `/tmp/urlconverter.log` and `/tmp/urlconverter.err` for any issues.
- **Service Status**: Ensure the service is running by verifying its status with `launchctl list | grep com.user.urlconverter`.
- **Script Path**: Verify that the script path in the `plist` file is correct.

## Checking Python Accessibility Permissions

Ensure that Python has the necessary accessibility permissions. Here are the steps:

1. **Open System Preferences**:
   - Click the Apple menu and select "System Preferences."

2. **Access Security & Privacy Settings**:
   - Click on "Security & Privacy," then select the "Privacy" tab.

3. **Add Python to the Accessibility List**:
   - In the left sidebar, select "Accessibility."
   - If the lock icon in the lower-left corner shows as locked, click it and enter your administrator password to unlock.
   - Click the "+" button and navigate to the Python installation path. Select `/usr/local/bin/python3` (if installed via Homebrew).
   - Ensure Python is selected and enabled.

## Contributing

Feel free to submit issues or pull requests for improvements and bug fixes.

## License

This project is licensed under the MIT License.

---

This guide should help you set up and run the URL converter service. If you encounter any issues or need further assistance, please feel free to ask.