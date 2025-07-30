//! Extended Grapheme Cluster Utils
//!
//! Handy Grapheme Helper Utils
//!
//! ```rust
//! use grapheme_utils::*;
//! 
//! fn main() {
//!     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
//! 
//!     println!("num_graphemes {}", num_graphemes(&st)); // Prints 12, the string has 12 grapheme clusters total
//! 
//!     println!("string_width {}", string_width(&st)); // Prints 18, the string uses 18 columns
//! 
//!     println!("nth_grapheme_idx {}", nth_grapheme_idx(&st, 1)); // Prints 6 (index 6)
//!     println!("nth_grapheme_idx {}", nth_grapheme_idx(&st, 2)); // Prints 18 (index 18)
//! 
//!     println!("nth_grapheme {}", nth_grapheme(&st, 1)); // Prints न्दी, the 2nd byte in the string, base 0
//!     println!("nth_grapheme {}", nth_grapheme(&st, 2)); // Prints H, the 3rd bytes in the string, base 0
//! 
//!     println!("nth grapheme_width {}", nth_grapheme_width(&st, 1)); // Prints 3
//!     println!("nth grapheme_width {}", nth_grapheme_width(&st, 2)); // Prints 1
//! 
//!     // Grapheme Visual Column Width
//!     println!("grapheme_width_at_idx {}", grapheme_width_at_idx(&st, 18)); // Prints 1 (H is 1 column wide)
//!     let num = 7; // Anything between 6 and 17 inclusive
//!     println!(
//!         "grapheme_width_at_idx {}",
//!         grapheme_width_at_idx(&st, num)
//!     ); // Prints 3 (न्दी is 3 columns wide)
//! 
//!     // Grapheme utf8 byte count
//!     println!("grapheme_len {}", grapheme_len(&st, num)); // Prints 12 (न्दी uses 12 utf8 bytes)
//!     println!("grapheme_len {}", grapheme_len(&st, 18)); // Prints 1 (H uses 1 utf8 byte)
//! 
//!     // Matrix of grapheme functions:
//!     // [previous, current or next] grapheme given an index
//!     //
//!     // Outputing either the
//!     //  [extended grapheme cluster string, or starting string index]
//!     //
//!     // Take special note of the 2 characters before and after the H.
//!     // The characer before न्दी starts at string index 6, H is at 18, and 🧑 is at index 19
//!     //
//! 
//!     // Output string index
//!     println!(
//!         "prev_grapheme_idx_from_idx {}",
//!         prev_grapheme_idx_from_idx(&st, 18)
//!     ); // Prints 6
//! 
//!     println!("grapheme_idx_at_idx {}", grapheme_idx_at_idx(&st, 18)); // Prints 18
//! 
//!     println!(
//!         "next_grapheme_idx_from_idx {}",
//!         next_grapheme_idx_from_idx(&st, 18)
//!     ); // Prints 19
//! 
//!     // Output extended grapheme cluster
//!     println!("prev_grapheme_from_idx {}", prev_grapheme_from_idx(&st, 18)); // Prints न्दी
//! 
//!     println!("grapheme_at_idx {}", grapheme_at_idx(&st, 18)); // Prints H
//! 
//!     println!("next_grapheme_from_idx {}", next_grapheme_from_idx(&st, 18)); // Prints 🧑
//! 
//!     // Note, all of the above matrix of functions work with num, the range of inputs
//!     //       instead of requiring the exact start to each grapheme.
//!     // Examining the testing matrix may be instructive
//! }
//! ```


// Note: Ease of use over Ideomatic Rust
//       This code will return a '' grapheme or an index past the end of the
//       string instead of None, etc.
//
//       I find it much more convenient when working with text to concatenate ''
//       stings more convenient than checking an handling None for example. I
//       can easily test if a string is None (or len() == 0 when necessary.)
//
// Note: This code is a purposefully forgiving.
//       unicode_segmentation will panic if it ever index in the middle of a grapheme.
//       This code is wastefully hardened against those errors.
//
// Note: Utf-8 Can encode reverse text (right to left), probably downwards, etc.  
//       This crate ignores ALL THAT.
//
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

