// Many examples are from
// https://github.com/rust-lang/rust/issues/110111#issuecomment-1517800781
#![deny(rustdoc::broken_intra_doc_links)]

//! This test case is closely linked to [raphlinus/pulldown-cmark#441], getting offsets of
//! link components. In particular, pulldown-cmark doesn't provide the offsets of the contents
//! of a link.
//!
//! To work around this, rustdoc parses parts of a link definition itself. This is basically a
//! test suite for that link syntax parser.
//!
//! [raphlinus/pulldown-cmark#441]: https://github.com/raphlinus/pulldown-cmark/issues/441

use std::clone::Clone;

// Basic version //

/// [`struct@Clone`] //~ERROR link
pub struct LinkToCloneWithBackquotes;

/// [```struct@Clone```] //~ERROR link
pub struct LinkToCloneWithMultipleBackquotes;

/// [  `  struct@Clone  `  ] //~ERROR link
pub struct LinkToCloneWithSpacesAndBackquotes;

/// [  `Clone ()`  ] //~ERROR link
pub struct LinkToCloneWithSpacesBackquotesAndParens;

/// [`Clone ()`  ] //~ERROR link
pub struct LinkToCloneWithSpacesEndBackquotesAndParens;

/// [  `Clone ()`] //~ERROR link
pub struct LinkToCloneWithSpacesStartBackquotesAndParens;

/// [```Clone ()```] //~ERROR link
pub struct LinkToCloneWithMultipleBackquotesAndParens;

/// [```Clone \(\)```] // not URL-shaped enough
pub struct LinkToCloneWithMultipleBackquotesAndEscapedParens;

/// [  ```  Clone ()  ```  ] //~ERROR link
pub struct LinkToCloneWithSpacesMultipleBackquotesAndParens;

/// [ x \] ] // not URL-shaped enough
pub struct LinkWithEscapedCloseBrace;

/// [ x \[ ] // not URL-shaped enough
pub struct LinkWithEscapedOpenBrace;

/// [ x \( ] // not URL-shaped enough
pub struct LinkWithEscapedCloseParen;

/// [ x \) ] // not URL-shaped enough
pub struct LinkWithEscapedOpenParen;

/// [ Clone \(\) ] // not URL-shaped enough
pub struct LinkWithEscapedParens;

// [][] version //

/// [x][ struct@Clone] //~ERROR link
pub struct XLinkToCloneWithStartSpace;

/// [x][struct@Clone ] //~ERROR link
pub struct XLinkToCloneWithEndSpace;

/// [x][Clone\(\)] not URL-shaped enough
pub struct XLinkToCloneWithEscapedParens;

/// [x][`Clone`] not URL-shaped enough
pub struct XLinkToCloneWithBackquotes;

/// [x][Clone()] //~ERROR link
pub struct XLinkToCloneWithUnescapedParens;

/// [x][Clone  ()] //~ERROR link
pub struct XLinkToCloneWithUnescapedParensAndDoubleSpace;

/// [x][Clone  [] //~ERROR unresolved link to `x`
pub struct XLinkToCloneWithUnmatchedOpenParenAndDoubleSpace;

/// [x][Clone  \[] // not URL-shaped enough
pub struct XLinkToCloneWithUnmatchedEscapedOpenParenAndDoubleSpace;

/// [x][Clone  \]] // not URL-shaped enough
pub struct XLinkToCloneWithUnmatchedEscapedCloseParenAndDoubleSpace;

// []() version //

/// [w]( struct@Clone) //~ERROR link
pub struct WLinkToCloneWithStartSpace;

/// [w](struct@Clone ) //~ERROR link
pub struct WLinkToCloneWithEndSpace;

/// [w](Clone\(\)) //~ERROR link
pub struct WLinkToCloneWithEscapedParens;

/// [w](`Clone`) not URL-shaped enough
pub struct WLinkToCloneWithBackquotes;

/// [w](Clone()) //~ERROR link
pub struct WLinkToCloneWithUnescapedParens;

/// [w](Clone  ()) not URL-shaped enough
pub struct WLinkToCloneWithUnescapedParensAndDoubleSpace;

/// [w](Clone  () //~ERROR unresolved link to `w`
pub struct WLinkToCloneWithUnmatchedOpenParenAndDoubleSpace;

/// [w](Clone  \() //~ERROR unresolved link to `w`
pub struct WLinkToCloneWithUnmatchedEscapedOpenParenAndDoubleSpace;

/// [w](Clone  \)) //~ERROR unresolved link to `w`
pub struct WLinkToCloneWithUnmatchedEscapedCloseParenAndDoubleSpace;

// References

/// The [cln][] link here is going to be unresolved, because `Clone()` gets
//~^ ERROR link
/// rejected in Markdown for not being URL-shaped enough.
/// [cln]: Clone()
//~^ ERROR link
pub struct LinkToCloneWithParensInReference;

/// The [cln][] link here is going to produce a good inline suggestion
///
/// [cln]: struct@Clone
//~^ ERROR link
pub struct LinkToCloneWithWrongPrefix;

/// The [cln][] link here will produce a good inline suggestion
///
/// [cln]: Clone\(\)
//~^ ERROR link
pub struct LinkToCloneWithEscapedParensInReference;

/// The [cln][] link here will produce a good inline suggestion
///
/// [cln]: struct\@Clone
//~^ ERROR link
pub struct LinkToCloneWithEscapedAtsInReference;


/// This link reference definition isn't used, but since it is still parsed,
/// it should still produce a warning.
///
/// [cln]: struct\@Clone
//~^ ERROR link
pub struct UnusedLinkToCloneReferenceDefinition;

/// <https://github.com/rust-lang/rust/issues/133150>
///
/// - [`SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER`]: the
//~^ ERROR link
///   `(__unsafe_unretained)` NSWindow associated with the window, if you want
///   to wrap an existing window.
/// - [`SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER`]: the `(__unsafe_unretained)`
///   NSView associated with the window, defaults to `[window contentView]`
pub fn a() {}
#[allow(nonstandard_style)]
pub struct SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER;
#[allow(nonstandard_style)]
pub struct SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER;
