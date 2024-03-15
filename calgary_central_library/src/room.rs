use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

fn stringly_capacity(haystack: &str) -> Option<&str> {
    // TODO: consider using aho-corasick for this
    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"accommodate up to \S+ people").unwrap());
    static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"It has a capacity of \S+").unwrap());
    RE1.find(haystack)
        .map(|m| {
            m.as_str()
                .strip_prefix("accommodate up to ")
                .unwrap()
                .strip_suffix(" people")
                .unwrap()
        })
        .or(RE2
            .find(haystack)
            .map(|m| m.as_str().strip_prefix("It has a capacity of ").unwrap()))
}

// TODO: add historical capacity data
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum KnownRoom {
    // 2-05A Meeting Room
    R205AMeetingRoom,
    // 2-05B Meeting Room
    R205BMeetingRoom,
    // 2-05C Meeting Room
    R205CMeetingRoom,
    // 2-06A Terentiuk Space for Adult Learning
    R206ATerentiukSpaceForAdultLearning,
    // 2-06B Millar Family Learning and Discovery Room
    R206BMillarFamilyLearningAndDiscoveryRoom,
    // 3-20C Meeting Room
    R320CMeetingRoom,
    // 3-20G Meeting Room
    R320GMeetingRoom,
    // 3-20H Meeting Room
    R320HMeetingRoom,
    // 3-10A Meeting Room
    R310AMeetingRoom,
    // 3-10B Meeting Room
    R310BMeetingRoom,
    // 3-17A Meeting Room
    R317AMeetingRoom,
    // 3-17B Field Law Meeting Room
    R317BFieldLawMeetingRoom,
    // 3-19C Meeting Room
    R319CMeetingRoom,
    // 3-20A Idea Lab
    R320AIdeaLab,
    // 3-16B
    R316B,
}

/// Either a specific room or an unknown room.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RoomChoice {
    KnownRoom(KnownRoom),
    UnknownRoom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    choice: RoomChoice,
    title: String,
    description: String,
    inferred_capacity: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct TimeSlot(u8);

#[derive(Serialize, Deserialize)]
pub struct Availability(Vec<TimeSlot>);

impl RoomChoice {
    pub(crate) fn from_title(title: impl AsRef<str>) -> Self {
        use KnownRoom::*;

        let known_room = match title.as_ref() {
            "2-05A Meeting Room" => R205AMeetingRoom,
            "2-05B Meeting Room" => R205BMeetingRoom,
            "2-05C Meeting Room" => R205CMeetingRoom,
            "2-06A Terentiuk Space for Adult Learning" => R206ATerentiukSpaceForAdultLearning,
            "2-06B Millar Family Learning and Discovery Room" => {
                R206BMillarFamilyLearningAndDiscoveryRoom
            }
            "3-20C Meeting Room" => R320CMeetingRoom,
            "3-20G Meeting Room" => R320GMeetingRoom,
            "3-20H Meeting Room" => R320HMeetingRoom,
            "3-10A Meeting Room" => R310AMeetingRoom,
            "3-10B Meeting Room" => R310BMeetingRoom,
            "3-17A Meeting Room" => R317AMeetingRoom,
            "3-17B Field Law Meeting Room" => R317BFieldLawMeetingRoom,
            "3-19C Meeting Room" => R319CMeetingRoom,
            "3-20A Idea Lab" => R320AIdeaLab,
            "3-16B" => R316B,
            _ => return RoomChoice::UnknownRoom,
        };
        RoomChoice::KnownRoom(known_room)
    }
}

impl TimeSlot {
    fn add_signed_hours(orig: u8, delta: i8) -> u8 {
        let mut new = (orig as i8) + delta;
        if new < 0 {
            new += 24;
        }
        (new % 24) as u8
    }

