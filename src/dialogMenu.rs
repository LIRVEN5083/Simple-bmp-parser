use rfd::FileDialog;

pub fn pick_image_file() -> Option<String> {
    let file = FileDialog::new()
        .add_filter("Image", &["bmp"])
        .set_directory("/")
        .pick_file();

    match file {
        Some(path) => {
            println!("Success! Path:{:?}", path);
            path.to_str().map(|s| s.to_string())
        }
        None => {
            println!("No such file or directory");
            None
        }
    }
}