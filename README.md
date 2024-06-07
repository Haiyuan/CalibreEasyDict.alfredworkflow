# EasyDictHelper.alfredworkflow

EasyDictHelper is an Alfred workflow designed to enhance your productivity by providing quick access to specific applications and tasks. This workflow uses AppleScript to perform actions such as switching applications, getting the mouse cursor position, and activating specific applications.

## Features

- Quickly switch to specific applications (calibre-parallel, sublime_text, sublime_merge) and perform predefined actions.
- Retrieve and restore the mouse cursor position using Python scripts.
- Activate and interact with EasyDict using specific key combinations.

## Installation

1. Download the `EasyDictHelper.alfredworkflow` file from the [Releases](https://github.com/Haiyuan/EasyDictHelper.alfredworkflow/releases) page.
2. Double-click the downloaded file to import it into Alfred.

## Usage

The workflow includes two main AppleScripts, each triggered by different key combinations.

### Script 1: Triggered by `Option + D`

This script switches to a special application, gets the current mouse cursor position, saves it to the clipboard, and then restores the cursor position.

```applescript
-- 直接嵌入 special_apps.json 数据
set specialAppsJson to "{\"special_apps\": [\"calibre-parallel\", \"sublime_text\", \"sublime_merge\"]}"

-- 读取特殊应用列表
set specialApps to specialAppsJson

-- 获取当前应用的名称
tell application "System Events"
    set frontApp to name of first application process whose frontmost is true
end tell

-- 切换回之前的应用
if frontApp is in specialApps then
    -- 如果是特殊应用，使用特定方法切换
    tell application "System Events"
        -- 按下 Cmd + Alt + Ctrl + S
        key down command
        key down option
        key down control
        key code 1 -- S 键的 key code 是 1
        key up control
        key up option
        key up command
    end tell
end if

tell application "System Events"
    -- 按下 Cmd + Alt + Ctrl + D
    key down command
    key down option
    key down control
    key code 2 -- D 键的 key code 是 2
    key up control
    key up option
    key up command
end tell

-- 获取光标的位置，使用虚拟环境中的Python
set mousePosition to do shell script "~/myenv/bin/python -c 'from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, kCGMouseButtonLeft; import Quartz.CoreGraphics as CG; ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (0,0), kCGMouseButtonLeft); print(int(CG.CGEventGetLocation(ourEvent).x))'"

set mousePositionY to do shell script "~/myenv/bin/python -c 'from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, kCGMouseButtonLeft; import Quartz.CoreGraphics as CG; ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (0,0), kCGMouseButtonLeft); print(int(CG.CGEventGetLocation(ourEvent).y))'"

-- 将结果保存到剪贴板
try
    set the clipboard to frontApp & "," & mousePosition & "," & mousePositionY
    -- display dialog "Clipboard Content: " & frontApp & "," & mousePosition & "," & mousePositionY
on error errMsg
    display dialog "Error setting clipboard: " & errMsg
end try

-- 从剪贴板获取信息
set clipboardContent to the clipboard
set AppleScript's text item delimiters to ","
set {frontApp, mousePosition, mousePositionY} to text items of clipboardContent

-- 切换回之前的应用
if frontApp is in specialApps then
    -- 如果是特殊应用，使用特定方法切换
    tell application "System Events"
        -- 按下 Cmd + Alt + Ctrl + R
        key down command
        key down option
        key down control
        key code 15 -- R 键的 key code 是 15
        key up control
        key up option
        key up command
    end tell
else
    -- 切换回之前的应用
    tell application frontApp
        activate
    end tell
end if

try
    -- 恢复光标位置，使用虚拟环境中的Python
    set mouseX to mousePosition as string
    set mouseY to mousePositionY as string

    -- 直接嵌入 Python 脚本
    set pythonScript to "import sys
from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, CGEventPost
import Quartz.CoreGraphics as CG

mousePosition = float(sys.argv[1])
mousePositionY = float(sys.argv[2])

ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (mousePosition, mousePositionY), 0)
CGEventPost(0, ourEvent)"

    -- 确保 Python 脚本正确传递给 shell
    set shellScript to "~/myenv/bin/python -c " & quoted form of pythonScript & " " & mouseX & " " & mouseY
    set result to do shell script shellScript

    -- 输出 shell 脚本的结果，调试用
    -- display dialog "Shell script executed: " & result

    -- 调试信息，弹出对话框确认光标位置恢复
    -- display dialog "Restored to Application: " & frontApp & " | Mouse Position: " & mousePosition & "," & mousePositionY
on error errMsg number errNum
    display dialog "Error: " & errMsg & " (" & errNum & ")"
end try
```

### Script 2: Triggered by `Ctrl + S`

This script switches to a special application, gets the current mouse cursor position, saves it to the clipboard, activates EasyDict, and then restores the cursor position.

```applescript
-- 直接嵌入 special_apps.json 数据
set specialAppsJson to "{\"special_apps\": [\"calibre-parallel\", \"sublime_text\", \"sublime_merge\"]}"

-- 读取特殊应用列表
set specialApps to specialAppsJson

-- 获取当前应用的名称
tell application "System Events"
    set frontApp to name of first application process whose frontmost is true
end tell

-- 切换回之前的应用
if frontApp is in specialApps then
    -- 如果是特殊应用，使用特定方法切换
    tell application "System Events"
        -- 按下 Cmd + Alt + Ctrl + S
        key down command
        key down option
        key down control
        key code 1 -- S 键的 key code 是 1
        key up control
        key up option
        key up command
    end tell
end if

-- 获取光标的位置，使用虚拟环境中的Python
set mousePosition to do shell script "~/myenv/bin/python -c 'from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, kCGMouseButtonLeft; import Quartz.CoreGraphics as CG; ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (0,0), kCGMouseButtonLeft); print(int(CG.CGEventGetLocation(ourEvent).x))'"

set mousePositionY to do shell script "~/myenv/bin/python -c 'from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, kCGMouseButtonLeft; import Quartz.CoreGraphics as CG; ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (0,0), kCGMouseButtonLeft); print(int(CG.CGEventGetLocation(ourEvent).y))'"

-- 将结果保存到剪贴板
try
    set the clipboard to frontApp & "," & mousePosition & "," & mousePositionY
    -- display dialog "Clipboard Content: " & frontApp & "," & mousePosition & "," & mousePositionY
on error errMsg
    display dialog "Error setting clipboard: " & errMsg
end try

tell application "EasyDict"
    activate
end tell

tell application "System Events"
    key code 101 -- F9 键的 key code 是 101
end tell

delay 0.5 -- 等待应用激活

tell application "System Events"
    -- 按下 Cmd + Alt + S
    key down command
    key down option
    key code 1 -- S 键的 key code 是 1
    key up option
    key up command
end tell

delay 0.5 -- 等待应用激活

-- 从剪贴板获取信息
set clipboardContent to the clipboard
set AppleScript's text item delimiters to ","
set {frontApp, mousePosition, mousePositionY} to text items of clipboardContent

-- 切换回之前的应用
if frontApp is in specialApps then
    -- 如果是特殊应用，使用特定方法切换
    tell application "System Events"
        -- 按下 Cmd + Alt + Ctrl + R
        key down command
        key down option
        key down control
        key code 15 -- R 键的 key code 是 15
        key up control
        key up option
        key up command
    end tell
else
    -- 切换回之前的应用
    tell application frontApp
        activate
    end tell
end if

try
    -- 恢复光标位置，使用虚拟环境中的Python
    set mouseX to mousePosition as string
    set mouseY to mousePositionY as string

    -- 直接嵌入 Python 脚本
    set pythonScript to "import sys
from Quartz.CoreGraphics import CGEventCreateMouseEvent, kCGEventMouseMoved, CGEventPost
import Quartz.CoreGraphics as CG

mousePosition = float(sys.argv[1])
mousePositionY = float(sys.argv[2])

ourEvent = CG.CGEventCreateMouseEvent(None, kCGEventMouseMoved, (mousePosition, mousePositionY), 0)
CGEventPost(0, ourEvent)"

    -- 确保 Python 脚本正确传递给 shell
    set shellScript to "~/myenv/bin/python -c " & quoted form of pythonScript & " " & mouseX & " " & mouseY
    set result to do shell script shellScript

    -- 输出 shell 脚本的结果，调试用
    -- display dialog "Shell script executed: " & result

    -- 调试信息，弹出对话框确认光标位置恢复
    -- display dialog "Restored to Application: " & frontApp & " | Mouse Position: " & mousePosition & "," & mousePositionY
on error errMsg number errNum
    display dialog "Error: " & errMsg & " (" & errNum & ")"
end try
```

## Requirements

- [Alfred](https://www.alfredapp.com/) with the Powerpack
- Python (installed in a virtual environment at `~/myenv/bin/python`)
- macOS with accessibility permissions enabled for Alfred

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Alfred](https://www.alfredapp.com/) for providing an amazing productivity tool.
- [AppleScript](https://developer.apple.com/applescript/) and [Python](https://www.python.org/) for making automation possible.

---

Feel free to customize this README as needed. Happy automating with EasyDictHelper!