use alloc::string::String;

pub(crate) fn get() -> Option<String> {
    web_sys::window()?.navigator().language()
}
