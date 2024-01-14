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
Set Height 900
Output assets/quick_start.gif

Type@40ms "cargo run --example quick_start" Sleep 300ms Enter
Sleep 4s

Type "empty test" Sleep 1s Backspace 10 Sleep 500ms Enter Sleep 3s
Type "wadackel" Sleep 2s Enter
Sleep 2s Right Sleep 2s Left Sleep 2s Enter
Sleep 2s Down 11 Sleep 2s Up 11 Sleep 2s Enter

Sleep 5s
