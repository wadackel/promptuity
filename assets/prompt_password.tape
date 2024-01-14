Require cargo
Set Shell "zsh"
Set Theme "Dracula+"
Set WindowBar Colorful
Set WindowBarSize 60
Set Margin 24
Set MarginFill "#f0f4f7"
Set Padding 36
Set BorderRadius 16
Set CursorBlink false
Set FontSize 30
Set TypingSpeed 120ms

Set Width 1500
Set Height 500
Output assets/prompt_password.gif

Type@40ms "cargo run --example prompt_password" Enter
Sleep 3s

Type "123" Sleep 1s Enter
Sleep 3s Ctrl+U Sleep 1s Enter
Sleep 3s Type "secret123" Sleep 2s Enter

Sleep 5s
