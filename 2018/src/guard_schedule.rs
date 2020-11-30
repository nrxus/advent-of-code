use std::{collections::HashMap, convert::TryFrom, num::ParseIntError};

pub struct Schedule {
    pub minutes: [GuardState; 60],
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            minutes: [GuardState::Awake; 60],
        }
    }

    pub fn guard_schedules(entries: &[Entry]) -> Result<HashMap<u16, Vec<Self>>, EntryParseError> {
        let mut id = if let Entry::BeginShift(_, id) = entries[0] {
            Ok(id)
        } else {
            Err(EntryParseError)
        }?;
        let mut schedule = Schedule::new();
        let mut guards: HashMap<_, Vec<_>> = HashMap::new();

        for entry in entries[1..].iter() {
            match entry {
                Entry::BeginShift(_, next_id) => {
                    guards.entry(id).or_default().push(schedule);

                    id = *next_id;
                    schedule = Schedule::new();
                }
                Entry::FallingAsleep(d) => {
                    let minute = d[14..16].parse()?;
                    schedule.set_after(minute, GuardState::Asleep);
                }
                Entry::WakingUp(d) => {
                    let minute = d[14..16].parse()?;
                    schedule.set_after(minute, GuardState::Awake);
                }
            }
        }

        guards.entry(id).or_default().push(schedule);
        Ok(guards)
    }

    pub fn asleep_minutes(&self) -> impl Iterator<Item = usize> + '_ {
        self.minutes
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == GuardState::Asleep)
            .map(|(m, _)| m)
    }

    fn set_after(&mut self, minute: usize, state: GuardState) {
        self.minutes[minute..].iter_mut().for_each(|m| {
            *m = state;
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuardState {
    Awake,
    Asleep,
}

impl<'s> TryFrom<&'s str> for Entry<'s> {
    type Error = EntryParseError;

    fn try_from(raw: &'s str) -> Result<Self, Self::Error> {
        let date = &raw[1..17];
        let action = &raw[19..];
        if action.contains("falls asleep") {
            Ok(Entry::FallingAsleep(date))
        } else if action.contains("wakes up") {
            Ok(Entry::WakingUp(date))
        } else {
            let id = action[7..]
                .split(' ')
                .nth(0)
                .ok_or(EntryParseError)?
                .parse::<u16>()?;
            Ok(Entry::BeginShift(date, id))
        }
    }
}

#[derive(Debug)]
pub struct EntryParseError;

impl From<ParseIntError> for EntryParseError {
    fn from(_: ParseIntError) -> Self {
        EntryParseError
    }
}

#[derive(Debug)]
pub enum Entry<'s> {
    BeginShift(&'s str, u16),
    FallingAsleep(&'s str),
    WakingUp(&'s str),
}

impl Entry<'_> {
    pub fn date(&self) -> &str {
        match self {
            Entry::BeginShift(d, _) => &d,
            Entry::FallingAsleep(d) => &d,
            Entry::WakingUp(d) => &d,
        }
    }
}
