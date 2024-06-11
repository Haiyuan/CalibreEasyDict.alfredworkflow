on getMousePosition()
    set mousePositionScript to "~/myenv/bin/python ~/url_converter/get_mouse_position.py"
    set mousePosition to do shell script mousePositionScript
    set AppleScript's text item delimiters to " "
    set {mousePositionX, mousePositionY} to text items of mousePosition
    return {mousePositionX, mousePositionY}
end getMousePosition

getMousePosition()