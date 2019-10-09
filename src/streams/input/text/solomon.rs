#[cfg(test)]
#[path = "../../../../tests/unit/streams/input/text/solomon_tests.rs"]
mod solomon_tests;

use crate::models::common::{Dimensions, Location, TimeWindow};
use crate::models::problem::{Costs, Driver, Fleet, Job, Jobs, Place, Single, TransportCost, Vehicle};
use crate::models::Problem;
use crate::utils::TryCollect;
use std::fs::read;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::sync::Arc;

pub fn parse_solomon_format<R: Read>(mut reader: BufReader<R>) -> Result<Problem, String> {
    SolomonReader { buffer: String::new(), reader, matrix: Matrix::new() }.read_problem()
}

struct SolomonReader<R: Read> {
    buffer: String,
    reader: BufReader<R>,
    matrix: Matrix,
}

struct VehicleLine {
    number: usize,
    capacity: usize,
}

struct JobLine {
    id: usize,
    x: usize,
    y: usize,
    demand: usize,
    start: usize,
    end: usize,
    service: usize,
}

impl<R: Read> SolomonReader<R> {
    pub fn read_problem(&mut self) -> Result<Problem, String> {
        let fleet = self.read_fleet()?;
        let jobs = self.read_jobs(&fleet);

        unimplemented!()
    }

    fn read_fleet(&mut self) -> Result<Fleet, String> {
        self.skip_lines(4)?;
        let vehicle = self.read_vehicle()?;
        self.skip_lines(4)?;
        let depot = self.read_customer()?;

        Ok(Fleet::new(
            vec![Driver {
                costs: Costs {
                    fixed: 0.0,
                    per_distance: 0.0,
                    per_driving_time: 0.0,
                    per_waiting_time: 0.0,
                    per_service_time: 0.0,
                },
                dimens: create_dimens_with_id("driver".to_string()),
                details: Default::default(),
            }],
            (0..vehicle.number)
                .map(|i| Vehicle {
                    profile: 0,
                    costs: Costs {
                        fixed: 0.0,
                        per_distance: 1.0,
                        per_driving_time: 0.0,
                        per_waiting_time: 0.0,
                        per_service_time: 0.0,
                    },
                    dimens: create_dimens_with_id(["v".to_string(), i.to_string()].concat()),
                    details: vec![],
                })
                .collect(),
        ))
    }

    fn read_jobs(&mut self, fleet: &Fleet) -> Result<Vec<Arc<Job>>, String> {
        let mut jobs: Vec<Arc<Job>> = Default::default();
        let mut i: usize = 1;
        loop {
            match self.read_customer() {
                Ok(customer) => {
                    jobs.push(Arc::new(Job::Single(Arc::new(Single {
                        places: vec![Place {
                            location: Some(self.matrix.location((customer.x, customer.y))),
                            duration: customer.service as f64,
                            times: vec![TimeWindow { start: customer.start as f64, end: customer.end as f64 }],
                        }],
                        dimens: create_dimens_with_id(["c".to_string(), i.to_string()].concat()),
                    }))));
                    i = i + 1;
                }
                Err(error) => {
                    if self.buffer.is_empty() {
                        break;
                    } else {
                        Err(error)?;
                    }
                }
            }
        }

        Ok(jobs)
    }

    fn read_vehicle(&mut self) -> Result<VehicleLine, String> {
        self.read_line()?;
        let (number, capacity) = self
            .buffer
            .split_whitespace()
            .map(|line| line.parse::<usize>().unwrap())
            .try_collect()
            .ok_or("Cannot parse vehicle number or/and capacity".to_string())?;

        Ok(VehicleLine { number, capacity })
    }

    fn read_customer(&mut self) -> Result<JobLine, String> {
        self.read_line()?;
        let (id, x, y, demand, start, end, service) = self
            .buffer
            .split_whitespace()
            .map(|line| line.parse::<usize>().unwrap())
            .try_collect()
            .ok_or("Cannot read depot line".to_string())?;
        Ok(JobLine { id, x, y, demand, start, end, service })
    }

    fn skip_lines(&mut self, count: usize) -> Result<(), String> {
        for i in 0..count {
            self.read_line().map_err(|_| "Cannot skip lines")?;
        }

        Ok(())
    }

    fn read_line(&mut self) -> Result<usize, String> {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).map_err(|err| err.to_string())
    }
}

fn create_dimens_with_id(id: String) -> Dimensions {
    let mut dimens = Dimensions::new();
    dimens.insert("id".to_string(), Box::new(id));
    dimens
}

struct Matrix {
    locations: Vec<(usize, usize)>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix { locations: vec![] }
    }

    fn location(&mut self, location: (usize, usize)) -> Location {
        0
    }
}
