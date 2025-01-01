use core::panic;
use std::cell::RefCell;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Display, Result};
use std::usize;
use chrono::prelude::*;
use chrono::NaiveDate;

use crate::timeset::Timeset;

/// Maintains Timesets registered under Lockstep
/// Timesets are organized under a BST with the location of the root maintained under root_idx.
/// The BST requires entries to be unique, which is tracked by the timeset_freqmap.
pub struct TimesetMgr {
    current_time: Option<NaiveTime>,
    timestamp: u64,
    //timesets: Vec<&'a Timeset>,
    freqmap: HashMap<NaiveDate, usize>,
    ordered_timesets: BinaryHeap<Timeset>,
    timesets: BTreeMap<NaiveDate,Timeset>
}

impl TimesetMgr {

    fn new(&mut self) {
        todo!()
    }

    fn load(_filename: String) {
        todo!()
    }

    fn store(_filename: String) {
        todo!()
    }

    fn swap(array: &mut Vec<Timeset>, lidx: usize, ridx: usize) {
        if lidx != ridx {
            array.swap(lidx, ridx);
        }
    }

    pub fn add_timeset(&mut self, tmst: Timeset) -> Result { 
        if self.freqmap.contains_key(&tmst.date) {
            //panic!("Failed to add timeset");
            Err(std::fmt::Error)
        } else {
            self.timeset.insert(tmst.date, tmst);
            self.freqmap.insert(tmst.date,1);
            Ok(())
        }
    }

    pub fn delete_timeset(&mut self, date: NaiveDate) {
    }
}

impl Default for TimesetMgr {
   fn default() -> Self {
      Self {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        current_time: Some(Local::now().time()),
        freqmap: HashMap::new(),
        ordered_timesets: BinaryHeap::new(),
        timesets: BinaryMap::new()
      } 
   } 
}

impl Display for TimesetMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[ timestamp: {} ]", self.timestamp )?;
        writeln!(f, "[ current_time: {} ]", self.current_time.unwrap() )?;
        writeln!(f, "[ uuid ][ date ][ interval ]")?;
        for tmst in self.ordered_timesets.iter() {
            writeln!(f, "{} | {} | {}", tmst.uuid, tmst.date, tmst.interval)?;
        }
        Ok(()) 
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Result;

    use crate::timeset::Timeset;

    use super::NaiveDate;
    use super::TimesetMgr;

    #[test]
    fn test_date_cmp() {

        // Compare early date with later date
        let date1 = NaiveDate::from_ymd_opt(2022, 12, 30);
        let date2 = NaiveDate::from_ymd_opt(2024, 12, 31);
        assert!(date1 < date2);
        
        // Compare later date with early date 
        let date2 = NaiveDate::from_ymd_opt(2022, 8, 9);
        assert!(date1 > date2);

        // Test if dates are the same
        let date2 = NaiveDate::from_ymd_opt(2022, 12, 30);
        assert_eq!(date1, date2);
    }

    #[test]
    fn test_timesetmgr_insert() -> Result {
        let mut mgr: TimesetMgr = TimesetMgr::default();
        let tmst1 = Timeset::new(NaiveDate::from_ymd_opt(2015, 4, 20).unwrap());
        let tmst2 = Timeset::new(NaiveDate::from_ymd_opt(2015, 4, 28).unwrap());
        let tmst3 = Timeset::new(NaiveDate::from_ymd_opt(2020, 4, 20).unwrap());
        mgr.add_timeset(tmst1)?;
        mgr.add_timeset(tmst3)?;
        mgr.add_timeset(tmst2)?;
        println!("{}", mgr);
        //assert_eq!(mgr.timesets.len(), 3);
        Ok(())
    }
}

