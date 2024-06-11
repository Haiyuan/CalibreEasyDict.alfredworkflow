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
    let output = Command::new("python3")
        .arg("-c")
        .arg(r#"
import Quartz.CoreGraphics as CG
loc = CG.CGEventGetLocation(CG.CGEventCreate(None))
print(int(loc.x), int(loc.y))
"#)
        .output()
        .expect("Failed to execute Python script");

    if !output.status.success() {
        panic!("Failed to get mouse position: {:?}", output.stderr);
    }

    let result = String::from_utf8_lossy(&output.stdout);
    println!("Mouse position raw output: {}", result); // Print raw output for debugging

    let coords: Vec<&str> = result.trim().split_whitespace().collect();

    if coords.len() != 2 {
        panic!("Failed to get mouse position: {}", result);
    }

    let x: i32 = coords[0].trim().parse().expect("Failed to parse X coordinate");
    let y: i32 = coords[1].trim().parse().expect("Failed to parse Y coordinate");

    (x, y)
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
            set shellScript to "/usr/bin/python3 -c " & quoted form of pythonScript & " " & mouseX & " " & mouseY
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