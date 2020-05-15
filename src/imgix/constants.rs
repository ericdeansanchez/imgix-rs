/// Representation of an image with a width of zero. This value is used
/// in validation contexts, i.e. "is the width of the passed or requested
/// image greater than or equal to the 'zero width image'."
pub const IMAGE_ZERO_WIDTH: f32 = 0.0;

/// The minimum width of a default generated image-width.
pub const IMAGE_MIN_WIDTH: f32 = 100.0;

/// The maximum width of a default generated image-width.
pub const IMAGE_MAX_WIDTH: f32 = 8192.0;

/// The srcset width tolerance dictates the _maximum tolerated size_
/// difference between an image's downloaded size and its rendered size.
/// For example, setting this value to 10.0 means that an image will not
/// render more than 10% larger or smaller than its native size.
pub const SRCSET_WIDTH_TOLERANCE: f32 = 8.0;

pub const SRCSET_TARGET_WIDTHS: [u32; 31] = [
    100, 116, 135, 156, 181, 210, 244, 283, 328, 380, 441, 512, 594, 689, 799, 927, 1075, 1247,
    1446, 1678, 1946, 2257, 2619, 3038, 3524, 4087, 4741, 5500, 6380, 7401, 8192,
];

/// The default density pixel ratios (dpr).
pub const SRCSET_TARGET_DPR_RATIOS: [u32; 5] = [1, 2, 3, 4, 5];

pub const SRCSET_DPR_QUALITIES: [u32; 5] = [75, 50, 35, 23, 20];

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
        let mut index = 0usize;
        while prev <= IMAGE_MAX_WIDTH && index < SRCSET_TARGET_WIDTHS.len() {
            assert_eq!(SRCSET_TARGET_WIDTHS[index], prev.round() as u32);
            index += 1;
            prev *= 1.0 + (SRCSET_WIDTH_TOLERANCE / 100.0) * 2.0;
        }

        // Check that we constructed the correct number of widths;
        // this accounts for zero-based indexing; at the end of the
        // loop the index is 30 (i.e. length([0 - 30]) == 31) so
        // we subtract one from the length and check for equality.
        assert_eq!(index, SRCSET_TARGET_WIDTHS.len() - 1);
    }
}
