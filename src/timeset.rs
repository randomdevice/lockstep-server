
use std::fmt::{Display, Formatter, Result};
use uuid::Uuid;
use chrono::prelude::*;
use std::cmp::Ordering;

use crate::task::Task;

/// A struct representing a [Timeset]
///  
/// A [Timeset] manages timeslots (referred to as slots) for a given day. The number of slots is based off of the
/// interval of the timeset for each slot. The interval splits 24 hours in a day into a set number
/// of slots.
#[derive(Debug, Eq)]
pub struct Timeset {

    /// unique identifier for this Timeset
    pub uuid: Uuid,

    /// Associated date this Timeset manages
    pub date: NaiveDate,

    /// the interval of time a timeslot represents (10 min, 30 min, 60 min, 120 min)
    pub interval: u64,            
    
    /// the number of slots this timeset manages
    pub n_slots: u64,

    /// a vector of size n_slots representing the tasks assigned to the current timeset
    _slots: Vec<Task>,

    // the index of the current active slot
    _active_slot_idx: u64,

}


impl PartialEq for Timeset {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl PartialOrd for Timeset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl Ord for Timeset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Display for Timeset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[  Timeset  ]\n")?;
        write!(f, "uuid: {}\n", self.uuid)?;
        write!(f, "date: {}\n", self.date)?;
        write!(f, "interval: {}\n", self.interval)?;
        write!(f, "slots: {}\n", self.n_slots)?;
        Ok(())
    }
}

impl Default for Timeset {

   /// Default implementation of Timeset
   /// Sets a Timeset to have 24 slots per day
   /// Each slot is 60 mins long
   fn default() -> Self {
       Self {
            uuid: Uuid::now_v7(),
            date: Local::now().date_naive(),
            interval: 60,
            n_slots: 24,
            _slots: vec![Task::default();24],
            _active_slot_idx: 0,
       }
   } 

}

impl Timeset {

    pub fn new(date: NaiveDate) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            date,
            interval: 60,
            n_slots: 24,
            _slots: vec![Task::default();24],
            _active_slot_idx: 0,
        }
    }

    /// Sets the current time interval managed by the existing Timeset
    pub fn set_interval(&mut self, interval: u64) {

        match interval {
            120 => self.n_slots = 12,
            90 => self.n_slots = 16,
            60 => self.n_slots = 1 * 24,
            30 => self.n_slots = 2 * 24,
            20 => self.n_slots = 3 * 24,
            15 => self.n_slots = 4 * 24,
            12 => self.n_slots = 5 * 24,
            10 => self.n_slots = 6 * 24,
            _ => panic!("Invalid interval, must be an increment within an hour, 60, 30, 20, 15, 12, or 10")
        }

        self.interval = interval;

    }
}
