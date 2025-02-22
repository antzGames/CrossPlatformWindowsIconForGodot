# Cross Platform Windows Icon Generator for Godot

This simple (work in progress) addon allows you to set the icon for your Window EXE file without having to use Wine, or build on a Windows computer with `rcedit.exe`.

It uses `editpe` (https://github.com/Systemcluster/editpe), a Rust project that does the same thing that `rcedit.exe` does on Windows. But because it uses Rust, I made a Rust based GDExtention to make a simple Node that can now do the same thing `rcedit` does, but now is able to run on other platforms like Linux.

# This is only a technology sampler project

My solution is not ideal.  I created it only to prove that everything works cross platform.  

I will try to contact a contributor and see if they can build this solution into the Windows export target or directly into Godot.

![image](https://github.com/user-attachments/assets/93fdf31f-053e-4113-94c0-e04d2b463024)

# Limitations

- Only the Windows and Linux versions of the GDExtention (the addon) works.
- You need to specify the FULLY QUALIFIED path to the icon image file and the Windows EXE file that you want to apply a plugin.
- Its built as a Node, so you need to use the `WindowIconGenerator` Node in a test scene to generate it manually.  A test scene is included in the `addons/WindowsIcon` directory.
- there is no error checking or exception handling so your paths need to be perfect.

# How to use

1. Open the `windows_icon_test.tscn` scene.
2. Update the `Icon Path` and `Exe Path` values to the FULLY QUALIFIED path to the icon image file and the Windows EXE file that you want to apply a plugin.
3. This test scene just calls `make_icon()` method `on_ready()` and then exits (automatically).
4. Run THIS scene (make sure of this or else nothing happens).

# How to build Rust GDExtention

I use CROSS (https://github.com/cross-rs/cross) to compile and build the Windows and Linux versions.  Installing and configuring CROSS is too big to detail out in this README.
