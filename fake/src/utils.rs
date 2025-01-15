use crate::{Dummy, Fake, Faker};
use rand::Rng;

pub trait IntoInner {
    type Target;
    fn into_inner(self) -> Self::Target;
}

pub struct EitherFaker<A, B> {
    pub a: A,
    pub b: B,
}

pub struct WrappedVal<T>(pub T);
impl<T> WrappedVal<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}

impl<T> IntoInner for WrappedVal<T> {
    type Target = T;
    fn into_inner(self) -> Self::Target {
        self.0
    }
}

impl<A, B, T> Dummy<EitherFaker<A, B>> for WrappedVal<T>
where
    T: Dummy<A> + Dummy<B>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &EitherFaker<A, B>, rng: &mut R) -> Self {
        if Faker.fake_with_rng(rng) {
            WrappedVal::new(config.a.fake_with_rng(rng))
        } else {
            WrappedVal::new(config.b.fake_with_rng(rng))
        }
    }
}

pub fn either<A, B>(a: A, b: B) -> EitherFaker<A, B> {
    EitherFaker { a, b }
}

#[cfg(feature = "always-true-rng")]
mod always_true_rng {
    use rand::{rngs::mock::StepRng, Error, RngCore};
    use rand_core::impls;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct AlwaysTrueRng {
        inner: StepRng,
        increment: u64,
    }

    impl AlwaysTrueRng {
        /// Implemented using StepRng's `true` range.
        /// See https://github.com/rust-random/rand/pull/1304
        /// Other "always true" rngs can be created so that they are
        /// more random, but that is not desirable for tests.
        pub fn new(initial: u64, increment: u64) -> Self {
            AlwaysTrueRng {
                inner: StepRng::new(initial, increment),
                increment,
            }
        }
    }

    impl Default for AlwaysTrueRng {
        fn default() -> Self {
            AlwaysTrueRng::new(1 << 31, (1 << 31) + 1)
        }
    }

    impl RngCore for AlwaysTrueRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            let mut rv = self.inner.next_u64();
            if rv & (1 << 31) == 0 {
                self.inner = StepRng::new(rv | 1 << 31, self.increment);
                rv = self.inner.next_u64();
            }
            rv
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest);
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }
}

#[cfg(feature = "always-true-rng")]
pub use always_true_rng::AlwaysTrueRng;
