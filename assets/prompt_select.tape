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
Output assets/prompt_select.gif

Type@40ms "cargo run --example prompt_select" Enter
Sleep 3s

Down Type "j" Sleep 1s
Up Sleep 1s Enter

Sleep 5s
