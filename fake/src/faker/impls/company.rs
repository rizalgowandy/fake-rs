use crate::faker::company::raw::*;
use crate::faker::name::raw::LastName;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::SliceRandom;
use rand::Rng;

impl<L: Data> Dummy<CompanySuffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CompanySuffix<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CompanySuffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CompanySuffix<L>, rng: &mut R) -> Self {
        L::COMPANY_SUFFIX.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<CompanyName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &CompanyName<L>, rng: &mut R) -> Self {
        let name_tpl = *L::COMPANY_NAME_TPLS.choose(rng).unwrap();
        name_tpl
            .replace("{Name_1}", LastName(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Name_2}", LastName(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Suffix}", CompanySuffix(c.0).fake_with_rng::<&str, _>(rng))
    }
}

impl<L: Data> Dummy<Buzzword<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Buzzword<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BUZZWORD_HEAD.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Buzzword<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Buzzword<L>, rng: &mut R) -> Self {
        L::COMPANY_BUZZWORD_HEAD.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<BuzzwordMiddle<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BuzzwordMiddle<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BUZZWORD_MIDDLE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<BuzzwordMiddle<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BuzzwordMiddle<L>, rng: &mut R) -> Self {
        L::COMPANY_BUZZWORD_MIDDLE.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<BuzzwordTail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BuzzwordTail<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BUZZWORD_TAIL.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<BuzzwordTail<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BuzzwordTail<L>, rng: &mut R) -> Self {
        L::COMPANY_BUZZWORD_TAIL.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<CatchPhrase<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &CatchPhrase<L>, rng: &mut R) -> Self {
        L::COMPANY_CATCH_PHASE_TPL
            .replace("{Head}", Buzzword(c.0).fake_with_rng::<&str, _>(rng))
            .replace(
                "{Middle}",
                BuzzwordMiddle(c.0).fake_with_rng::<&str, _>(rng),
            )
            .replace("{Tail}", BuzzwordTail(c.0).fake_with_rng::<&str, _>(rng))
    }
}

impl<L: Data> Dummy<BsVerb<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsVerb<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BS_VERBS.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<BsVerb<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsVerb<L>, rng: &mut R) -> Self {
        L::COMPANY_BS_VERBS.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<BsAdj<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsAdj<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BS_ADJ.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<BsAdj<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsAdj<L>, rng: &mut R) -> Self {
        L::COMPANY_BS_ADJ.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<BsNoun<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsNoun<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_BS_NOUNS.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<BsNoun<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BsNoun<L>, rng: &mut R) -> Self {
        L::COMPANY_BS_NOUNS.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<Bs<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Bs<L>, rng: &mut R) -> Self {
        L::COMPANY_BS_TPL
            .replace("{Verb}", BsVerb(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Adj}", BsAdj(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Noun}", BsNoun(c.0).fake_with_rng::<&str, _>(rng))
    }
}

impl<L: Data> Dummy<Profession<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Profession<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_PROFESSION.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Profession<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Profession<L>, rng: &mut R) -> Self {
        L::COMPANY_PROFESSION.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<Industry<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Industry<L>, rng: &mut R) -> Self {
        let s = *L::COMPANY_INDUSTRY.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Industry<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Industry<L>, rng: &mut R) -> Self {
        L::COMPANY_INDUSTRY.choose(rng).unwrap()
    }
}
