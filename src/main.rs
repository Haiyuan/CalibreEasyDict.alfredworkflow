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
            println!("Mouse position raw output: {}", result); // 打印原始输出以便调试
            println!("Mouse position error output: {}", error); // 打印错误输出以便调试

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