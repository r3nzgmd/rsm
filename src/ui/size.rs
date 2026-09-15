// helper functions for parsing bytes into kib/mib/gib/tib
pub fn parse_full(bytes: f64) -> String {
    let mut usage = bytes;
    let mut unit = String::from("B");

    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("KiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("MiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("GiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("TiB"); }

    format!("{:.2} {}", usage, unit)
}

pub fn parse_full_one_decimal(bytes: f64) -> String {
    let mut usage = bytes;
    let mut unit = String::from("B");

    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("KiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("MiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("GiB"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("TiB"); }

    format!("{:.1} {}", usage, unit)
}

pub fn parse_short(bytes: f64) -> String {
    let mut usage = bytes;
    let mut unit = String::from("B");

    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("K"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("M"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("G"); }
    if usage >= 1024.0 { usage /= 1024.0; unit = String::from("T"); }

    format!("{:.1}{}", usage, unit)
}
