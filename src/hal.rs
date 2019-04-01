// Copyright (c) 2017-2019 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Miscellaneous `embedded-hal` trait implementations.
//!
//! The `hal` module consists of a collection of `embedded-hal` trait
//! implementations for traits that aren't tied to a specific peripheral.
//!
//! This module is only included when either the `hal` or `hal-unproven` feature
//! flag is enabled.

use std::thread;
use std::time::{Duration, Instant};

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::CountDown;
use void::Void;

/// Implements the `embedded-hal` `DelayMs` and `DelayUs` traits.
#[derive(Debug, Default)]
pub struct Delay {}

impl Delay {
    /// Constructs a new `Delay`.
    pub fn new() -> Delay {
        Delay {}
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        thread::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        thread::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        thread::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u64> for Delay {
    fn delay_ms(&mut self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        thread::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        thread::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        thread::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u64> for Delay {
    fn delay_us(&mut self, us: u64) {
        thread::sleep(Duration::from_micros(us));
    }
}

/// Implements the `embedded-hal` `CountDown` trait.
#[derive(Debug, Default, Copy, Clone)]
pub struct Timer {
    now: Instant,
    duration: Duration,
}

impl Timer {
    /// Constructs a new `Timer`.
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            duration: Duration::from_micros(0),
        }
    }
}

pub struct Millisecond(pub u64);
pub struct Microsecond(pub u64);
pub struct Second(pub u64);

impl From<Millisecond> for Microsecond {
    fn from(item: Millisecond) -> Self {
        Microsecond(item.0 * 1_000)
    }
}

impl From<Second> for Microsecond {
    fn from(item: Second) -> Self {
        Microsecond(item.0 * 1_000_000)
    }
}

impl Microsecond {
    fn as_u64(&self) -> u64 {
        let &Microsecond(t) = self;
        t
    }
}

impl CountDown for Timer {
    type Time = Microsecond;

    /// Start the timer with a `timeout`
    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Microsecond>,
    {
        self.duration = Duration::from_micros(timeout.into().as_u64());
        self.now = Instant::now();
    }

    /// Return `Ok` if the timer has wrapped
    /// Automatically clears the flag and restarts the time
    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.now.elapsed() >= self.duration {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}