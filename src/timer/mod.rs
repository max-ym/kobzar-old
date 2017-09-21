
/// Time split into hours, minutes, seconds and nanos.
pub struct TimeSplit {
    hours   : u32,
    minutes : u8,
    seconds : u8,
    nanos   : u32,
}

impl TimeSplit {

    /// Create new TimeSplit. All overflows are corrected automatically.
    pub fn new(hours: u32, minutes: u32, seconds: u32, nanos: u32) -> Self {
        let nanos_overflow  = nanos / 1000_000_000;
        let nanos           = nanos % 1000_000_000;

        let seconds = seconds + nanos_overflow;

        let seconds_overflow    = seconds / 60;
        let seconds             = seconds % 60;

        let minutes = minutes + seconds_overflow;

        let minutes_overflow    = minutes / 60;
        let minutes             = minutes % 60;

        let hours = hours + minutes_overflow;

        TimeSplit {
            hours   : hours     as u32,
            minutes : minutes   as u8,
            seconds : seconds   as u8,
            nanos   : nanos     as u32,
        }
    }

    /// Get nanoseconds.
    pub fn nanos(&self) -> u32 {
        self.nanos
    }

    /// Get seconds.
    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    /// Get minutes.
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    /// Get hours.
    pub fn hours(&self) -> u32 {
        self.hours
    }
}

/// Any structure that implements this trait contains time value.
pub trait Time {

    /// Count of nanoseconds. Remainder from full time divided by 1000_000_000.
    fn nanos(&self) -> u32;

    /// Count of full microseconds.
    /// Remainder from full time divided by 1000_000.
    fn micros(&self) -> u32 {
        self.nanos() / 1000
    }

    /// Count of full milliseconds.
    /// Remainder from full time divided by 1000.
    fn millis(&self) -> u32 {
        self.nanos() / 1000_000
    }

    /// Count of full seconds.
    fn seconds(&self) -> u32;

    /// Count of full minutes.
    fn minutes(&self) -> u32 {
        self.seconds() / 60
    }

    /// Count of full hours.
    fn hours(&self) -> u32 {
        self.seconds() / 60 / 60
    }

    /// Split this time into hours, minutes, seconds and nanoseconds that
    /// do not overflow.
    fn split(&self) -> TimeSplit {
        // All overflows are automatically corrected by constructor.
        TimeSplit::new(0, 0, self.seconds(), self.nanos())
    }
}

/// System timer that can run specified functions when time events occur.
pub trait Timer {

    /// Time type that this timer can use.
    type T : Time;

    /// Set callback function which will be called when specified
    /// time goes out.
    fn callback_on_timeout(&mut self, time: Self::T, callback: &Fn());
}
