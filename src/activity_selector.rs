//! This module contains the solution to finding a maximum-size set
//! of mutual exclusive activities.

/// The type `Activity` owns the start time and end time of of an activity.
#[derive(Debug)]
pub struct Activity {
    start: usize,
    end: usize,
}

impl Activity {
    /// Create a new activity.
    pub fn new(start: usize, end: usize) -> Self {
        Activity { start, end }
    }

    /// Create a list of activities from the start list and end list.
    /// It assumes that the start and end time at a given index are for the same activity.
    pub fn from_list(starts: Vec<usize>, ends: Vec<usize>) -> Vec<Self> {
        let activities: Vec<Activity> = starts
            .into_iter()
            .zip(ends.into_iter())
            .map(|(s, e)| Activity::new(s, e))
            .collect();
        activities
    }

    /// Find the maximum set time for a scheduling activities.
    /// It assumes that the activities are sorted in  monotonic and increasing order.
    pub fn maximum_set(activities: &[Activity], position: usize) -> Vec<&Activity> {
        fn recursive<'a>(
            activities: &'a [Activity],
            position: usize,
            result: &mut Vec<&'a Activity>,
        ) {
            let mut next = position + 1;

            while next < position && activities[next].start < activities[position].end {
                next += 1;
            }

            if next < activities.len() {
                result.push(&activities[next]);
                recursive(activities, next, result);
            }
        }

        let mut result = Vec::new();
        recursive(activities, position, &mut result);
        return result;
    }
}
