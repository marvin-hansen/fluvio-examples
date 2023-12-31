use crate::types::data_types::data_bar::DataBar;
use std::fmt;
use std::fmt::Display;

impl Display for DataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataTime: {},\n Symbol {},\n Open {},\n High {},\n Low {},\n Close {},\n Volume {}",
            self.date_time, self.symbol, self.open, self.high, self.low, self.close, self.volume
        )
    }
}
