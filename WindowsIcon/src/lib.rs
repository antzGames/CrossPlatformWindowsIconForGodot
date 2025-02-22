use godot::prelude::*;
use godot::classes::Node;
use editpe::*;
struct WindowsIcon;

#[gdextension]
unsafe impl ExtensionLibrary for WindowsIcon {}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct WindowsIconGenerator {
    is_done: bool,
    timer: f64,

    #[init(val = GString::from("D:\\Downloads\\antz_256.ico"))]
    #[export]
    icon_path: GString,

    #[init(val = GString::from("D:\\Downloads\\test.exe"))]
    #[export]
    exe_path: GString,
    #[base]
    base: Base<Node>
}


#[godot_api]
impl WindowsIconGenerator {
    #[func]
    fn make_icon(&mut self) {
        self.is_done = false;
        self.timer = 0.0;
	    let exe_path_string = String::from(&self.exe_path);
	    let icon_path_string = String::from(&self.icon_path);

        let exe_str = exe_path_string.as_str();
        let icon_str = icon_path_string.as_str();

	    godot_print!("WindowsIconGenerator started!");

        godot_print!("   Parsing Windows EXE file.");
        let mut image = Image::parse_file(&exe_str).expect("Could not parse exe file");
	    let mut resources = image.resource_directory().cloned().unwrap_or_default();

        godot_print!("   Applying icon file.");
        resources.remove_main_icon().expect("Failed to set icon file");
        resources.set_main_icon_file(&icon_str).expect("Failed to set icon file");
        image.set_resource_directory(resources).expect("Failed to set resources");

        godot_print!("   Saving Windows EXE file.");
        image.write_file(&exe_str).expect("Failed to write exe file");

        godot_print!("Done!");
        self.is_done = true;
    }
}

#[godot_api]
impl INode for WindowsIconGenerator {

    fn process(&mut self, delta: f64) {
        if self.is_done {
            self.timer += delta;
            if self.timer > 1.0 {
                self.base().get_tree().unwrap().quit();
            }
        }
    }
}



