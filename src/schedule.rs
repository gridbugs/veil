use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

pub type ScheduleTicket = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleEntry<T> {
    // time at which entry will be removed
    pub release_time: u64,

    // monotonically increasing unique id
    pub ticket: ScheduleTicket,

    // data stored in entry
    pub value: T,

    // time this value will remain in the schedule
    pub duration: u64,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
enum Status {
    Inserted,
    Removed,
}

pub struct Schedule<T> {
    // id of next entry
    next_ticket: ScheduleTicket,

    // time at which last entry was removed
    absolute_time: u64,

    // heap of entries
    entries: BinaryHeap<ScheduleEntry<T>>,

    // tracks which entries are present, and
    // which are removed
    entry_status: HashMap<ScheduleTicket, Status>,
}

impl<T> Schedule<T> {
    pub fn new() -> Self {
        Schedule {
            next_ticket: 0,
            absolute_time: 0,
            entries: BinaryHeap::new(),
            entry_status: HashMap::new(),
        }
    }

    pub fn absolute_time(&self) -> u64 {
        self.absolute_time
    }

    fn next_ticket(&mut self) -> ScheduleTicket {
        let ticket = self.next_ticket;
        self.next_ticket += 1;
        ticket
    }

    pub fn insert(&mut self, value: T, duration: u64) -> ScheduleTicket {
        let ticket = self.next_ticket();

        let entry = ScheduleEntry {
            release_time: self.absolute_time + duration,
            ticket: ticket,
            value: value,
            duration: duration,
        };

        self.entries.push(entry);
        self.entry_status.insert(ticket, Status::Inserted);

        ticket
    }

    pub fn remove(&mut self, ticket: ScheduleTicket) -> bool {
        if let Some(status) = self.entry_status.get_mut(&ticket) {
            if *status == Status::Removed {
                return false;
            }

            *status = Status::Removed;

            return true;
        }
        false
    }

    pub fn next(&mut self) -> Option<ScheduleEntry<T>> {
        while let Some(entry) = self.entries.pop() {
            if let Some(status) = self.entry_status.remove(&entry.ticket) {
                if status == Status::Removed {
                    // ignore entries that were removed
                    continue;
                }
            } else {
                panic!("Missing status for schedule entry {}", entry.ticket);
            }

            // if we're here, it means the entry wasn't removed

            self.absolute_time = entry.release_time;
            return Some(entry);
        }

        assert!(self.entry_status.is_empty(), "Inconsistent entry heap and status map");

        None
    }

    pub fn next_value(&mut self) -> Option<T> {
        self.next().map(|e| e.value)
    }
}

impl<T> Ord for ScheduleEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let abs_time_ord = other.release_time.cmp(&self.release_time);
        if abs_time_ord == Ordering::Equal {
            other.ticket.cmp(&self.ticket)
        } else {
            abs_time_ord
        }
    }
}

impl<T> PartialOrd for ScheduleEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for ScheduleEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ticket == other.ticket
    }
}

impl<T> Eq for ScheduleEntry<T> {}
