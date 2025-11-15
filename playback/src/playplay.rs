unsafe extern "C" {
    pub fn playplay_get_version() -> i32;

    pub fn playplay_get_token(output_buffer: *mut u8, // 16 bytes
    );

    pub fn playplay_decrypt(
        key: *const u8,         // 16 bytes
        file_id: *const u8,     // 16 bytes
        output_buffer: *mut u8, // 16 bytes
    );
}

pub fn get_version() -> i32 {
    unsafe { playplay_get_version() }
}

pub fn get_token() -> [u8; 16] {
    let mut output = [0u8; 16];
    unsafe {
        playplay_get_token(
            output.as_mut_ptr(), // *mut u8
        );
    }
    output
}

pub fn decrypt(encrypted_key: [u8; 16], file_id: [u8; 16]) -> [u8; 16] {
    let mut output = [0u8; 16];
    unsafe {
        playplay_decrypt(
            encrypted_key.as_ptr(), // *const u8
            file_id.as_ptr(),       // *const u8
            output.as_mut_ptr(),    // *mut u8
        );
    }
    output
}
