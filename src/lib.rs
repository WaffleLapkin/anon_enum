//! Demo of anonymous enums. Demo is made for illustrating [**Anonymous Enums RFC (take 4)**][rfc].
//!
//! This demo uses macros for making syntax proposed in the RFC possible (`Ae!` for types,
//! `ae!` for expressions and `ae_pat!` for patterns). If you'll remove all the macros from tests
//! that should be compilable code under proposed `#![feature(anonymous_enums)]`.
//!
//! **Note**: apart from the RFC, this crate uses `Either` + `Never` in a hlist-like way, so it's
//! capable of implementing traits on enums of any size. However with `anonymous_enums` in the
//! compiler that would require additional feature and thus explicitly left out from the RFC.
//!
//! Also this demo supports one variant (`Ae![T]`, `ae!(::0(t))`, `ae_pat!(::0(_))`) and empty
//! enums (`Ae![]`), both of which are in "Unresolved questions" section in the RFC.
//!
//! [rfc]: https://github.com/WaffleLapkin/rfcs/blob/anonymous-enums/text/0000-anonymous-enums.md
#![feature(exhaustive_patterns)]

/// trait implementations for Either and Never
mod impls;

/// Analog to `!`.
///
/// Implementation detail of [`Ae`], [`ae`], [`ae_pat`] macros
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Never {}

/// Either `L` or `R`.
///
/// Analog to any other either like [`either::Either`].
///
/// Implementation detail of [`Ae`], [`ae`], [`ae_pat`] macros
///
/// [`either::Either`]: https://docs.rs/either/1.5.3/either/enum.Either.html
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Ae {
    [] => { $crate::Never };
    [$head:ty $( | $tail:ty )* ] => {
        $crate::Either<$head, $crate::Ae![$( $tail )|*]>
    }
}

#[macro_export]
macro_rules! ae {
    ($( <$( $ty:ty )|+> )? :: $l:tt ($e:expr) ) => {
        $crate::ae!($( <$( $ty )|+> )? :: $l)($e)
    };
    ($( <$( $ty:ty )|+> )? :: 0 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Left(it) }
    };
    ($( <$( $ty:ty )|+> )? :: 1 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Left(it)) }
    };
    ($( <$( $ty:ty )|+> )? :: 2 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Right($crate::Either::Left(it))) }
    };
    ($( <$( $ty:ty )|+> )? :: 3 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left(it))))) }
    };
    ($( <$( $ty:ty )|+> )? :: 4 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left(it)))))) }
    };
    ($( <$( $ty:ty )|+> )? :: 5 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left(it))))))) }
    };
    ($( <$( $ty:ty )|+> )? :: 6 ) => {
        |it| $( -> $crate::Ae![$( $ty )|+]  )? { $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left(it)))))))) }
    };
    ($( <$( $ty:ty )|+> )? :: $l:literal) => {
        compile_error!("Only creation of anonymous enums of size <= 6 is supported by this demo")
    }
}

#[macro_export]
macro_rules! ae_pat {
    (:: 0 ($p:pat) ) => {
        $crate::Either::Left($p)
    };
    (:: 1 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Left($p))
    };
    (:: 2 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Left($p)))
    };
    (:: 3 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left($p)))))
    };
    (:: 4 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left($p))))))
    };
    (:: 5 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left($p)))))))
    };
    (:: 6 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Left($p))))))))
    };

    (never :: 0 ($p:pat) ) => {
        $crate::Either::Right($p)
    };
    (never :: 1 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($p))
    };
    (never :: 2 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($p)))
    };
    (never :: 3 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($p)))))
    };
    (never :: 4 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($p))))))
    };
    (never :: 5 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($p)))))))
    };
    (never :: 6 ($p:pat) ) => {
        $crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($crate::Either::Right($p))))))))
    };

    ( $(never)? :: $l:literal) => {
        compile_error!("Only patterns for anonymous enums of size <= 6 are supported by this demo")
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::str::FromStr;
    use std::io;

    fn fun(arg: Ae![i32 | i32 | String]) -> String {
        match arg {
            ae_pat!(::0(int)) | ae_pat!(::1(int)) => int.to_string(),
            ae_pat!(::2(string)) => string,
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(
            fun(ae!(::0(1))),
            "1"
        );

        assert_eq!(
            fun(ae!(::1(42))),
            "42"
        );

        assert_eq!(
            fun(ae!(::2(String::from("hi")))),
            "hi"
        );
    }

    fn read_file(path: &str) -> io::Result<String> {
        if path == "./foo" {
            Ok(String::from("hello"))
        } else if path == "./bar" {
            Ok(String::from("1"))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "only \"./foo\" and \"./bar\" paths are permitted"))
        }
    }

    fn read<T>(path: &str) -> Result<T, Ae![io::Error | T::Err]>
    where
        T: FromStr,
    {
        // There are 2 thing you need to note:
        //   1. We can't use `?` dicerectly, since `A | B` can't implement both `From<A>` and `From<B>` :(
        //   2. We can use `::0` as a function, just like `Either::Left`, `Enum::Var`
        let string = read_file(path).map_err(ae![::0])?;
        T::from_str(&string[..]).map_err(ae![::1])
    }

    #[test]
    fn read_test() {
        assert!(matches!(read::<i32>("./path"), Err(ae_pat![::0(_)])));
        assert!(matches!(read::<i32>("./foo"), Err(ae_pat![::1(_)])));
        assert!(matches!(read::<i32>("./bar"), Ok(1)));
    }

    #[test]
    fn r#async() {
        // Actually we don't run this, but at least this compiles

        async fn a() {}
        async fn b() {}

        async {
            let fut: Ae![_ | _] = if true { ae![::0(a())] } else { ae![::1(b())] };
            //           ^^^^^ this is weird, but without it code doesn't compile
            let () = fut.await;
        };
    }

    #[test]
    fn single() {
        let x: Ae![&str] = ae!(::0("x"));
        let ae_pat!(::0(unx)) = x;
        assert_eq!(unx, "x");
    }

    #[test]
    fn never() {
        let never: Ae![] = return;
        let never_: i32 = match never {};
        let never_: String = match never {};
    }
}