//  Notes on Graphemes
//	It's complicated...  and with the way the world works, it keeps getting more complicated.
//	One comic suggested that the unicode foundation has the job of trying to direct a flooding
//	river with traffic signs!
//
//	A utf-8 character can be encoded in 1 to 4 bytes. See table below.
//
//	Extended Graphemes Clusters can consist of multiple utf-8 characters, many of which modify
//	the initial character, making certain works 100 or more utf-8 bytes long.
//
//	All together...  it's complicated.
//
//
//    - UTF-8 encoding follows a pattern:
//     - 1-byte sequence: `0xxxxxxx` (ASCII range, covers basic Latin)  Follow Bytes are:
//     - 2-byte sequence: `110xxxxx 10xxxxxx`                   CD      8?, 9?, A?, or B?
//     - 3-byte sequence: `1110xxxx 10xxxxxx 10xxxxxx`          E?
//     - 4-byte sequence: `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx` F?
//
//  pub enum GraphemeCat - https://github.com/unicode-rs/unicode-segmentation/blob/master/src/grapheme.rs
//	Catch All	GC_Any,
//	Carriage Return	GC_CR,
//	Control < 20h	GC_Control,
//	Extended	GC_Extend,
//	Ext Pic		GC_Extended_Pictographic,
//	Conanical Brk	GC_InCB_Consonant,
//	Left Part	GC_L,
//	Line Feed (CR)	GC_LF,
//	Letter/Vowel	GC_LV,
//	Let/Volel/Tone	GC_LVT,
//	Prepend		GC_Prepend,
//	Regional	GC_Regional_Indicator,
//	Visable Join	GC_SpacingMark,
//	Trailing	GC_T,
//	Vowel		GC_V,
//	Zero Width Join	GC_ZWJ,
//
//  Wide Chars Test
//  "I😂J你KツL한M😂N\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}O👨👩👧👦P\u{FF21}Q\u{20000}R"
//
//  Note:
//  Challenge Case - /t is reported as a single character, real width depends on column, and tabstops!!!


/// Return the grapheme at the given string idx
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("grapheme_at_idx {}", grapheme_at_idx(&st, 18)); // Prints H
/// }
/// ```
pub fn grapheme_at_idx(st: &str, idx: usize) -> &str {
    let pos = grapheme_idx_at_idx(&st, idx);
    &st[pos..pos + st[pos..].graphemes(true).next().unwrap_or("").len()]
}

/// Grapheme length in Bytes
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     let num = 7; // Anything between 6 and 17 inclusive
/// 
///     // Grapheme utf8 byte count or length in bytes
///     println!("grapheme_len {}", grapheme_len(&st, num)); // Prints 12 (न्दी uses 12 utf8 bytes)
///     println!("grapheme_len {}", grapheme_len(&st, 18)); // Prints 1 (H uses 1 utf8 byte)
/// }
/// ```
pub fn grapheme_len(st: &str, idx: usize) -> usize {
    let pos = grapheme_idx_at_idx(&st, idx);
    st[pos..].graphemes(true).next().unwrap_or("").len()
}

/// Starting idx of Grapheme
///
/// This returns the starting index for the grapheme given
/// a string byte idx
///
/// Note: This is an example wasteful function, but it
///       usually returns the same index you're providing.
///       
///   
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("grapheme_idx_at_idx {}", grapheme_idx_at_idx(&st, 18)); // Prints 18
/// }
/// ```
pub fn grapheme_idx_at_idx(st: &str, idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    let mut pos = idx;

    if idx >= st.len() {
        return st.len();
    }
    let mut cursor = GraphemeCursor::new(idx, st.len(), true);

    loop {
        while pos > 0 && (st.as_bytes()[pos] & 0xc0) == 0x80 {
            pos -= 1;
        }
        cursor.set_cursor(pos);
        if cursor.is_boundary(st, 0).unwrap_or(false) {
            break;
        }
        pos -= 1;
    }
    pos
}

