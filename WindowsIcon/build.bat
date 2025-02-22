cls
cross build --release --target x86_64-pc-windows-gnu
copy target\x86_64-pc-windows-gnu\release\WindowsIcon.dll ..\godot\addons\WindowsIcon\
cross build --release --target x86_64-unknown-linux-gnu
copy target\x86_64-unknown-linux-gnu\release\libWindowsIcon.so ..\godot\addons\WindowsIcon\

