// TODO: file logger

#[cfg(debug_assertions)]
pub fn report_debug(text: &str) {
    match msgbox::create("Cpu Optimizer Debug", text, msgbox::IconType::Info) {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn report_error(text: &str) {
    match msgbox::create("Cpu Optimizer Error", text, msgbox::IconType::Error) {
        Ok(_) => {}
        Err(_) => {}
    }
}