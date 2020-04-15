use crate::util::error::Error;

#[macro_use]
pub mod error;
#[macro_use]
pub mod logging;

pub type HikaruResult<T> = Result<T, Error>;

const TIME_PERIOD_TABLE: [u64; 5] = [ 1000 * 60 * 60 * 24 * 365, 1000 * 60 * 60 * 24, 1000 * 60 * 60, 1000 * 60, 1000 ];
const TIME_PERIOD_LABELS: [&str; 5] = [ "year", "d", "h", "min", "s" ];

pub fn format_time_period(period: u64) -> String {
    for (index, it) in TIME_PERIOD_TABLE.iter().enumerate() {
        if &period > it {
            return format!("{}{}", period / it, TIME_PERIOD_LABELS[index]);
        }
    }
    return format!("{}ms", period);
}