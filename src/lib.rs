extern crate uom;
pub use uom::si::f64::*;
use uom::si::pressure::pascal;
use uom::si::Quantity;

pub mod gummy_float;

#[warn(missing_docs)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_pressure(pressure: Pressure) {
    println!("{:?}",pressure);
}

// try to have a function which prints any quantity
//
// do we use an enum with all the units?? IDK
// But we can do one with all the types first
pub fn print_quantity(quantity: Pressure) {
    println!("{:?}",quantity);
}



#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Gummy<T: GummyUnit> {
    quantity: T,
    sigma_uncertainty: T
}

impl<T: GummyUnit> Gummy<T> {
    fn new(user_input_quantity: T,
           user_input_sigma_uncertainty: T) -> Self {
        return Self { 
            quantity: user_input_quantity, 
            sigma_uncertainty: user_input_sigma_uncertainty
        }
    }
}

// one can start implement traits for gummy units, such as add
use std::ops::{Add,Sub,Mul};
use uom::num::Float;
impl<T: GummyUnit + Float> Add for Gummy<T> {

    type Output = Self;

    fn add(self, other: Self) -> Self {


        
        let calculated_variance = 
            self.sigma_uncertainty * self.sigma_uncertainty +
            other.sigma_uncertainty * other.sigma_uncertainty;

        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        Self {
            quantity: self.quantity + other.quantity, 
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: GummyUnit + Add<Output = T> + Sub<Output = T>> Sub for Gummy<T> {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            quantity: self.quantity - other.quantity, 
            sigma_uncertainty: self.sigma_uncertainty + other.sigma_uncertainty
        }
    }
}

// I'll use traits to mark which units are okay
pub trait GummyUnit {}

// only allowed units can be put into gummy units
impl GummyUnit for Pressure {}
impl GummyUnit for f64 {}
impl GummyUnit for f32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics_uom() {
        let pressure = Pressure::new::<pascal>(1.0);
        print_pressure(pressure);

        // syntax for new gummy pressure quantity
        let pressure_gummy = 
            Gummy::<Pressure>::new(pressure, pressure);
        let f64_gummy = 
            Gummy::<f64>::new(51.4, 2.0);

        let two_gummy_add = 
            f64_gummy + f64_gummy;

    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
