use crate::patch_utils::patch_bytes;
use crate::runtime_log::{log_info, log_warn};

const INJECTION_OFFSET: usize = 0x7BD27D;
const THRESHOLD_OFFSET: usize = 0xB49AF0;
const ORIGINAL_BYTES: [u8; 6] = [0xDD, 0x05, 0xF0, 0x9A, 0xF4, 0x00];

pub(crate) unsafe fn apply_brake_light_fix_patch(base: usize, threshold: f64) {
    let target = base + INJECTION_OFFSET;

    // Verify original bytes before patching
    let current = std::slice::from_raw_parts(target as *const u8, 6);
    if current != ORIGINAL_BYTES {
        log_warn(
            "brake_light_fix",
            &format!(
                "Expected bytes not found at {target:#x}, patch skipped. Found: {:02X?}",
                current
            ),
        );
        return;
    }

    // Write new threshold value directly to TestDrive2.exe+B49AF0
    let threshold_addr = base + THRESHOLD_OFFSET;
    patch_bytes(threshold_addr, &threshold.to_le_bytes());

    log_info(
        "brake_light_fix",
        &format!(
            "BrakeLightFix applied: threshold={:.2} at {threshold_addr:#x}",
            threshold
        ),
    );
}
