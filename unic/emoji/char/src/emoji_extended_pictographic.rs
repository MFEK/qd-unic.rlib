// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `ExtendedPictographic` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`ExtendedPictographic`](https://www.unicode.org/reports/tr51/#Emoji_Properties).
    ///
    /// The value is `true` for characters that have emoji extended_pictographic by default.
    pub struct ExtendedPictographic(bool) {
        abbr => "ExtendedPictographic";
        long => "ExtendedPictographic";
        human => "ExtendedPictographic";

        data_table_path => "../tables/emoji_extended_pictographic.rsv";
    }

    /// The value is `true` for characters that have emoji extended_pictographic by default.
    pub fn is_emoji_extended_pictographic(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_emoji_extended_pictographic;

        assert!(is_emoji_extended_pictographic('\u{a9}'));
        assert!(is_emoji_extended_pictographic('\u{1fffc}'));
        assert!(!is_emoji_extended_pictographic('\u{1fb00}'));
    }
}
