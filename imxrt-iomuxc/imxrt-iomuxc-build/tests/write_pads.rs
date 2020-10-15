use imxrt_iomuxc_build::{write_pads, PadRange};

#[test]
fn test_write_pads() {
    let expected_tokens = quote::quote! {
        /// Contains all of the pads
        ///
        /// This module is auto-generated by the `imxrt-iomuxc-build` crate. See
        /// that crate for more information.
        mod pads {
            #![allow(non_camel_case_types)] // Conform with reference manual

            #[doc = "Pads with the prefix 'FOO'"]
            pub mod foo {
                use crate::{ErasedPad, Pad, consts::*};
                use super::super::bases::*;

                pub type FOO_02 = Pad<FOO, U2>;
                pub type FOO_03 = Pad<FOO, U3>;

                #[doc = "Pads with the prefix 'FOO'"]
                pub struct Pads {
                    pub p02: FOO_02,
                    pub p03: FOO_03
                }

                #[doc = "Erased pads with the prefix 'FOO'"]
                ///
                /// Use [`Pads::erase()`](struct.Pads.html#method.erase) to get an `ErasedPads` instance.
                pub type ErasedPads = [ErasedPad; 2usize];

                impl Pads {
                    /// Take all pads from this group
                    ///
                    /// # Safety
                    ///
                    /// You may safely call this once to acquire all of the pads. Subsequent calls
                    /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                    /// at the start of your program.
                    ///
                    /// Know that the top-level [`Pads::new()`](../struct.Pads.html#method.new) will call this `new()`.
                    pub const unsafe fn new() -> Pads {
                        Pads {
                            p02: <FOO_02>::new(),
                            p03: <FOO_03>::new()
                        }
                    }

                    /// Erase all of the pads
                    ///
                    /// The return type is an array, where the index indicates the pad offset
                    /// from the start of the group. For example, `AD_B0_03` would be referenced
                    /// as `erased_pads[3]`.
                    ///
                    /// See [`ErasedPad`](../struct.ErasedPad.html) for more information.
                    pub fn erase(self) -> ErasedPads {
                        [
                            self.p02.erase(),
                            self.p03.erase()
                        ]
                    }
                }
            }

            #[doc = "Pads with the prefix 'BAR'"]
            pub mod bar {
                use crate::{ErasedPad, Pad, consts::*};
                use super::super::bases::*;

                pub type BAR_37 = Pad<BAR, U37>;
                pub type BAR_38 = Pad<BAR, U38>;

                #[doc = "Pads with the prefix 'BAR'"]
                pub struct Pads {
                    pub p37: BAR_37,
                    pub p38: BAR_38
                }

                #[doc = "Erased pads with the prefix 'BAR'"]
                ///
                /// Use [`Pads::erase()`](struct.Pads.html#method.erase) to get an `ErasedPads` instance.
                pub type ErasedPads = [ErasedPad; 2usize];

                impl Pads {
                    /// Take all pads from this group
                    ///
                    /// # Safety
                    ///
                    /// You may safely call this once to acquire all of the pads. Subsequent calls
                    /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                    /// at the start of your program.
                    ///
                    /// Know that the top-level [`Pads::new()`](../struct.Pads.html#method.new) will call this `new()`.
                    pub const unsafe fn new() -> Pads {
                        Pads {
                            p37: <BAR_37>::new(),
                            p38: <BAR_38>::new()
                        }
                    }

                    /// Erase all of the pads
                    ///
                    /// The return type is an array, where the index indicates the pad offset
                    /// from the start of the group. For example, `AD_B0_03` would be referenced
                    /// as `erased_pads[3]`.
                    ///
                    /// See [`ErasedPad`](../struct.ErasedPad.html) for more information.
                    pub fn erase(self) -> ErasedPads {
                        [
                            self.p37.erase(),
                            self.p38.erase()
                        ]
                    }
                }
            }

            /// All of the pads
            ///
            /// # Convention
            ///
            /// The members of `Pads` are additional structs that provide pads as
            /// objects. The `p` prefix of each pad denotes "pad."
            pub struct Pads {
                pub foo: foo::Pads,
                pub bar: bar::Pads
            }

            /// All of the pads, with all types erased
            ///
            /// # Convention
            ///
            /// The members of `ErasedPads` are arrays that provide erased pads
            /// as objects. Pads are grouped by their prefix, like `ad_b0`. The array
            /// index corresponds to the final pad identifier.
            ///
            /// Use [`Pads::erase()`](struct.Pads.html#method.erase) to get an `ErasedPads`.
            pub struct ErasedPads {
                pub foo: foo::ErasedPads,
                pub bar: bar::ErasedPads
            }

            impl Pads {
                /// Take all of the pads
                ///
                /// # Safety
                ///
                /// You may safely call this once to acquire all of the pads. Subsequent calls
                /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                /// at the start of your program.
                pub const unsafe fn new() -> Pads {
                    Pads {
                        foo: <foo::Pads>::new(),
                        bar: <bar::Pads>::new()
                    }
                }

                /// Erase the types of all pads
                ///
                /// See [`ErasedPad`](struct.ErasedPad.html) for more information.
                pub fn erase(self) -> ErasedPads {
                    ErasedPads {
                        foo: self.foo.erase(),
                        bar: self.bar.erase()
                    }
                }
            }
        }
    };
    let expected = expected_tokens.to_string();
    let mut actual = Vec::new();
    write_pads(
        &mut actual,
        vec![&PadRange::new("FOO", 2..4), &PadRange::new("BAR", 37..39)],
    )
    .unwrap();
    assert_eq!(std::str::from_utf8(&actual).unwrap(), expected);
}