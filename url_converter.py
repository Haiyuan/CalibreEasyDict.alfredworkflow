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

        if not word:
            self.send_response(400)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b'Missing query text.')
            return

        encoded_word = urllib.parse.quote(word)
        mouse_position = self.get_mouse_position()
        self.record_current_window()
        easydict_url = f"easydict://query?text={encoded_word}"
        webbrowser.open(easydict_url)
        time.sleep(1)

        if len(word.split()) >= 2:
            self.handle_special_cases()
            time.sleep(7)

        self.switch_back_to_previous_window()
        self.restore_mouse_position(mouse_position)

        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'URL has been converted and opened.')

    def get_mouse_position(self):
        script = """
        on getMousePosition()
            set mousePositionScript to "~/myenv/bin/python -c 'import Quartz.CoreGraphics as CG; loc = CG.CGEventGetLocation(CG.CGEventCreate(None)); print(int(loc.x), int(loc.y))'"
            set mousePosition to do shell script mousePositionScript
            set AppleScript's text item delimiters to " "
            set {mousePositionX, mousePositionY} to text items of mousePosition
            return {mousePositionX, mousePositionY}
        end getMousePosition

        getMousePosition()
        """
        result = subprocess.check_output(['osascript', '-e', script])
        return result.decode('utf-8').strip().split(' ')

    def record_current_window(self):
        script = """
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
        """
        subprocess.run(["osascript", "-e", script])

    def handle_special_cases(self):
        script = """
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
        """
        subprocess.run(['osascript', '-e', script])

    def switch_back_to_previous_window(self):
        script = """
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
        """
        subprocess.run(["osascript", "-e", script])

    def restore_mouse_position(self, position):
        script = f"""
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

        restoreMousePosition({position[0]}, {position[1]})
        """
        subprocess.run(['osascript', '-e', script])

def run(server_class=http.server.HTTPServer, handler_class=RequestHandler):
    server_address = ('', 8080)
    httpd = server_class(server_address, handler_class)
    print('Starting http server...')
    httpd.serve_forever()

if __name__ == "__main__":
    run()