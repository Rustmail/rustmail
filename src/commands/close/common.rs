use std::time::Duration;

pub fn parse_duration_spec(spec: &str) -> Option<Duration> {
    if spec.is_empty() {
        return None;
    }
    let mut total: u64 = 0;
    let mut num: u64 = 0;
    let mut has_unit_segment = false;
    for ch in spec.chars() {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10)? as u64;
            num = num.saturating_mul(10).saturating_add(digit);
        } else {
            let unit_secs = match ch {
                's' | 'S' => 1,
                'm' | 'M' => 60,
                'h' | 'H' => 3600,
                'd' | 'D' => 86400,
                _ => return None,
            };
            total = total.saturating_add(num.saturating_mul(unit_secs));
            num = 0;
            has_unit_segment = true;
        }
    }
    if num > 0 {
        if has_unit_segment {
            total = total.saturating_add(num);
        } else {
            total = total.saturating_add(num * 60);
        }
    }
    if total == 0 {
        None
    } else {
        Some(Duration::from_secs(total))
    }
}