/// Return the grapheme starting at or after the given byte index in a string.
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     // Grapheme Visual Column Width
///     println!("grapheme_width_at_idx {}", grapheme_width_at_idx(&st, 18)); // Prints 1 (H is 1 column wide)
///
///     let num = 7; // Anything between 6 and 17 inclusive
///     println!(
///         "grapheme_width_at_idx {}",
///         grapheme_width_at_idx(&st, num)
///     ); // Prints 3 (न्दी is 3 columns wide)
/// }
/// ```
pub fn grapheme_width_at_idx(st: &str, idx: usize) -> usize {
    let pos = grapheme_idx_at_idx(&st, idx);
    st[pos..].graphemes(true).next().unwrap_or("").width()
}

/// Next Grapheme from Current Index
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("next_grapheme_from_idx {}", next_grapheme_from_idx(&st, 18)); // Prints 🧑
/// }
/// ```
pub fn next_grapheme_from_idx(st: &str, idx: usize) -> &str {
    let st_len = st.len();
    if idx >= st_len {
        return "";
    }
    let pos = next_grapheme_idx_from_idx(&st, idx);
    st[pos..].graphemes(true).next().unwrap_or("")
}

/// Byte Index of the Next Extended Grapheme from Current Index
///
/// NOTE: This can return the st.len(), meaning an illegal index
///       if this is the last Grapheme in the string!
///
/// Note: In testing, The Rust library currently divides
///       some characters that should be singular "🧑🌾"
///
/// Note: This code is inefficient...  small, but inefficient.
///
/// Note: This function has been modified to be panic proof.
///       The underlying library can panic:
///       unicode-segmentation-1.12.0/src/grapheme.rs:787:29:
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("next_grapheme_from_idx {}", next_grapheme_from_idx(&st, 18)); // Prints 🧑
/// }
/// ```
// Note: next_boundary is written to only require
//       a short chunk of the string instead of the
//       whole thing:
//           next_boundary(st[beg..beg+ch.len_utf8+etc], beg)
//
//       next_boundary can then send an error back saying if
//       it needs more (GraphemeIncomplete::NextChunk)
//
//       It may or may not to land on exact utf8 char
//       boundaries, but it's really hard for prev_grapheme_idx
//       where you need to know the exact info we're wanting.
//
pub fn next_grapheme_idx_from_idx(st: &str, idx: usize) -> usize {
    let st_len = st.len();
    if idx >= st_len {
        return st_len;
    }
    let mut pos = idx;
    while pos > 0 && (st.as_bytes()[pos] & 0xc0) == 0x80 {
        pos -= 1;
    }
    let mut cursor = GraphemeCursor::new(pos, st_len, true);
    cursor
        .next_boundary(st, 0)
        .ok()
        .flatten()
        .unwrap_or_else(|| st_len)
}

/// nth Grapheme
///
/// Note, this will return the st.len() index if it would be
///       past the end of the string even if the string
///       is empty.
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("nth_grapheme {}", nth_grapheme(&st, 1)); // Prints न्दी, the 2nd byte in the string, base 0
///     println!("nth_grapheme {}", nth_grapheme(&st, 2)); // Prints H, the 3rd bytes in the string, base 0
/// }
/// ```
// UUGH - Full Iter to nth!
//
pub fn nth_grapheme(st: &str, nth: usize) -> &str {
    UnicodeSegmentation::grapheme_indices(st, true)
        .nth(nth)
        .map(|(_, g)| g)
        .unwrap_or_else(|| "")
}

/// nth Grapheme Index from nth
///
/// Note, this will return the st.len() index if it would be
///       past the end of the string even if the string
///       is empty.
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("nth_grapheme_idx {}", nth_grapheme_idx(&st, 1)); // Prints 6 (index 6)
///     println!("nth_grapheme_idx {}", nth_grapheme_idx(&st, 2)); // Prints 18 (index 18)
/// }
/// ```
// Uugh - Full Iter!
//
pub fn nth_grapheme_idx(st: &str, nth: usize) -> usize {
    UnicodeSegmentation::grapheme_indices(st, true)
        .nth(nth)
        .map(|(idx, _)| idx)
        .unwrap_or_else(|| st.len())
}

