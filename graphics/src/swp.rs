//! Simple SWP parser
//!
//! SWP is a curve format created for a 6.837 assignment

use crate::curve::Curve;

pub fn from_str(raw_swp: &str) -> Vec<dyn Curve> {
    for line in raw_swp.lines() {
        for element in line.split(" ") {
            match element {
                "bez2" => {}
                _ => {}
            }
        }
    }
}
