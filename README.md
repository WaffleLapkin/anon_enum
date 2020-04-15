## Anonymous Enams Demo

Demo of anonymous enums. Demo is made for illustrating [**Anonymous Enums RFC (take 4)**][rfc].

This demo uses macros for making syntax proposed in the RFC possible (`Ae!` for types,
`ae!` for expressions and `ae_pat!` for patterns). If you'll remove all the macros from tests
that should be compilable code under proposed `#![feature(anonymous_enums)]`.

**Note**: apart from the RFC, this crate uses `Either` + `Never` in a hlist-like way, so it's
capable of implementing traits on enums of any size. However with `anonymous_enums` in the
compiler that would require additional feature and thus explicitly left out from the RFC.

Also this demo supports one variant (`Ae![T]`, `ae!(::0(t))`, `ae_pat!(::0(_))`) and empty
enums (`Ae![]`), both of which are in "Unresolved questions" section in the RFC.

[rfc]: https://github.com/WaffleLapkin/rfcs/blob/anonymous-enums/text/0000-anonymous-enums.md