/// nth Grapheme Width
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("nth grapheme_width {}", nth_grapheme_width(&st, 1)); // Prints 3
///     println!("nth grapheme_width {}", nth_grapheme_width(&st, 2)); // Prints 1
/// }
/// ```
pub fn nth_grapheme_width(st: &str, nth: usize) -> usize {
    UnicodeSegmentation::grapheme_indices(st, true)
        .nth(nth)
        .map(|(_, g)| g)
        .unwrap_or_else(|| "")
        .width()
}

/// Num Graphemes In &str
///
/// Note, this will return the st.len() index if it would be
///       past the end of the string even if the string
///       is empty.
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("num_graphemes {}", num_graphemes(&st)); // Prints 12, the string has 12 grapheme clusters total
/// }
/// ```
pub fn num_graphemes(st: &str) -> usize {
    UnicodeSegmentation::grapheme_indices(st, true).count()
}

/// Previoius Grapheme from current idx
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("prev_grapheme_from_idx {}", prev_grapheme_from_idx(&st, 18)); // Prints न्दी
/// }
/// ```
pub fn prev_grapheme_from_idx(st: &str, idx: usize) -> &str {
    if idx == 0 {
        return "";
    }
    let pos = prev_grapheme_idx_from_idx(&st, idx);
    grapheme_at_idx(&st, pos)
}

