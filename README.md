# Grapheme Utils
Robust Grapheme Utils to make working with Extended Grapheme Clusters nearly as easy as working with Ascii only strings. See Async-Editor an example of a Grapheme enabled editor.

## What is this?

Grapheme Utils are a collection of handy utilities to make working with Extended Grapheme Clusters, measuring character widths, and examining the previous, current or next grapheme cluster or string index as straight-forward as possible.

**Note**: Ease of use over Ideomatic Rust

This code will return a "" grapheme or an index past the end of the string instead of None. Instead of testing for None, test for .len() == 0 for example.

**Note**: This code is very forgiving, and handles any index values that hits any part of a grapheme cluster as though it pointed at the beginning.

**Note**: This crate has not been tested with any of the very special unicode Utf-8 modes like reverse or downward text.

## Example Usage

```rust
use grapheme-utils::GraphemeUtils;

fn main() {

    let st = "हिन्दीH🧑🌾e‘︀o‘︁réé".to_string(),

    print!("num_graphemes {}", num_graphemes(st));  // Prints 12, the string has 12 grapheme clusters total

    print!("string_width {}", string_width(st));  // Prints 18, the string uses 18 columns


    print!("nth_grapheme_idx", nth_grapheme_idx(st, 1));  // Prints 6 (index 6)
    print!("nth_grapheme_idx", nth_grapheme_idx(st, 2));  // Prints 18 (index 18)

    print!("nth_grapheme {}", nth_grapheme(st, 1));  // Prints न्दी, the 2nd byte in the string, base 0
    print!("nth_grapheme {}", nth_grapheme(st, 2)); // Prints H, the 3rd bytes in the string, base 0

    print!("nth grapheme_width", nth_grapheme_width(st, 1));  // Prints 3
    print!("nth grapheme_width", nth_grapheme_width(st, 2));  // Prints 1


    // Grapheme Visual Column Width
    print!("grapheme_width_at_idx {}", grapheme_width_at_idx(st, 18)); // Prints 1 (H is 1 column wide)
    let num = Anything between 6 and 17 inclusive
    print!("grapheme_width_at_idx {}", grapheme_width_at_idx(st, num )); // Prints 3 (न्दी is 3 columns wide)

    // Grapheme utf8 byte count
    print!("grapheme_len", grapheme_len(st, num));  // Prints 12 (न्दी uses 12 utf8 bytes)
    print!("grapheme_len", grapheme_len(st, 18));  // Prints 1 (H uses 1 utf8 byte)



    // Matrix of grapheme functions:
    // [previous, current or next] grapheme given an index
    // 
    // Outputing either the
    //  [extended grapheme cluster string, or starting string index]
    //
    // Take special note of the 2 characters before and after the H.
    // The characer before न्दी starts at string index 6, H is at 18, and 🧑 is at index 19
    //

    // Output string index
    print!("prev_grapheme_idx_from_idx {}", prev_grapheme_idx_from_idx(st, 18)); // Prints 6

    print!("grapheme_idx_at_idx {}", grapheme_idx_at_idx(st, 18)); // Prints 18

    print!("next_grapheme_idx_from_idx {}", next_grapheme_idx_from_idx(st, 18)); // Prints 19


    // Output extended grapheme cluster
    print!("prev_grapheme_from_idx {}"m prev_grapheme_from_idx(st, 18)); // Prints न्दी

    print!("grapheme_at_idx", grapheme_at_idx(st, 18));  // Prints H

    print!("next_grapheme_from_idx {}", next_grapheme_from_idx(st, 18)); // Prints 🧑


    // Note, all of the above matrix of functions work with num, the range of inputs
    //       instead of requiring the exact start to each grapheme.
    // Examining the test code may be instructive
}
```

## Features

- **Easy to Use**: This crate as been designed to be as easy to use as possible.
- **Robust and Forgiving**: This crate accepts any index value without panicing.

# Installation

Add `grapheme-utils` to your `Cargo.toml`:

```toml
[dependencies]
grapheme-utils = "0.1"
```

Then, include it in your Rust project:

```rust
use grapheme_utils::*;
```
