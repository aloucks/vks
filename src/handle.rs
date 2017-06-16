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

macro_rules! define_handle {
    (
        $(#[$attr_name:meta])*
        type $name:ident;

        $(#[$attr_opaque:meta])*
        struct $opaque:ident;
    ) => (
        $( #[$attr_opaque] )*
        #[repr(C)]
        pub struct $opaque(::libc::c_void);

        $( #[$attr_name] )*
        pub type $name = *mut $opaque;
    )
}

macro_rules! define_non_dispatchable_handle {
    (
        $(#[$attr:meta])*
        struct $name:ident;
    ) => (
        $( #[$attr] )*
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            #[inline]
            pub fn null() -> Self {
                Default::default()
            }

            #[inline]
            pub fn is_null(&self) -> bool {
                self.0 == 0
            }

            #[inline]
            pub fn id(&self) -> u64 {
                self.0
            }
        }
    )
}
