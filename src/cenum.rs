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

macro_rules! cenum {
    ($name:ident: $ty:ty { $( $(#[$cond:meta])* const $symbol:ident = $value:expr,)* }) => (
        #[repr(C)]
        #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $name($ty);

        impl $name {
            pub fn from_raw(v: $ty) -> Self {
                $name(v)
            }

            pub fn as_raw(&self) -> $ty {
                self.0
            }
        }

        $(
            $(#[$cond])*
            pub const $symbol: $name = $name($value);
        )*

        #[allow(unused_mut)]
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, concat!(stringify!($name), "({} = "), self.0)?;
                let mut first = true;

                $(
                    $(#[$cond])*
                    {
                        if self.0 == $value {
                            if !first {
                                write!(f, ", ")?;
                            }

                            write!(f, stringify!($symbol))?;
                            first = false;
                        }
                    }
                )*

                if first {
                    write!(f, "<unknown>")?;
                }

                write!(f, ")")
            }
        }
    )
}
