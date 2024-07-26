//! A helper module for processing geo coordinates in problem and solution.

#[cfg(test)]
#[path = "../../tests/unit/format/coord_index_test.rs"]
mod coord_index_test;

use crate::format::problem::{Problem, VehicleBreak};
use crate::format::{CustomLocationType, Location};
use std::cmp::Ordering::Less;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// A helper struct which keeps track of coordinate mapping.
pub struct CoordIndex {
    direct_index: HashMap<Location, usize>,
    reverse_index: HashMap<usize, Location>,
    custom_locations: HashSet<Location>,
    max_matrix_index: usize,
    flags: u8,
}

impl CoordIndex {
    /// Creates a new instance of `CoordIndex`.
    pub fn new(problem: &Problem) -> Self {
        let mut index = Self {
            direct_index: Default::default(),
            reverse_index: Default::default(),
            custom_locations: Default::default(),
            max_matrix_index: 0,
            flags: 0,
        };

        // process plan
        problem.plan.jobs.iter().for_each(|job| {
            job.pickups
                .iter()
                .chain(job.deliveries.iter())
                .chain(job.replacements.iter())
                .chain(job.services.iter())
                .flat_map(|tasks| tasks.iter().flat_map(|task| task.places.iter()))
                .for_each(|place| {
                    index.add(&place.location);
                });
        });

        // process fleet
        problem.fleet.vehicles.iter().for_each(|vehicle| {
            vehicle.shifts.iter().for_each(|shift| {
                index.add(&shift.start.location);

                if let Some(end) = &shift.end {
                    index.add(&end.location);
                }

                if let Some(breaks) = &shift.breaks {
                    breaks
                        .iter()
                        .filter_map(|vehicle_break| match vehicle_break {
                            VehicleBreak::Optional { places, .. } => Some(places),
                            VehicleBreak::Required { .. } => None,
                        })
                        .flat_map(|places| places.iter())
                        .filter_map(|place| place.location.as_ref())
                        .for_each(|location| index.add(location));
                }

                if let Some(reloads) = &shift.reloads {
                    reloads.iter().for_each(|reload| index.add(&reload.location));
                }

                if let Some(recharges) = &shift.recharges {
                    recharges.stations.iter().for_each(|station| index.add(&station.location));
                }
            });
        });

        index.max_matrix_index = index.direct_index.len().max(1) - 1;

        let start_offset = index.direct_index.len() * index.direct_index.len();
        // NOTE promote custom locations to the index to use usize outside
        index.custom_locations.iter().enumerate().for_each(|(offset, location)| {
            debug_assert!(matches!(location, Location::Custom { .. }));

            let value = start_offset + offset;
            index.direct_index.insert(location.clone(), value);
            index.reverse_index.insert(value, location.clone());
        });

        index
    }

    /// Adds location to indices.
    pub(crate) fn add(&mut self, location: &Location) {
        if !self.direct_index.contains_key(location) {
            let value = match location {
                Location::Coordinate { lat: _, lng: _ } => {
                    self.flags |= 0b0001;
                    self.direct_index.len()
                }
                Location::Reference { index } => {
                    self.flags |= 0b0010;
                    *index
                }
                Location::Custom { r#type: CustomLocationType::Unknown } => {
                    self.flags |= 0b0100;
                    // NOTE do not add custom location in the index yet
                    self.custom_locations.insert(location.clone());
                    return;
                }
            };

            self.direct_index.insert(location.clone(), value);
            self.reverse_index.insert(value, location.clone());
        }
    }

    /// Gets index of location.
    pub fn get_by_loc(&self, location: &Location) -> Option<usize> {
        self.direct_index.get(location).cloned()
    }

    /// Get location by index.
    pub fn get_by_idx(&self, index: usize) -> Option<Location> {
        self.reverse_index.get(&index).cloned()
    }

    /// Gets unique locations.
    pub fn unique(&self) -> Vec<Location> {
        let mut sorted_pairs: Vec<_> = self.reverse_index.iter().collect();
        sorted_pairs.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Less));
        sorted_pairs.iter().map(|pair| pair.1.clone()).collect()
    }

    /// Checks whether given id belongs to special (custom) location range.
    pub(crate) fn is_special_index(&self, index: usize) -> bool {
        let start = (self.max_matrix_index + 1).pow(2);
        let end = start + self.custom_locations_len();

        (start..end).contains(&index)
    }

    /// Max location index in matrix.
    pub(crate) fn max_matrix_index(&self) -> usize {
        self.max_matrix_index
    }

    /// Returns size of custom locations index.
    pub(crate) fn custom_locations_len(&self) -> usize {
        self.custom_locations.len()
    }

    /// Returns true if problem has coordinates.
    pub fn has_coordinates(&self) -> bool {
        (self.flags & 0b0001) > 0
    }

    /// Returns true if problem has indices.
    pub fn has_indices(&self) -> bool {
        (self.flags & 0b0010) > 0
    }

    /// Returns true if problem has custom locations.
    pub fn has_custom(&self) -> bool {
        (self.flags & 0b0100) > 0
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Location::Coordinate { lat: l_lat, lng: l_lng }, Location::Coordinate { lat: r_lat, lng: r_lng }) => {
                (l_lat - r_lat).abs() < f64::EPSILON && (l_lng - r_lng).abs() < f64::EPSILON
            }
            (Location::Reference { index: left }, Location::Reference { index: right }) => left == right,
            (
                Location::Custom { r#type: CustomLocationType::Unknown },
                Location::Custom { r#type: CustomLocationType::Unknown },
            ) => true,
            _ => false,
        }
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Location::Coordinate { lat, lng } => {
                state.write_u64(lat.to_bits());
                state.write_u64(lng.to_bits());
            }
            Location::Reference { index } => {
                state.write_usize(*index);
            }
            Location::Custom { r#type: CustomLocationType::Unknown } => {
                state.write_usize(0);
            }
        }
    }
}
