#![allow(dead_code)]
#![allow(unused_variables)]

use crate::constants::*;
use crate::structs::*;
use std::option::Option;

type Callback = fn();

/// For:
/// 1. generating 1 minute bar data from tick data
/// 2. generateing x minute bar/x hour bar data from 1 minute data
/// Notice:
/// 1. for x minute bar, x must be able to divide 60: 2, 3, 5, 6, 10, 15, 20, 30
/// 2. for x hour bar, x can be any number
pub struct BarGenerator {
    bar: Option<BarData>,
    on_bar: Option<Callback>,
    interval: Interval,
    interval_count: usize,
    window: usize,
    window_bar: Option<BarData>,
    on_window_bar: Option<Callback>,
    last_tick: Option<TickData>,
    last_bar: Option<BarData>,
}

impl Default for BarGenerator {
    fn default() -> BarGenerator {
        BarGenerator {
            bar: None,
            on_bar: None,
            interval: Interval::MINUTE,
            interval_count: 0,
            window: 0,
            window_bar: None,
            on_window_bar: None,
            last_tick: None,
            last_bar: None,
        }
    }
}

impl BarGenerator {
    pub fn new(
        on_bar: Callback,
        window: usize,
        on_window_bar: Callback,
        interval: Interval,
    ) -> BarGenerator {
        BarGenerator {
            on_bar: Some(on_bar),
            interval,
            window,
            on_window_bar: Some(on_window_bar),
            ..BarGenerator::default()
        }
    }
    /// Update new tick data into generator.
    fn update_tick(&mut self, tick: TickData) {
        todo!();
    }
    /// Update 1 minute bar into generator
    fn update_bar(&mut self, tick: BarData) {
        todo!();
    }
}
