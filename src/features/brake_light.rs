use windows_sys::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE,
};

static THRESHOLD_PTR: std::sync::atomic::AtomicUsize =
std::sync::atomic::AtomicUsize::new(0);

pub(crate) unsafe fn apply_brake_light_fix_patch(base: usize, threshold: f64) {
    let target = base + INJECTION_OFFSET;

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

    // Allocate memory in the process for our threshold value
    let alloc = VirtualAlloc(
        std::ptr::null(),
                             8,
                             MEM_COMMIT | MEM_RESERVE,
                             PAGE_READWRITE,
    );

    if alloc.is_null() {
        log_warn("brake_light_fix", "VirtualAlloc failed, patch skipped");
        return;
    }

    // Write threshold value to allocated memory
    std::ptr::write(alloc as *mut f64, threshold);
    THRESHOLD_PTR.store(alloc as usize, std::sync::atomic::Ordering::Relaxed);

    // Build: DD 05 [alloc addr as 4 LE bytes]
    let alloc_addr = alloc as u32;
    let mut patch = [0u8; 6];
    patch[0] = 0xDD;
    patch[1] = 0x05;
    patch[2..6].copy_from_slice(&alloc_addr.to_le_bytes());

    patch_bytes(target, &patch);

    log_info(
        "brake_light_fix",
        &format!(
            "BrakeLightFix applied: threshold={:.2}, alloc={alloc:#x}, patch at {target:#x}",
            threshold
        ),
    );
}
