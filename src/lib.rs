#![feature(associated_type_defaults)]
#![feature(auto_traits)]
#![feature(negative_impls)]

mod macros;
mod type_logic;

pub mod units;

pub use macros::*;

use std::{
    borrow::Cow,
    fmt::Display,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Reprensents a measure unit of a physical magnitude.
pub trait MeasureUnit: Sized {
    /// Represents the Unit type that Self is an alias of.
    /// By default is assigned to Self, effectively indicating that
    /// the current unit is not an alias of any other unit.
    ///
    /// E. g if the current instance represents the unit "Gbps", it
    /// would be an alias of Div<Gigabit, Second>, and therefore
    /// `AliasedUnit = Div<Gigabit, Second>`.
    type AliasedUnit: MeasureUnit = Self;

    /// Returns the symbol that accompanies the value of this unit
    /// when printed, and identifies it.
    fn symbol() -> Cow<'static, str>;
}

/// Trait that defines conversions between measurements of different units.
pub trait FromUnit<U>: Sized {
    fn from_value(input: Measurement<U>) -> Measurement<Self>;
}
/// Marker trait that indicates that a relationship of a unit U with
/// Self is linear.  Its implementation is unsafe because it is
/// responsability of the developer to ensure that the underlying
/// implementation of FromUnit ensures so. (Probably there's a better
/// way to do this, but choosing this one from now as it is simple.)
pub unsafe trait FromUnitLinear<U>: FromUnit<U> {}

/// Represents a complex unit that is composed of a unit divided by another.
/// E. g if the units Kilometer and Hour are already defined, the unit
/// km/h can be defined with the unit `DivUnit<Kilometer, Hour>`.
pub struct DivUnit<N, D> {
    _n: PhantomData<N>,
    _d: PhantomData<D>,
}

/// Represents the value of a physical property, measured using the
/// unit U.
pub struct Measurement<U> {
    _marker: PhantomData<U>,
    value: f64,
}

impl<U: MeasureUnit> std::fmt::Debug for Measurement<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Measurement<{}>", &U::symbol()))
            .field("value", &self.value)
            .finish()
    }
}

impl<U> Copy for Measurement<U> {}

impl<U> Clone for Measurement<U> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker.clone(),
            value: self.value.clone(),
        }
    }
}

impl<U> Default for Measurement<U> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            value: Default::default(),
        }
    }
}

impl<N: MeasureUnit, D: MeasureUnit> MeasureUnit for DivUnit<N, D> {
    fn symbol() -> Cow<'static, str> {
        [&N::symbol(), "/", &D::symbol()].concat().into()
    }
}

impl<N: MeasureUnit, D: MeasureUnit, N1: MeasureUnit, D1: MeasureUnit> FromUnit<DivUnit<N, D>>
    for DivUnit<N1, D1>
where
    N1: FromUnitLinear<N>,
    D1: FromUnitLinear<D>,
{
    fn from_value(input: Measurement<DivUnit<N, D>>) -> Measurement<Self> {
        let n = Measurement::<N>::new(input.value());
        let n1 = N1::from_value(n);
        let div = D1::from_value(Measurement::<D>::new(1.0));
        Measurement::new(n1.value() / div.value())
    }
}

impl<U> Measurement<U> {
    /// Creates a new measurement from the given numerical value.
    pub fn new(value: f64) -> Measurement<U> {
        Self {
            _marker: PhantomData::default(),
            value,
        }
    }

    /// Returns the current numerical value.
    pub fn value(self) -> f64 {
        self.value
    }

    /// Converts the current measurement into the given unit V.
    pub fn into_unit<V: MeasureUnit>(self) -> Measurement<V>
    where
        V::AliasedUnit: FromUnit<U>,
    {
        let value_non_aliased = V::AliasedUnit::from_value(self);
        Measurement::new(value_non_aliased.value())
    }
}

impl<Lhs: MeasureUnit, Rhs> Add<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    type Output = Self;

    fn add(self, rhs: Measurement<Rhs>) -> Self::Output {
        //
        Measurement::new(self.value + rhs.into_unit::<Lhs>().value)
    }
}

impl<Lhs: MeasureUnit, Rhs> AddAssign<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    fn add_assign(&mut self, rhs: Measurement<Rhs>) {
        self.value += rhs.into_unit::<Lhs>().value
    }
}

impl<Lhs: MeasureUnit, Rhs> Sub<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    type Output = Self;

    fn sub(self, rhs: Measurement<Rhs>) -> Self::Output {
        Measurement::new(self.value - rhs.into_unit::<Lhs>().value)
    }
}

impl<Lhs: MeasureUnit, Rhs> SubAssign<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    fn sub_assign(&mut self, rhs: Measurement<Rhs>) {
        self.value -= rhs.into_unit::<Lhs>().value
    }
}

impl<U> Mul<f64> for Measurement<U> {
    type Output = Measurement<U>;

    fn mul(self, rhs: f64) -> Self::Output {
        Measurement::new(self.value * rhs)
    }
}

impl<U> MulAssign<f64> for Measurement<U> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<U> Div<f64> for Measurement<U> {
    type Output = Measurement<U>;

    fn div(self, rhs: f64) -> Self::Output {
        Measurement::new(self.value / rhs)
    }
}

impl<U> DivAssign<f64> for Measurement<U> {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

impl<U> Display for Measurement<U>
where
    U: MeasureUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <f64 as Display>::fmt(&self.value, f)?;
        f.write_str(" ")?;
        f.write_str(&U::symbol())
    }
}

