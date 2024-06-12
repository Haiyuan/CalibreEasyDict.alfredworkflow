use std::net::TcpListener;
use std::io::{Read, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use url::Url;
use webbrowser;

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
            record_current_window();
            let easydict_url = format!("easydict://query?text={}", encoded_word);
            webbrowser::open(&easydict_url).unwrap();
            sleep(Duration::from_secs(1));

            if word.split_whitespace().count() >= 2 {
                handle_special_cases();
                sleep(Duration::from_secs(7));
            }

            switch_back_to_previous_window();

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