/// Byte Index of the Previous Extended Grapheme from Current Idx
///
/// NOTE: This will return 0, even when the string is empty.
///
/// Note: In testing, The Rust library currently divides
///       some characters that should be singular "🧑🌾"
///
/// Note: This function has been modified to be panic proof.
///       The underlying library can panic:
///       unicode-segmentation-1.12.0/src/grapheme.rs:787:29:
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!(
///         "prev_grapheme_idx_from_idx {}",
///         prev_grapheme_idx_from_idx(&st, 18)
///     ); // Prints 6
/// }
/// ```
// Note: prev_boundary is written to only require
//       a short chunk of the string instead of the
//       whole thing:
//           let beg = idx - prev-ch.len_utf8 - etc
//           prev_boundary(st[beg..idx], beg)
//
//       This Code could be IMPROVED by implementing PrevChunk
//
//       next_boundary can then send an error back saying if
//       it needs more (GraphemeIncomplete::PrevChunk)
//
//       beg MUST land on exact utf8 char
//       boundaries, but it's really hard for prev_grapheme_idx
//       where you need to know the exact info we're wanting.
pub fn prev_grapheme_idx_from_idx(st: &str, idx: usize) -> usize {
    let st_len = st.len();
    if st_len == 0 {
        return 0;
    }

    let max_len = st_len.saturating_sub(1);

    let mut pos = idx;
    while pos <= max_len && (st.as_bytes()[pos] & 0xc0) == 0x80 {
        pos += 1;
    }
    if pos > st_len {
        pos = st_len;
    }

    let mut cursor = GraphemeCursor::new(pos, st_len, true);
    let pos = match cursor.prev_boundary(st, 0) {
        Ok(Some(prev)) => prev,
        _ => 0, // If we can't find a valid breakpoint or are at the start, return 0
    };
    pos
}

/// Return the string_width
///
/// ```rust
/// use grapheme_utils::*;
/// 
/// fn main() {
///     let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string();
/// 
///     println!("string_width {}", string_width(&st)); // Prints 18, the string uses 18 columns
/// }
/// ```
pub fn string_width(st: &str) -> usize {
    let mut total = 0;
    for (_, grapheme) in st.grapheme_indices(true) {
        total += grapheme.width();
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;
    type TestData = (
        usize, // testnum
        usize, // pgifi: prev_grapheme_idx_from_idx
        usize, // giati: grapheme_idx_at_idx
        usize, // nxgifi: next_grapheme_idx_from_idx
        &'static str, // pgfi: prev_grapheme_from_idx
        &'static str, // gati: grapheme_at_idx
        &'static str, // nxgfi: next_grapheme_from_idx
        usize, // gwi: grapheme width_idx
        usize, // glen: grapheme_len
        &'static str, // nthg: nth_grapheme
        usize, // ngw: nth_grapheme width
        usize, // nthgi: nth_grapheme_idx
        usize, // numg: num_graphemes
        usize, // sw: string_width
    );

    fn run_grapheme_test(st: &str, expected: Vec<TestData>) {
        // Note:  Testing An Error:
        // The Character 🧑🌾 is supposed to be 1 Character.
        // (Look at it in a real editor)
        // I expect this to be fixed eventually, but it's here for now.
        //
        // Note, the 2 éé are differnt.
        // First one from a French AZERTY keyboard ( utf8 bytes c3a9, or codepoint e9)
        // Second from the standard linux Ctrl-U character entry: (utf8 bytes 65 cc 81, or codepoint-modifier 65 301)
        let string_len = st.len();
        println!("Testing grapheme vector for \"{}\"", st);
        assert_eq!(expected.len(), string_len + 2); // Ensure there's one more expected result than the length of the input

        for i in 0..string_len + 2 {
            print!("Testing String: \"{}\", at byte index: {} \n", st, i);
            assert_eq!(expected[i].0, i); // Position index
            print!("i:{} ok, \n", i);

            let pgifi = prev_grapheme_idx_from_idx(st, i);
            print!("prev_grapheme_idx_from_idx");
            assert_eq!(expected[i].1, pgifi);
            println!("  ok:{}", pgifi);

            let giati = grapheme_idx_at_idx(st, i);
            print!("grapheme_idx_at_idx");
            assert_eq!(expected[i].2, giati);
            println!("  ok:{}", giati);

            let nxgifi = next_grapheme_idx_from_idx(st, i);
            print!("next_grapheme_idx_from_idx");
            assert_eq!(expected[i].3, nxgifi);
            println!("  ok:{}\n", nxgifi);

            let pgfi = prev_grapheme_from_idx(st, i);
            print!("prev_grapheme_from_idx");
            assert_eq!(expected[i].4, pgfi);
            println!("  ok:{}", pgfi);

            let gati = grapheme_at_idx(st, i);
            print!("grapheme_at_idx");
            assert_eq!(expected[i].5, gati);
            println!("  ok:{}", gati);

            let nxgfi = next_grapheme_from_idx(st, i);
            print!("next_grapheme_from_idx");
            assert_eq!(expected[i].6, nxgfi);
            println!("  ok:{}\n", nxgfi);

            let gwi = grapheme_width_at_idx(st, i);
            print!("grapheme_width_from_idx");
            assert_eq!(expected[i].7, gwi);
            println!("  ok:{}", gwi);

            let glen = grapheme_len(st, i);
            print!("grapheme_len");
            assert_eq!(expected[i].8, glen);
            println!("  ok:{}", glen);

            let nthg = nth_grapheme(st, i);
            print!("nth_grapheme");
            assert_eq!(expected[i].9, nthg);
            println!("  ok:{}", nthg);

            let ngw = nth_grapheme_width(st, i);
            print!("nth grapheme_width");
            assert_eq!(expected[i].10, ngw);
            println!("  ok:{}", ngw);

            let nthgi = nth_grapheme_idx(st, i);
            print!("nth_grapheme_idx");
            assert_eq!(expected[i].11, nthgi);
            println!("  ok:{}", nthgi);

            let numg = num_graphemes(st);
            print!("num_graphemes");
            assert_eq!(expected[i].12, numg);
            println!("  ok:{}", numg);

            let sw = string_width(st);
            print!("string_width");
            assert_eq!(expected[i].13, sw);
            println!("  ok:{}", sw);
        }
    }

    #[test]
    fn test_grapheme_vectors() {
        let test_cases: Vec<(String, Vec<TestData>)> = vec![
            (
                "".to_string(),
                vec![
                    (0, 0, 0, 0, "", "", "", 0, 0, "", 0, 0, 0, 0),
                    (1, 0, 0, 0, "", "", "", 0, 0, "", 0, 0, 0, 0),
                ],
            ),
            (
                "é".to_string(),
                vec![
                    (0, 0, 0, 2, "", "é", "", 1, 2, "é", 1, 0, 1, 1),
                    (1, 0, 0, 2, "é", "é", "", 1, 2, "", 0, 2, 1, 1),
                    (2, 0, 2, 2, "é", "", "", 0, 0, "", 0, 2, 1, 1),
                    (3, 0, 2, 2, "é", "", "", 0, 0, "", 0, 2, 1, 1),
                ],
            ),
            (
                "é".to_string(),
                vec![
                    (0, 0, 0, 3, "", "é", "", 1, 3, "é", 1, 0, 1, 1),
                    (1, 0, 0, 3, "é", "é", "", 1, 3, "", 0, 3, 1, 1),
                    (2, 0, 0, 3, "é", "é", "", 1, 3, "", 0, 3, 1, 1),
                    (3, 0, 3, 3, "é", "", "", 0, 0, "", 0, 3, 1, 1),
                    (4, 0, 3, 3, "é", "", "", 0, 0, "", 0, 3, 1, 1),
                ],
            ),
            (
                "aé".to_string(),
                vec![
                    (0, 0, 0, 1, "", "a", "é", 1, 1, "a", 1, 0, 2, 2),
                    (1, 0, 1, 4, "a", "é", "", 1, 3, "é", 1, 1, 2, 2),
                    (2, 1, 1, 4, "é", "é", "", 1, 3, "", 0, 4, 2, 2),
                    (3, 1, 1, 4, "é", "é", "", 1, 3, "", 0, 4, 2, 2),
                    (4, 1, 4, 4, "é", "", "", 0, 0, "", 0, 4, 2, 2),
                    (5, 1, 4, 4, "é", "", "", 0, 0, "", 0, 4, 2, 2),
                ],
            ),
            (
                "aé".to_string(),
                vec![
                    (0, 0, 0, 1, "", "a", "é", 1, 1, "a", 1, 0, 2, 2),
                    (1, 0, 1, 3, "a", "é", "", 1, 2, "é", 1, 1, 2, 2),
                    (2, 1, 1, 3, "é", "é", "", 1, 2, "", 0, 3, 2, 2),
                    (3, 1, 3, 3, "é", "", "", 0, 0, "", 0, 3, 2, 2),
                    (4, 1, 3, 3, "é", "", "", 0, 0, "", 0, 3, 2, 2),
                ],
            ),
            (
                "aé".to_string(),
                vec![
                    (0, 0, 0, 1, "", "a", "é", 1, 1, "a", 1, 0, 2, 2),
                    (1, 0, 1, 4, "a", "é", "", 1, 3, "é", 1, 1, 2, 2),
                    (2, 1, 1, 4, "é", "é", "", 1, 3, "", 0, 4, 2, 2),
                    (3, 1, 1, 4, "é", "é", "", 1, 3, "", 0, 4, 2, 2),
                    (4, 1, 4, 4, "é", "", "", 0, 0, "", 0, 4, 2, 2),
                    (5, 1, 4, 4, "é", "", "", 0, 0, "", 0, 4, 2, 2),
                ],
            ),
            (
                "éa".to_string(),
                vec![
                    (0, 0, 0, 2, "", "é", "a", 1, 2, "é", 1, 0, 2, 2),
                    (1, 0, 0, 2, "é", "é", "a", 1, 2, "a", 1, 2, 2, 2),
                    (2, 0, 2, 3, "é", "a", "", 1, 1, "", 0, 3, 2, 2),
                    (3, 2, 3, 3, "a", "", "", 0, 0, "", 0, 3, 2, 2),
                    (4, 2, 3, 3, "a", "", "", 0, 0, "", 0, 3, 2, 2),
                ],
            ),
            (
                "éa".to_string(),
                vec![
                    (0, 0, 0, 3, "", "é", "a", 1, 3, "é", 1, 0, 2, 2),
                    (1, 0, 0, 3, "é", "é", "a", 1, 3, "a", 1, 3, 2, 2),
                    (2, 0, 0, 3, "é", "é", "a", 1, 3, "", 0, 4, 2, 2),
                    (3, 0, 3, 4, "é", "a", "", 1, 1, "", 0, 4, 2, 2),
                    (4, 3, 4, 4, "a", "", "", 0, 0, "", 0, 4, 2, 2),
                    (5, 3, 4, 4, "a", "", "", 0, 0, "", 0, 4, 2, 2),
                ],
            ),
            (
                "abcd".to_string(),
                vec![
                    (0, 0, 0, 1, "", "a", "b", 1, 1, "a", 1, 0, 4, 4),
                    (1, 0, 1, 2, "a", "b", "c", 1, 1, "b", 1, 1, 4, 4),
                    (2, 1, 2, 3, "b", "c", "d", 1, 1, "c", 1, 2, 4, 4),
                    (3, 2, 3, 4, "c", "d", "", 1, 1, "d", 1, 3, 4, 4),
                    (4, 3, 4, 4, "d", "", "", 0, 0, "", 0, 4, 4, 4),
                    (5, 3, 4, 4, "d", "", "", 0, 0, "", 0, 4, 4, 4),
                ],
            ),
            (
                "abcहि".to_string(),
                vec![
                    (0, 0, 0, 1, "", "a", "b", 1, 1, "a", 1, 0, 4, 5),
                    (1, 0, 1, 2, "a", "b", "c", 1, 1, "b", 1, 1, 4, 5),
                    (2, 1, 2, 3, "b", "c", "हि", 1, 1, "c", 1, 2, 4, 5),
                    (3, 2, 3, 9, "c", "हि", "", 2, 6, "हि", 2, 3, 4, 5),
                    (4, 3, 3, 9, "हि", "हि", "", 2, 6, "", 0, 9, 4, 5),
                    (5, 3, 3, 9, "हि", "हि", "", 2, 6, "", 0, 9, 4, 5),
                    (6, 3, 3, 9, "हि", "हि", "", 2, 6, "", 0, 9, 4, 5),
                    (7, 3, 3, 9, "हि", "हि", "", 2, 6, "", 0, 9, 4, 5),
                    (8, 3, 3, 9, "हि", "हि", "", 2, 6, "", 0, 9, 4, 5),
                    (9, 3, 9, 9, "हि", "", "", 0, 0, "", 0, 9, 4, 5),
                    (10, 3, 9, 9, "हि", "", "", 0, 0, "", 0, 9, 4, 5),
                ],
            ),
            (
                "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string(),
                vec![
                    (0, 0, 0, 6, "", "हि", "न्दी", 2, 6, "हि", 2, 0, 12, 18),
                    (1, 0, 0, 6, "हि", "हि", "न्दी", 2, 6, "न्दी", 3, 6, 12, 18),
                    (2, 0, 0, 6, "हि", "हि", "न्दी", 2, 6, "H", 1, 18, 12, 18),
                    (3, 0, 0, 6, "हि", "हि", "न्दी", 2, 6, "🧑", 2, 19, 12, 18),
                    (4, 0, 0, 6, "हि", "हि", "न्दी", 2, 6, "🌾", 2, 23, 12, 18),
                    (5, 0, 0, 6, "हि", "हि", "न्दी", 2, 6, "e", 1, 27, 12, 18),
                    (6, 0, 6, 18, "हि", "न्दी", "H", 3, 12, "‘︀", 1, 28, 12, 18),
                    (7, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "o", 1, 34, 12, 18),
                    (8, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "‘︁", 2, 35, 12, 18),
                    (9, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "r", 1, 41, 12, 18),
                    (10, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "é", 1, 42, 12, 18),
                    (11, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "é", 1, 44, 12, 18),
                    (12, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (13, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (14, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (15, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (16, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (17, 6, 6, 18, "न्दी", "न्दी", "H", 3, 12, "", 0, 47, 12, 18),
                    (18, 6, 18, 19, "न्दी", "H", "🧑", 1, 1, "", 0, 47, 12, 18),
                    (19, 18, 19, 23, "H", "🧑", "🌾", 2, 4, "", 0, 47, 12, 18),
                    (20, 19, 19, 23, "🧑", "🧑", "🌾", 2, 4, "", 0, 47, 12, 18),
                    (21, 19, 19, 23, "🧑", "🧑", "🌾", 2, 4, "", 0, 47, 12, 18),
                    (22, 19, 19, 23, "🧑", "🧑", "🌾", 2, 4, "", 0, 47, 12, 18),
                    (23, 19, 23, 27, "🧑", "🌾", "e", 2, 4, "", 0, 47, 12, 18),
                    (24, 23, 23, 27, "🌾", "🌾", "e", 2, 4, "", 0, 47, 12, 18),
                    (25, 23, 23, 27, "🌾", "🌾", "e", 2, 4, "", 0, 47, 12, 18),
                    (26, 23, 23, 27, "🌾", "🌾", "e", 2, 4, "", 0, 47, 12, 18),
                    (27, 23, 27, 28, "🌾", "e", "‘︀", 1, 1, "", 0, 47, 12, 18),
                    (28, 27, 28, 34, "e", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (29, 28, 28, 34, "‘︀", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (30, 28, 28, 34, "‘︀", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (31, 28, 28, 34, "‘︀", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (32, 28, 28, 34, "‘︀", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (33, 28, 28, 34, "‘︀", "‘︀", "o", 1, 6, "", 0, 47, 12, 18),
                    (34, 28, 34, 35, "‘︀", "o", "‘︁", 1, 1, "", 0, 47, 12, 18),
                    (35, 34, 35, 41, "o", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (36, 35, 35, 41, "‘︁", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (37, 35, 35, 41, "‘︁", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (38, 35, 35, 41, "‘︁", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (39, 35, 35, 41, "‘︁", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (40, 35, 35, 41, "‘︁", "‘︁", "r", 2, 6, "", 0, 47, 12, 18),
                    (41, 35, 41, 42, "‘︁", "r", "é", 1, 1, "", 0, 47, 12, 18),
                    (42, 41, 42, 44, "r", "é", "é", 1, 2, "", 0, 47, 12, 18),
                    (43, 42, 42, 44, "é", "é", "é", 1, 2, "", 0, 47, 12, 18),
                    (44, 42, 44, 47, "é", "é", "", 1, 3, "", 0, 47, 12, 18),
                    (45, 44, 44, 47, "é", "é", "", 1, 3, "", 0, 47, 12, 18),
                    (46, 44, 44, 47, "é", "é", "", 1, 3, "", 0, 47, 12, 18),
                    (47, 44, 47, 47, "é", "", "", 0, 0, "", 0, 47, 12, 18),
                    (48, 44, 47, 47, "é", "", "", 0, 0, "", 0, 47, 12, 18),
                ],
            ),
        ];

        for (st, expected) in test_cases {
            run_grapheme_test(&st, expected);
        }
    }

    #[test]
    fn test_num_graphemes() {
        assert_eq!(num_graphemes(""), 0);
        assert_eq!(num_graphemes("hello"), 5);
        assert_eq!(num_graphemes("😊"), 1);
        assert_eq!(num_graphemes("😊b"), 2);
        assert_eq!(num_graphemes("a😊"), 2);
        assert_eq!(num_graphemes("😊😊"), 2);
        assert_eq!(num_graphemes("hello 😊 world"), 13);
        assert_eq!(num_graphemes("é"), 1);
        let complex_str = "áb̌c̃d̄";
        assert_eq!(num_graphemes(complex_str), 4);
        let flag_str = "🇫🇷"; // French flag
        assert_eq!(num_graphemes(flag_str), 1);
    }
}
