use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::result::Result;
use std::vec::Vec;

use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn manhattan(c1: &Self, c2: &Self) -> isize {
        (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
    }
}

fn read_sensors(filename: &str) -> MyResult<Vec<(Coord, Coord)>> {
    let mut l: Vec<(Coord, Coord)> = Vec::new();
    let reader = io::BufReader::new(File::open(filename)?);
    let expr = Regex::new(r"-?\d+").unwrap();
    for line in reader.lines() {
        let nums: Vec<isize> = expr.find_iter(line?.as_ref())
                                   .map(|m| isize::from_str_radix(m.as_str(), 10))
                                   .collect::<Result<Vec<isize>, ParseIntError>>()?;
        if nums.len() != 4 {
            return Err("Invalid input line, need 4 integers".into());
        }
        l.push((Coord{x: nums[0], y: nums[1]}, Coord{x: nums[2], y: nums[3]}));
    }
    Ok(l)
}

/**
 * The RangeSet stores a set of integers, using a sorted list of ranges.
 *
 * There are a few obvious ways to store a set of integers:
 *
 * 1. An array of booleans. This requires an explicit, smallish domain of
 *    integers -- since you can't have an infinite array, and allocating one
 *    which has INT_MAX entries is probably prohibitive, depending on your
 *    integer type. The array also requires storage size O(N) where N is size of
 *    the domain of your set.  It features constant time membership testing,
 *    but adding elements to the set is O(M), where M is the number of elements
 *    you'd like to add.
 * 2. A hash table. This is a decent general purpose solution, assuming a decent
 *    hash function. The memory size is no longer limited to the domain of your
 *    integers, but it is still O(M), as is the process of adding those M
 *    elements to the set.
 * 3. A sorted list. This is also a pretty good solution, but again, O(M) for
 *    the size of your elements, and of course now the membership testing is
 *    O(log M).
 *
 * If your M is large because of lots of contiguous ranges, you can instead use
 * a sorted list of ranges! This could also probably be represented as a binary
 * tree (most sorted arrays could be). But tree balancing is a drag. So let's
 * use the array. If we define K as the number of ranges that are members, then
 * your storage size is O(K) and lookup time is O(log K).
 *
 * So that's what this data structure is. Create a set, and add ranges. Ranges
 * are automatically merged as you create them. You can also invert the set, and
 * iterate over all elements in the set (or, as a result, the inverted set).
 *
 * I haven't implemented removal of elements (which is usually the tricky part
 * for this sort of data structure, as it can create holes). I also don't
 * actually have an explicit membership testing function, though it would be
 * trivial.
 */
struct RangeSet {
    list: Vec<(isize, isize)>,
}

