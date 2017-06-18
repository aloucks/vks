// Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

// When repr(transparent) is stabilized, this macro will be removed and we'll switch back to the
// actual bitflags crate.

macro_rules! vks_bitflags {
    (
        $( #[$attr:meta] )*
        pub struct $name:ident: $ty:ty {
            $(
                $( #[$flag_attr:meta] )*
                const $flag:ident = $value:expr;
            )+
        }
    ) => (

    $( #[$attr] )*
    pub type $name = $ty;

    $(
        $( #[$flag_attr] )*
        pub const $flag: $name = $value;
    )+
)}
