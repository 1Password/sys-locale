fn main() {
    windows_bindgen::bindgen(&["--etc", "./bindings.config"]).unwrap();
}