    pub(crate) fn from_label(label: impl AsRef<str>) -> Option<Self> {
        let label: &str = label.as_ref();
        if label.starts_with("Booked") {
            return None;
        }
        let mut it = label.split(":");
        let hour = it.next()?.parse::<u8>().ok()?;
        let mut it = it.next()?.split(" ");
        let min = it.next()?.parse::<u8>().ok()?;
        assert!(min == 0 || min == 30);
        let am_pm = it.next()?;
        // convert to 24 hour time
        let hour = match am_pm {
            "AM" | "am" => hour,
            "PM" | "pm" => hour + 12,
            _ => return None,
        };
        let time_slot = Self::add_signed_hours(hour, -5) * 2 + min / 30;
        Some(Self(time_slot))
    }

    pub(crate) fn to_label(&self) -> String {
        let hour = Self::add_signed_hours(self.0 / 2, 5);
        let min = if self.0 % 2 == 0 { "00" } else { "30" };
        let am_pm = if hour < 12 { "AM" } else { "PM" };
        let hour = if hour == 0 { 12 } else { hour % 12 };
        format!("{}:{} {}", hour, min, am_pm)
    }

    pub(crate) fn to_discriminant(&self) -> u8 {
        self.0
    }
}

impl Room {
    fn infer_capacity_from_description(description: impl AsRef<str>) -> Option<u8> {
        let description: &str = description.as_ref();
        let capacity = stringly_capacity(description)?;
        match capacity {
            "ten" => Some(10),
            "six" => Some(6),
            "four" => Some(4),
            number if number.chars().all(|c| c.is_ascii_digit()) => {
                Some(number.parse::<u8>().unwrap())
            }
            other => {
                println!("Couldn't infer capacity from {:?}", other);
                None
            }
        }
    }
    pub(crate) fn new(choice: RoomChoice, title: String, description: String) -> Self {
        let inferred_capacity = Self::infer_capacity_from_description(description.as_str());
        Self {
            choice,
            title,
            description,
            inferred_capacity,
        }
    }
}

impl<It> From<It> for Availability
where
    It: IntoIterator<Item = String>,
{
    fn from(it: It) -> Self {
        let mut time_slots = Vec::new();
        for time_slot in it.into_iter().filter_map(|s| TimeSlot::from_label(s)) {
            time_slots.push(time_slot);
        }
        Self(time_slots)
    }
}

impl std::fmt::Display for Availability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "Fully booked");
        };
        let mut it = self.0.iter();
        write!(f, "[")?;
        if let Some(time_slot) = it.next() {
            write!(f, "{}", time_slot.to_label())?;
            for time_slot in it {
                write!(f, ", {}", time_slot.to_label())?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_signed_hours_forward() {
        let before = 0;
        let after = TimeSlot::add_signed_hours(before, 1);
        assert_eq!(after, 1);
    }

    #[test]
    fn check_add_signed_hours_forward_overflow() {
        let before = 23;
        let after = TimeSlot::add_signed_hours(before, 1);
        assert_eq!(after, 0);
    }

    #[test]
    fn check_add_signed_hours_backward() {
        let before = 1;
        let after = TimeSlot::add_signed_hours(before, -1);
        assert_eq!(after, 0);
    }

    #[test]
    fn check_add_signed_hours_backward_overflow() {
        let before = 0;
        let after = TimeSlot::add_signed_hours(before, -1);
        assert_eq!(after, 23);
    }

    #[test]
    fn five_am_is_the_origin() {
        let time_slot = TimeSlot::from_label("5:00 AM").unwrap();
        assert_eq!(time_slot.to_label(), "5:00 AM");
        assert_eq!(time_slot.to_discriminant(), 0);
    }

    #[test]
    fn check_timeslot() {
        let time_slot_a = TimeSlot::from_label("10:00 AM").unwrap();
        assert_eq!(time_slot_a.to_label(), "10:00 AM");
        let time_slot_b = TimeSlot::from_label("10:30 AM").unwrap();
        assert_eq!(time_slot_b.to_label(), "10:30 AM");
        assert!(time_slot_a < time_slot_b);
        let discr_diff = time_slot_b.to_discriminant() - time_slot_a.to_discriminant();
        assert_eq!(discr_diff, 1);
    }
}
