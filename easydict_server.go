package main

import (
	"fmt"
	"net/http"
	"net/url"
	"os/exec"
	"strings"
	"time"
)

func recordCurrentWindow() {
	script := `
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
`
	exec.Command("osascript", "-e", script).Run()
}

func handleSpecialCases() {
	script := `
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
`
	exec.Command("osascript", "-e", script).Run()
}

func switchBackToPreviousWindow() {
	script := `
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
`
	exec.Command("osascript", "-e", script).Run()
}

func handler(w http.ResponseWriter, r *http.Request) {
	parsedURL, _ := url.Parse(r.URL.String())
	query := parsedURL.Query()
	word := query.Get("text")

	if word == "" {
		http.Error(w, "Missing query text.", http.StatusBadRequest)
		return
	}

	encodedWord := url.QueryEscape(word)
	recordCurrentWindow()
	easydictURL := fmt.Sprintf("easydict://query?text=%s", encodedWord)
	exec.Command("open", easydictURL).Start()
	time.Sleep(1 * time.Second)

	if len(strings.Fields(word)) >= 2 {
		handleSpecialCases()
		time.Sleep(7 * time.Second)
	}

	switchBackToPreviousWindow()

	w.Header().Set("Content-Type", "text/html")
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("URL has been converted and opened."))
}

func main() {
	http.HandleFunc("/", handler)
	fmt.Println("Starting http server on :8080")
	http.ListenAndServe(":8080", nil)
}