# Setup Guide for EasyDict Server Project

## Overview

This guide will help you set up, compile, and configure the `easydict_server` Go application to run as a startup item on macOS.

## Prerequisites

- Go programming language installed
- macOS operating system

## Steps

### 1. Clone the Repository

First, clone the repository containing the Go project to your local machine:

```bash
git clone https://github.com/Haiyuan/EasyDictHelper.alfredworkflow.git
cd EasyDictHelper.alfredworkflow
```

### 2. Compile the Go Program

Navigate to the project directory and compile the Go program:

```bash
go build -o easydict_server
```

### 3. Create a Launch Agent

To run the program as a startup item on macOS, create a Launch Agent file. Use the following content to create a file named `com.user.easydict_server.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.easydict_server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/easydict_server</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>/path/to/easydict_server.err</string>
    <key>StandardOutPath</key>
    <string>/path/to/easydict_server.out</string>
</dict>
</plist>
```

Replace `/path/to/easydict_server` with the actual path to your compiled `easydict_server` executable.

### 4. Place the Launch Agent File

Move the `com.user.easydict_server.plist` file to the `~/Library/LaunchAgents` directory:

```bash
cp com.user.easydict_server.plist ~/Library/LaunchAgents/
```

### 5. Load the Launch Agent

Load the Launch Agent to make it start at login:

```bash
launchctl load ~/Library/LaunchAgents/com.user.easydict_server.plist
```

### 6. Verify the Launch Agent

Ensure that the Launch Agent is loaded and running:

```bash
launchctl list | grep com.user.easydict_server
```

### 7. Unload the Launch Agent (Optional)

If you need to stop or unload the Launch Agent, use the following command:

```bash
launchctl unload ~/Library/LaunchAgents/com.user.easydict_server.plist
```

## Conclusion

Following these steps will set up and run your Go program as a startup item on macOS. If you encounter any issues, please check the log files specified in the Launch Agent (`easydict_server.err` and `easydict_server.out`) for troubleshooting.

## Troubleshooting

- Ensure that the path to your `easydict_server` executable is correct in the `.plist` file.
- Check for any errors in the log files (`easydict_server.err` and `easydict_server.out`).
- Make sure the `~/Library/LaunchAgents` directory exists and is accessible.

For further assistance, you may refer to the macOS Launch Services documentation or contact the project maintainers.