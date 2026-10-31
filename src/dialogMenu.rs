use rfd::FileDialog;

pub fn pick_image_file() {
    let file = FileDialog::new()
        .add_filter("Image", &["bmp"])
        .set_directory("/")
        .pick_file();

    match file {
        Some(path) => println!("Success! Path:{:?}", path),
        None => println!("No such file or directory"),
    }
}