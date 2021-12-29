/// Defines a non-linear relationship between two units, using the
/// given closures for computing the relationship between them.
#[macro_export]
macro_rules! define_nonlinear_conversion {
    ($from_unit:ident -> $to_unit:ident, |$arg: ident| $expr:expr) => {
	impl FromUnit<$from_unit> for $to_unit {
	    fn from_value(input: Measurement<$from_unit>) -> Measurement<Self> {
		fn do_conversion($arg: f64) -> f64 {
		    $expr
		}

		Measurement::new(do_conversion(input.value()))
	    }
	}
    };

    ($first_unit:ident <> $last_unit:ident,
     |$first_arg:ident| $first_expr:expr,
     |$last_arg:ident| $last_expr:expr) => {
	$crate::define_nonlinear_conversion!($first_unit -> $last_unit, |$first_arg| $first_expr);
	$crate::define_nonlinear_conversion!($last_unit -> $first_unit, |$last_arg| $last_expr);
    };

}

/// Defines a unit, given a name and its symbol.
#[macro_export]
macro_rules! define_unit {
    ($id:ident, $symbol:literal) => {
        pub struct $id;
        impl $crate::MeasureUnit for $id {
            fn symbol() -> std::borrow::Cow<'static, str> {
                $symbol.into()
            }
        }
    };
}

/// Defines a alias unit, that holds its own symbol and it is
/// equivalent to another unit.
#[macro_export]
macro_rules! define_alias {
    ($unit:ty as $aliasunit:ident, $symbol:literal) => {
        pub struct $aliasunit;
        impl $crate::MeasureUnit for $aliasunit {
            type AliasedUnit = $unit;

            fn symbol() -> std::borrow::Cow<'static, str> {
                $symbol.into()
            }
        }

        impl<T> $crate::FromUnit<$aliasunit> for T
        where
            T: $crate::FromUnit<$unit>,
        {
            fn from_value(input: $crate::Measurement<$aliasunit>) -> $crate::Measurement<Self> {
                T::from_value($crate::Measurement::<$unit>::new(input.value()))
            }
        }
    };
}

/// Defines the conversions of a set of units whose relationship is linear between them, given a multiply factor.
#[macro_export]
macro_rules! define_linear_conversions {
    (@impl_from_unit from:($lunit:ident, $lmul:expr), to:($runit:ident, $rmul:expr)) => {
	unsafe impl $crate::FromUnitLinear<$lunit> for $runit {}

	impl $crate::FromUnit<$lunit> for $runit {
	    fn from_value(input: $crate::Measurement<$lunit>) -> $crate::Measurement<Self> {
		$crate::Measurement::new(input.value() * (($lmul) as f64) / (($rmul) as f64))
	    }
	}
    };

    (@cartesian_product ($lunit:ident, $lmul:expr); $(($runit:ident, $rmul:expr))*) => {
	$(
	    $crate::define_linear_conversions!(@impl_from_unit from: ($lunit, $lmul), to: ($runit, $rmul));
	)*
    };

    (@cartesian_product ($lheadunit:ident, $lheadmul:expr) $(($lunit:ident, $lmul:expr))*; $(($runit:ident, $rmul:expr))*) => {
	$crate::define_linear_conversions!(@cartesian_product ($lheadunit, $lheadmul); $(($runit, $rmul))*);
	$crate::define_linear_conversions!(@cartesian_product $(($lunit, $lmul))*; $(($runit, $rmul))*);

    };

    ($(($unit:ident, $mul:literal)),*) => {
	$crate::define_linear_conversions!(@cartesian_product $(($unit, $mul))*; $(($unit, $mul))*);
    };
}