impl<Lhs: MeasureUnit, Rhs> PartialOrd<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    fn partial_cmp(&self, other: &Measurement<Rhs>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.into_unit::<Lhs>().value)
    }
}

impl<Lhs: MeasureUnit, Rhs> PartialEq<Measurement<Rhs>> for Measurement<Lhs>
where
    Lhs::AliasedUnit: FromUnit<Rhs>,
{
    fn eq(&self, other: &Measurement<Rhs>) -> bool {
        self.value == other.into_unit::<Lhs>().value
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        units::{Gbps, Hour, Kbps, Kilobit, Megabit, Second},
        DivUnit, MeasureUnit, Measurement,
    };
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use rand::Rng;

    impl<U: 'static> Arbitrary for Measurement<U> {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Self::new(rand::thread_rng().gen_range(-1000000.0..1000000.0))
        }
    }

    macro_rules! cmp_float {
        ($l:expr, $r: expr) => {
            ((($l) - ($r)).abs() < 0.1)
        };
    }

    #[quickcheck]
    fn test_add_same_unit(value1: Measurement<Hour>, value2: Measurement<Hour>) -> bool {
        let r: Measurement<Hour> = value1 + value2;
        cmp_float!(r.value(), value1.value() + value2.value())
    }

    #[quickcheck]
    fn test_add_different_unit1(value1: Measurement<Hour>, value2: Measurement<Second>) -> bool {
        let r: Measurement<Hour> = value1 + value2;

        cmp_float!(r.value(), value1.value() + value2.value() / 3600.0)
    }

    #[quickcheck]
    fn test_add_different_unit2(value1: Measurement<Hour>, value2: Measurement<Second>) -> bool {
        let r: Measurement<Second> = value2 + value1;

        cmp_float!(r.value(), value1.value() * 3600.0 + value2.value())
    }

    #[quickcheck]
    fn test_add_complex(
        value1: Measurement<DivUnit<Megabit, Hour>>,
        value2: Measurement<DivUnit<Kilobit, Second>>,
    ) -> bool {
        let r: Measurement<DivUnit<Megabit, Hour>> = value1 + value2;
        cmp_float!(
            r.value(),
            value1.value() + value2.value() * 3600.0 / 1_000.0
        )
    }

    #[quickcheck]
    fn test_sub_same_unit(value1: Measurement<Hour>, value2: Measurement<Hour>) -> bool {
        let r: Measurement<Hour> = value1 - value2;
        cmp_float!(r.value(), value1.value() - value2.value())
    }

    #[quickcheck]
    fn test_sub_different_unit1(value1: Measurement<Hour>, value2: Measurement<Second>) -> bool {
        let r: Measurement<Hour> = value1 - value2;

        cmp_float!(r.value(), value1.value() - value2.value() / 3600.0)
    }

    #[quickcheck]
    fn test_sub_different_unit2(value1: Measurement<Hour>, value2: Measurement<Second>) -> bool {
        let r: Measurement<Second> = value2 - value1;
        cmp_float!(r.value(), value2.value() - value1.value() * 3600.0)
    }

    #[quickcheck]
    fn test_sub_complex(
        value1: Measurement<DivUnit<Megabit, Hour>>,
        value2: Measurement<DivUnit<Kilobit, Second>>,
    ) -> bool {
        let r: Measurement<DivUnit<Megabit, Hour>> = value1 - value2;
        cmp_float!(
            r.value(),
            value1.value() - value2.value() * 3600.0 / 1_000.0
        )
    }

    #[test]
    fn test_add_compiles() {
        let m1: Measurement<DivUnit<Kilobit, Second>> = Default::default();
        let m2: Measurement<Kbps> = Default::default();

        let _: Measurement<DivUnit<Kilobit, Second>> = m1 + m1;
        let _: Measurement<DivUnit<Kilobit, Second>> = m1 + m2;
        let _: Measurement<Kbps> = m2 + m2;
        let _: Measurement<Kbps> = m2 + m1;
    }

    #[test]
    fn test_display() {
        let m1: Measurement<Hour> = Measurement::new(42.42);
        assert_eq!("42.42 h", format!("{}", m1));
    }

    #[test]
    fn test_display_complex() {
        let m1: Measurement<DivUnit<Kilobit, Second>> = Measurement::new(42.42);
        assert_eq!("42.42 Kb/s", format!("{}", m1));
    }

    #[test]
    fn test_display_alias() {
        let m1: Measurement<Kbps> = Measurement::new(42.42);
        assert_eq!("42.42 Kbps", format!("{}", m1));
    }

    #[test]
    fn test_sub_compiles() {
        let m1: Measurement<DivUnit<Kilobit, Second>> = Default::default();
        let m2: Measurement<Kbps> = Default::default();

        let _: Measurement<DivUnit<Kilobit, Second>> = m1 - m1;
        let _: Measurement<DivUnit<Kilobit, Second>> = m1 - m2;
        let _: Measurement<Kbps> = m2 - m2;
        let _: Measurement<Kbps> = m2 - m1;
    }

    #[test]
    fn test_into_unit_compiles() {
        let m1: Measurement<Hour> = Default::default();
        let m2: Measurement<DivUnit<Kilobit, Second>> = Default::default();
        let m3: Measurement<Kbps> = Default::default();

        let _: Measurement<Second> = m1.into_unit();
        let _: Measurement<Kbps> = m2.into_unit();
        let _: Measurement<DivUnit<Kilobit, Second>> = m2.into_unit();
        let _: Measurement<DivUnit<Megabit, Hour>> = m3.into_unit();
    }
}
