//! Lockstep is an application for daily timemanagement (may expand to weekly, monthly, and yearly
//! later)
//!
//! Lockstep follows 1 rule, 1 active task per timeslot. A timeslot is a unit of time given to complete a
//! task. Tasks must fit within the given interval a timeslot takes up, otherwise, your task is too
//! big and needs to be divided or reduced in scope.
//! 
//! An active task is a task intended to be worked for the duration of time in a slot.
//! Lockstep manages timeslots based on the interval set for each timeslot, and divides 24 hours
//! into a whole number of slots.
//!
//! By default, a Timeset has slots filled with tasks in the Idle state. User defined tasks will
//! automatically be either in a Scheduled state (in the future), Active state (in the present), 
//! or Expired state (in the past).  

// use core::panic;
// use clap::Parser;

mod timeset;
mod task;
mod timesetmgr;
mod taskmgr;
use chrono::NaiveDate;

fn main() {
   // let timeset_instance = timeset::Timeset::default();
   // let tmstmgr = timesetmgr::TimesetMgr::default(); 
   println!("Hello!");
}

