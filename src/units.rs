use crate::{define_alias, define_linear_conversions, define_unit, DivUnit};

// Time
define_unit!(Second, "s");
define_unit!(Minute, "min");
define_unit!(Hour, "h");

define_linear_conversions! {
    (Second, 1),
    (Minute, 60),
    (Hour, 3600)
}

// Bits
define_unit!(Bit, "b");
define_unit!(Kilobit, "Kb");
define_unit!(Megabit, "Mb");
define_unit!(Gigabit, "Gb");
define_unit!(Terabit, "Tb");
define_unit!(Petabit, "Pb");
define_unit!(Exabit, "Eb");
define_unit!(Zettabit, "Zb");
define_unit!(Yottabit, "Yb");

// Bytes
define_unit!(Byte, "B");
define_unit!(Kilobyte, "KB");
define_unit!(Megabyte, "MB");
define_unit!(Gigabyte, "GB");
define_unit!(Terabyte, "TB");
define_unit!(Petabyte, "PB");
define_unit!(Exabyte, "EB");
define_unit!(Zettabyte, "ZB");
define_unit!(Yottabyte, "YB");

// Bytes (power of 2)
define_unit!(Kibibyte, "KiB");
define_unit!(Mebibyte, "MiB");
define_unit!(Gibibyte, "GiB");
define_unit!(Tebibyte, "TiB");
define_unit!(Pebibyte, "PiB");
define_unit!(Exbibyte, "EiB");
define_unit!(Zebibyte, "ZiB");
define_unit!(Yobibyte, "YiB");

define_linear_conversions! {
    // Bits
    (Bit     , 1.0),
    (Kilobit , 1_000.0),
    (Megabit , 1_000_000.0),
    (Gigabit , 1_000_000_000.0),
    (Terabit , 1_000_000_000_000.0),
    (Petabit , 1_000_000_000_000_000.0),
    (Exabit  , 1_000_000_000_000_000_000.0),
    (Zettabit, 1_000_000_000_000_000_000_000.0),
    (Yottabit, 1_000_000_000_000_000_000_000_000.0),

    // Bytes
    (Byte     , 8.0),
    (Kilobyte , 8_000.0),
    (Megabyte , 8_000_000.0),
    (Gigabyte , 8_000_000_000.0),
    (Terabyte , 8_000_000_000_000.0),
    (Petabyte , 8_000_000_000_000_000.0),
    (Exabyte  , 8_000_000_000_000_000_000.0),
    (Zettabyte, 8_000_000_000_000_000_000_000.0),
    (Yottabyte, 8_000_000_000_000_000_000_000_000.0),

    // Bytes (power of 2)
    (Kibibyte, 8_192.0),
    (Mebibyte, 8_388_608.0),
    (Gibibyte, 8_589_934_592.0),
    (Tebibyte, 8_796_093_022_208.0),
    (Pebibyte, 9_007_199_254_740_992.0),
    (Exbibyte, 9_223_372_036_854_775_808.0),
    (Zebibyte, 9_444_732_965_739_290_427_392.0),
    (Yobibyte, 9_671_406_556_917_033_397_649_408.0)
}

// Transmission speed
define_alias!(DivUnit<Bit    , Second> as Bps , "bps");
define_alias!(DivUnit<Kilobit, Second> as Kbps, "Kbps");
define_alias!(DivUnit<Megabit, Second> as Mbps, "Mbps");
define_alias!(DivUnit<Gigabit, Second> as Gbps, "Gbps");
define_alias!(DivUnit<Terabit, Second> as Tbps, "Tbps");
