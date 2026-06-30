use std::ptr;

// Offset of the injection point within TestDrive2.exe
const THRESHOLD_OFFSET: usize = 0x7BD27D;
const ORIGINAL_BYTES: [u8; 6] = [0xDD, 0x05, 0xF0, 0x9A, 0xF4, 0x00];

pub struct BrakeLightPatch {
    // New threshold value to write
    // 0.0 = lights activate immediately (new)
    // 0.10 = default dead zone (original)
    pub threshold: f64,
}

impl BrakeLightPatch {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub unsafe fn apply(&self, base_addr: usize) {
        let target = base_addr + THRESHOLD_OFFSET;

        // Verify original bytes before patching
        let current = std::slice::from_raw_parts(target as *const u8, 6);
        if current != ORIGINAL_BYTES {
            log::warn!("BrakeLightPatch: Expected bytes not found, patch skipped.");
            return;
        }

        // Make memory writable
        let mut old_protect = 0u32;
        winapi::um::memoryapi::VirtualProtect(
            target as *mut _,
            6,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        );

        // Write new threshold value directly to the address
        // that held the original 0.10 double
        let threshold_addr = (base_addr + 0xB49AF0) as *mut f64;
        ptr::write(threshold_addr, self.threshold);

        // Restore memory protection
        winapi::um::memoryapi::VirtualProtect(
            target as *mut _,
            6,
            old_protect,
            &mut old_protect,
        );

        log::info!(
            "BrakeLightPatch: Threshold set to {:.2}.",
            self.threshold
        );
    }

    pub unsafe fn revert(&self, base_addr: usize) {
        // Restore original threshold value
        let threshold_addr = (base_addr + 0xB49AF0) as *mut f64;
        ptr::write(threshold_addr, 0.10);
        log::info!("BrakeLightPatch: Reverted to default value.");
    }
}