impl RangeSet {
    /**
     * Create a new ranged set.
     */
    fn new() -> Self {
        RangeSet{list: Vec::new()}
    }
    /**
     * Add the range [start, end] -- INCLUSIVE on both ends -- to the set.
     */
    fn add(&mut self, start: isize, end: isize) {
        /* Let i be the first index where start <= t.0 */
        let mut i = self.list.partition_point(|t| t.0 < start);

        if i > 0 && self.list[i - 1].1 >= start {
            /* We overlap with the left-hand range. Reuse it. We'll need to
             * adjust its end, but we would be doing that anyway since there's
             * the possibility of a merge. */
            i -= 1;
        } else if i < self.list.len() && self.list[i].0 <= end {
            /* We overlap with the right-hand range. Reuse that. Since we
             * don't overlap on the left, just reset the start of that range and
             * continue to the merging.  */
            self.list[i].0 = start;
        } else {
            /* No overlap. We have to insert a new range, and there will not be
             * a merge necessary. */
            self.list.insert(i, (start, end));
            return;
        }

        /* Now, we are guaranteed that at index i, we have a range starting at
         * or before start. The end of the range at index i could be anything. */
        if end <= self.list[i].1 {
            /* The range at index i continues past our end, so we don't need to
             * do any follow up. */
            return;
        }
        /* We're extending the end of this range, now go ahead and delete/merge
         * any subsequent one. */
        self.list[i].1 = end;
        while i + 1 < self.list.len() {
            if self.list[i].0 > end {
                /* We can't merge with this one, we're done! */
                return;
            }
            /* We overlap with the beginning of this range, what about the end? */
            if end <= self.list[i + 1].1 {
                /* We this range extends beyond ours, copy its end, and delete it. */
                self.list[i].1 = self.list[i + 1].1;
                self.list.remove(i + 1);
                /* The precondition of this function is that no ranges overlap.
                 * Thus, no range after this overlapped, and we can return. */
                return;
            }
            /* We completely contain this range, delete it and check the next. */
            self.list.remove(i + 1);
        }
        /* Hm, we've reached the end of the array. Our range end is already set,
         * I guess we're all done! */
    }
    /**
     * Return a vector of ranges of members in the range [start, end]. You can
     * easily iterate over this by doing:
     *
     * for range in self.get_ranges_between(start, end) {
     *     for element in self.range.clone() {
     *         do_something_with(element);
     *     }
     * }
     */
    fn get_ranges_between(&self, start: isize, end: isize) -> Vec<RangeInclusive<isize>> {
        /* Let i be the first index where start <= t.0 */
        let i = self.list.partition_point(|t| t.0 < start);

        /* We will start from here and add each range. */
        let mut l: Vec<RangeInclusive<isize>> = Vec::new();

        if i > 0 && self.list[i - 1].1 >= start {
            l.push(start ..= cmp::min(end, self.list[i - 1].1))
        }
        /* At this point, start is after any previous range, and i, if it points
         * at anything, points at the next range which might intersect.
         */
        for i in i..self.list.len() {
            if self.list[i].0 > end {
                /* We moved past the end of the user's range, no more counting. */
                break;
            }
            l.push(self.list[i].0 ..= cmp::min(self.list[i].1, end));
        }
        l
    }
    /** Return the number of members in the range [start, end] */
    fn count(&self, start: isize, end: isize) -> isize {
        let mut amount = 0;
        for range in self.get_ranges_between(start, end).iter() {
            amount += *range.end() - *range.start() + 1;
        }
        return amount;
    }
    /** Return the total number of members in the set */
    fn count_all(&self) -> isize {
        if self.list.len() == 0 {
            0
        } else {
            self.count(self.list[0].0, self.list[self.list.len() - 1].1)
        }
    }
    /** Return all ranges in the set */
    fn get_ranges(&self) -> Vec<RangeInclusive<isize>> {
        if self.list.len() == 0 {
            Vec::new()
        } else {
            self.get_ranges_between(self.list[0].0, self.list[self.list.len() - 1].1)
        }
    }
    /** Return an inverted set: every member of self is not a member of the
     * inverted set, and vice versa. */
    fn invert(&self, start: isize, end: isize) -> Self {
        let mut l = Self::new();
        let mut prev_edge = start;
        for range in self.get_ranges_between(start, end).iter() {
            if *range.start() > prev_edge {
                l.add(prev_edge, *range.start() - 1);
            }
            prev_edge = *range.end() + 1;
        }
        if prev_edge <= end {
            l.add(prev_edge, end);
        }
        l
    }
}

/**
 * The actual implementation logic of part 1 / part 2.
 *
 * Given "sensors", which is a list of sensor / beacon pairs, and yline, a y
 * index to walk along, return a ranged set of X indices which are IMPOSSIBLE to
 * contain a beacon, because a sensor would have detected it.
 *
 * include_beacon is a flag, which indicates whether the beacons themselves
 * should be marked as IMPOSSIBLE. For part 1 of the problem, it it should be
 * false, because technically... a beacon does exist there, so it's not
 * impossible for there to be a beacon. It's just not the emergency beacon.
 *
 * For part 2, it is true, because we only want to find the emergency beacon.
 */
fn get_impossible_ranges(sensors: &Vec<(Coord, Coord)>, yline: isize, include_beacon: bool) -> RangeSet {
    let mut no_beacons = RangeSet::new();
    for (sensor, beacon) in sensors.iter() {
        let manhattan = Coord::manhattan(sensor, beacon);
        let diff = (sensor.y - yline).abs();
        if diff <= manhattan {
            let rem = manhattan - diff;
            let mut start = sensor.x - rem;
            let mut end = sensor.x +  rem;
            if beacon.y == yline && !include_beacon {
                if beacon.x == start {
                    start += 1;
                } else if beacon.x == end {
                    end -= 1;
                } else {
                    panic!("Should be impossible.");
                }
            }
            if start <= end {
                no_beacons.add(start, end);
            }
        }
    }
    no_beacons
}

fn main() {
    let sensors = read_sensors("input.txt").unwrap();
    const YLINE: isize = 2000000;

    // part 1
    let rangelist = get_impossible_ranges(&sensors, YLINE, false);
    println!("Total number of impossible ranges on y={} is: {}", YLINE, rangelist.count_all());

    // part 2
    for y in 0..=4000000 {
        let rangelist = get_impossible_ranges(&sensors, y, true);
        let count = rangelist.count(0, 4000000);
        if count != 4000001 {
            let inverted = rangelist.invert(0, 4000000);
            for range in inverted.get_ranges().iter() {
                for x in range.clone() {
                    let tuning = x * 4000000 + y;
                    println!("The location is ({}, {})", x, y);
                    println!("The tuning frequency is {}", tuning);
                }
            }
        }
    }
}
