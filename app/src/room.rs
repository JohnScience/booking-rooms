struct RoomInfo {
    name: String,
    description: String,
    inferred_capacity: Option<u8>,
}

pub(crate) enum RoomChoice {
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
    UnknownRoom,
}

struct Room {
    info: RoomInfo,
    choice: RoomChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TimeSlot(u8);

pub(crate) struct Availability(Vec<TimeSlot>);

impl RoomChoice {
    pub(crate) fn try_from_title(title: impl AsRef<str>) -> Option<Self> {
        use RoomChoice::*;

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

            _ => return None,
        };
        Some(known_room)
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
