use crate::{Dummy, Fake, Faker};
use rand::Rng;

pub struct Decimal;
pub struct NegativeDecimal;
pub struct PositiveDecimal;
pub struct NoDecimalPoints;

impl Dummy<Faker> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rust_decimal::Decimal::from_parts(
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
        )
    }
}

impl Dummy<Decimal> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Decimal, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl Dummy<NegativeDecimal> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &NegativeDecimal, rng: &mut R) -> Self {
        rust_decimal::Decimal::from_parts(
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            true,
            Faker.fake_with_rng(rng),
        )
    }
}

impl Dummy<PositiveDecimal> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &PositiveDecimal, rng: &mut R) -> Self {
        rust_decimal::Decimal::from_parts(
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            false,
            Faker.fake_with_rng(rng),
        )
    }
}

impl Dummy<NoDecimalPoints> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &NoDecimalPoints, rng: &mut R) -> Self {
        let mut decimal: rust_decimal::Decimal = Faker.fake_with_rng(rng);
        decimal.set_scale(0).expect("failed to set scale");
        decimal
    }
}
