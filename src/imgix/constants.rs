pub const SRCSET_INCREMENT_PERCENTAGE: f32 = 8.0;
pub const SRCSET_MAX_SIZE: f32 = 8192.0;

pub const SRCSET_TARGET_WIDTHS: [u32; 31] = [
    100, 116, 134, 156, 182, 210, 244, 282, 328, 380, 442, 512, 594, 688, 798, 926, 1074, 1246,
    1446, 1678, 1946, 2258, 2618, 3038, 3524, 4088, 4742, 5500, 6380, 7400, 8192,
];

pub fn lib_version() -> String {
    return format!("rust={}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_srcset_target_widths() {
        assert_eq!(SRCSET_TARGET_WIDTHS.len(), 31);
        let mut prev = 100f32;

        let ensure_even = |value: f32| -> u32 {
            let half = value / 2.0;
            return (2.0 * half.round()) as u32;
        };

        let mut index = 0usize;
        while prev <= SRCSET_MAX_SIZE && index < SRCSET_TARGET_WIDTHS.len() {
            assert_eq!(SRCSET_TARGET_WIDTHS[index], ensure_even(prev));
            index += 1;
            prev *= 1.0 + (SRCSET_INCREMENT_PERCENTAGE / 100.0) * 2.0;
        }
    }
}
