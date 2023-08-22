use windows::Win32::System::Threading::PROCESS_CREATION_FLAGS;

pub struct ModSettings {
    pub enabled: bool,
    pub priority: Option<PROCESS_CREATION_FLAGS>,
}
