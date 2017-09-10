use x86intrin::{m256i, mm256_setr_epi8};

#[inline]
pub fn mm256i(i: i8) -> m256i {
    mm256_setr_epi8(
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
        i,
    )
}

#[inline(always)]
pub fn u8_to_m256i(s: &[u8], i: usize) -> m256i {
    mm256_setr_epi8(
        s[i] as i8,
        s[i + 1] as i8,
        s[i + 2] as i8,
        s[i + 3] as i8,
        s[i + 4] as i8,
        s[i + 5] as i8,
        s[i + 6] as i8,
        s[i + 7] as i8,
        s[i + 8] as i8,
        s[i + 9] as i8,
        s[i + 10] as i8,
        s[i + 11] as i8,
        s[i + 12] as i8,
        s[i + 13] as i8,
        s[i + 14] as i8,
        s[i + 15] as i8,
        s[i + 16] as i8,
        s[i + 17] as i8,
        s[i + 18] as i8,
        s[i + 19] as i8,
        s[i + 20] as i8,
        s[i + 21] as i8,
        s[i + 22] as i8,
        s[i + 23] as i8,
        s[i + 24] as i8,
        s[i + 25] as i8,
        s[i + 26] as i8,
        s[i + 27] as i8,
        s[i + 28] as i8,
        s[i + 29] as i8,
        s[i + 30] as i8,
        s[i + 31] as i8,
    )
}

#[inline]
pub fn u8_to_m256i_rest(s: &[u8], i: usize) -> m256i {
    match s.len() - i {
        31 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            s[i + 26] as i8,
            s[i + 27] as i8,
            s[i + 28] as i8,
            s[i + 29] as i8,
            s[i + 30] as i8,
            0,
        ),
        30 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            s[i + 26] as i8,
            s[i + 27] as i8,
            s[i + 28] as i8,
            s[i + 29] as i8,
            0,
            0,
        ),
        29 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            s[i + 26] as i8,
            s[i + 27] as i8,
            s[i + 28] as i8,
            0,
            0,
            0,
        ),
        28 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            s[i + 26] as i8,
            s[i + 27] as i8,
            0,
            0,
            0,
            0,
        ),
        27 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            s[i + 26] as i8,
            0,
            0,
            0,
            0,
            0,
        ),
        26 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            s[i + 25] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        25 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            s[i + 24] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        24 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            s[i + 23] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        23 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            s[i + 22] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        22 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            s[i + 21] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        21 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            s[i + 20] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        20 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            s[i + 19] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        19 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            s[i + 18] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        18 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            s[i + 17] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        17 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            s[i + 16] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        16 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            s[i + 15] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        15 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            s[i + 14] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        14 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            s[i + 13] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        13 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            s[i + 12] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        12 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            s[i + 11] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        11 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            s[i + 10] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        10 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            s[i + 9] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        9 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            s[i + 8] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        8 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            s[i + 7] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        7 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            s[i + 6] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        6 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            s[i + 5] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        5 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            s[i + 4] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        4 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            s[i + 3] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        3 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            s[i + 2] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        2 => mm256_setr_epi8(
            s[i] as i8,
            s[i + 1] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        1 => mm256_setr_epi8(
            s[i] as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        _ => mm256_setr_epi8(
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm256i() {
        let test_cases = vec![0, 1, 2, 3];
        for i in test_cases {
            let want = mm256_setr_epi8(
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
                i,
            );
            let got = mm256i(i);
            assert_eq!(want.as_u8x32().as_array(), got.as_u8x32().as_array());
        }
    }
}
