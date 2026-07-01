use crate::patch_utils::patch_bytes;
use crate::runtime_log::{log_info, log_warn};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE,
};

const INJECTION_OFFSET: usize = 0x7BD27D;
const ORIGINAL_BYTES: [u8; 6] = [0xDD, 0x05, 0xF0, 0x9A, 0xF4, 0x00];

static mut THRESHOLD_STORAGE: f64 = 0.0;

pub(crate) unsafe fn apply_brake_light_fix_patch(base: usize, threshold: f64) {
    let target = base + INJECTION_OFFSET;

    // Verify original bytes
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

    // Write threshold to static storage
    THRESHOLD_STORAGE = threshold;
    let storage_addr = std::ptr::addr_of!(THRESHOLD_STORAGE) as u32;

    // Build new instruction: DD 05 [storage_addr as 4 LE bytes]
    // fld qword ptr [storage_addr]
    let mut patch = [0u8; 6];
    patch[0] = 0xDD;
    patch[1] = 0x05;
    patch[2..6].copy_from_slice(&storage_addr.to_le_bytes());

    patch_bytes(target, &patch);

    log_info(
        "brake_light_fix",
        &format!(
            "BrakeLightFix applied: threshold={:.2}, fld operand redirected to {storage_addr:#x}",
            threshold
        ),
    );
}
