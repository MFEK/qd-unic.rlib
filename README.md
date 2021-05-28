# QD-UNIC: Unicode and Internationalization Crates for Rust

[![UNIC-logo](docs/images/UNIC-logo.png)](https://github.com/MFEK/rust-unic.rlib/)

<https://github.com/MFEK/rust-unic.rlib/>

**This is a fork!** Scroll down.

## Original project blurb

**UNIC** is a project to develop components for the Rust programming language
to provide high-quality and easy-to-use crates for Unicode
and Internationalization data and algorithms. In other words, it's like
[ICU](http://site.icu-project.org/) for Rust, written completely in Rust, mostly
in *safe* mode, but also benefiting from performance gains of *unsafe* mode when
possible.

## Fork

This is the MFEK fork of UNIC—**QD-UNIC**, “Quick and Dirty UNIC”. The original
UNIC has been stuck for years on Unicode 10.0 because they're having trouble
figuring out what to do with [PR №
226](https://github.com/open-i18n/rust-unic/pull/226).

The error is that some segmentation tests don't compile. MFEK doesn't even need
that module, we only need blocks, names, and categories. So, I'm just updating
without fixing it. I ran `unic-gen` on the Unicode 14.0.0a data.

Sure, that's not the right way to do things. Sorry. That's why this is a fork.
Use it if you can handle the caveats, avoid it if you can't and go contribute
to UNIC.

## How to Use (QD-)UNIC

In `Cargo.toml`:

```toml
[dependencies]
unic = "0.9.0"  # This has Unicode 10.0.0 data and algorithms
```

And in `main.rs`:

```rust
extern crate unic;

use unic::ucd::common::is_alphanumeric;
use unic::bidi::BidiInfo;
use unic::normal::StrNormalForm;
use unic::segment::{GraphemeIndices, Graphemes, WordBoundIndices, WordBounds, Words};
use unic::ucd::normal::compose;
use unic::ucd::{is_cased, Age, BidiClass, CharAge, CharBidiClass, StrBidiClass, UnicodeVersion};

fn main() {

    // Age

    assert_eq!(Age::of('A').unwrap().actual(), UnicodeVersion { major: 1, minor: 1, micro: 0 });
    assert_eq!(Age::of('\u{A0000}'), None);
    assert_eq!(
        Age::of('\u{10FFFF}').unwrap().actual(),
        UnicodeVersion { major: 2, minor: 0, micro: 0 }
    );

    if let Some(age) = '🦊'.age() {
        assert_eq!(age.actual().major, 9);
        assert_eq!(age.actual().minor, 0);
        assert_eq!(age.actual().micro, 0);
    }

    // Bidi

    let text = concat![
        "א",
        "ב",
        "ג",
        "a",
        "b",
        "c",
    ];

    assert!(!text.has_bidi_explicit());
    assert!(text.has_rtl());
    assert!(text.has_ltr());

    assert_eq!(text.chars().nth(0).unwrap().bidi_class(), BidiClass::RightToLeft);
    assert!(!text.chars().nth(0).unwrap().is_ltr());
    assert!(text.chars().nth(0).unwrap().is_rtl());

    assert_eq!(text.chars().nth(3).unwrap().bidi_class(), BidiClass::LeftToRight);
    assert!(text.chars().nth(3).unwrap().is_ltr());
    assert!(!text.chars().nth(3).unwrap().is_rtl());

    let bidi_info = BidiInfo::new(text, None);
    assert_eq!(bidi_info.paragraphs.len(), 1);

    let para = &bidi_info.paragraphs[0];
    assert_eq!(para.level.number(), 1);
    assert_eq!(para.level.is_rtl(), true);

    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);
    assert_eq!(
        display,
        concat![
            "a",
            "b",
            "c",
            "ג",
            "ב",
            "א",
        ]
    );

    // Case

    assert_eq!(is_cased('A'), true);
    assert_eq!(is_cased('א'), false);

    // Normalization

    assert_eq!(compose('A', '\u{030A}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");

    // Segmentation

    assert_eq!(
        Graphemes::new("a\u{310}e\u{301}o\u{308}\u{332}").collect::<Vec<&str>>(),
        &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]
    );

    assert_eq!(
        Graphemes::new("a\r\nb🇺🇳🇮🇨").collect::<Vec<&str>>(),
        &["a", "\r\n", "b", "🇺🇳", "🇮🇨"]
    );

    assert_eq!(
        GraphemeIndices::new("a̐éö̲\r\n").collect::<Vec<(usize, &str)>>(),
        &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")]
    );

    assert_eq!(
        Words::new(
            "The quick (\"brown\") fox can't jump 32.3 feet, right?",
            |s: &&str| s.chars().any(is_alphanumeric),
        ).collect::<Vec<&str>>(),
        &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"]
    );

    assert_eq!(
        WordBounds::new("The quick (\"brown\")  fox").collect::<Vec<&str>>(),
        &["The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", " ", "fox"]
    );

    assert_eq!(
        WordBoundIndices::new("Brr, it's 29.3°F!").collect::<Vec<(usize, &str)>>(),
        &[
            (0, "Brr"),
            (3, ","),
            (4, " "),
            (5, "it's"),
            (9, " "),
            (10, "29.3"),
            (14, "°"),
            (16, "F"),
            (17, "!")
        ]
    );
}
```

You can find more examples under [`examples`](examples/) and [`tests`](tests/)
directories. (And more to be added as UNIC expands...)


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


## Code of Conduct

UNIC project follows **The Rust Code of Conduct**. You can find a copy of it in
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) or online at
<https://www.rust-lang.org/conduct.html>.
