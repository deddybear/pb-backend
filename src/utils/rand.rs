use std::time::{SystemTime, UNIX_EPOCH};

pub fn random_string(
    length: usize,
    with_uppercase: bool,
    with_lowercase: bool,
    with_numeric: bool,
) -> Result<String, &'static str> {
    if !with_uppercase && !with_lowercase && !with_numeric {
        return Err("Minimal satu karakter set harus diaktifkan");
    }

    // Bangun charset berdasarkan parameter
    let mut charset = Vec::new();

    if with_uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if with_lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    if with_numeric {
        charset.extend_from_slice(b"0123456789");
    }

    let charset_len = charset.len();
    let mut result = String::with_capacity(length);

    // Simple LCG (Linear Congruential Generator)
    // Seed dari waktu sistem
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    for _ in 0..length {
        // Parameter LCG yang umum digunakan (sama seperti glibc)
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let index = ((seed >> 33) as usize) % charset_len;
        result.push(charset[index] as char);
    }

    return Ok(result);
}
