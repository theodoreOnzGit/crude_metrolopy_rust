use uom::num::Float;


#[derive(Debug, Clone, Copy, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct GummyFloat<T: Float> {
    quantity: T,
    sigma_uncertainty: T
}

impl<T: Float> GummyFloat<T> {
    fn new(user_input_quantity: T,
           user_input_sigma_uncertainty: T) -> Self {
        return Self { 
            quantity: user_input_quantity, 
            sigma_uncertainty: user_input_sigma_uncertainty
        }
    }
}
#[cfg(test)]
mod std_ops {
    use super::*;

    #[test]
    fn test_add() {
        todo!();
    }

    #[test]
    fn test_add_assign() {
        todo!();
    }

    #[test]
    fn test_sub() {
        todo!();
    }

    #[test]
    fn test_sub_assign() {
        todo!();
    }

    #[test]
    fn test_mul() {
        todo!();
    }

    #[test]
    fn test_mul_assign() {
        todo!();
    }

    #[test]
    fn test_div() {
        todo!();
    }

    #[test]
    fn test_div_assign() {
        todo!();
    }
}

use std::ops::*;
impl<T: Float> Add for GummyFloat<T> {

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

impl<T: Float> AddAssign for GummyFloat<T> {

    fn add_assign(&mut self, other: Self) {


        
        let calculated_variance = 
            self.sigma_uncertainty * self.sigma_uncertainty +
            other.sigma_uncertainty * other.sigma_uncertainty;

        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        *self = Self {
            quantity: self.quantity + other.quantity, 
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> Sub for GummyFloat<T> {

    type Output = Self;

    fn sub(self, other: Self) -> Self {


        
        let calculated_variance = 
            self.sigma_uncertainty * self.sigma_uncertainty +
            other.sigma_uncertainty * other.sigma_uncertainty;

        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        Self {
            quantity: self.quantity - other.quantity, 
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> SubAssign for GummyFloat<T> {

    fn sub_assign(&mut self, other: Self) {


        
        let calculated_variance = 
            self.sigma_uncertainty * self.sigma_uncertainty +
            other.sigma_uncertainty * other.sigma_uncertainty;

        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        *self = Self {
            quantity: self.quantity - other.quantity, 
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> Div for GummyFloat<T> {

    type Output = Self;

    fn div(self, other: Self) -> Self {

        let calculated_quantity = 
            self.quantity / other.quantity;
        
        let calculated_variance = 
            calculated_quantity*calculated_quantity * (
            (self.sigma_uncertainty / self.quantity)* 
            (self.sigma_uncertainty / self.quantity) +
            (other.sigma_uncertainty / other.quantity)* 
            (other.sigma_uncertainty / other.quantity));



        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        Self {
            quantity: calculated_quantity,
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> DivAssign for GummyFloat<T> {

    fn div_assign(&mut self, other: Self) {


        
        let calculated_quantity = 
            self.quantity / other.quantity;
        
        let calculated_variance = 
            calculated_quantity*calculated_quantity * (
            (self.sigma_uncertainty / self.quantity)* 
            (self.sigma_uncertainty / self.quantity) +
            (other.sigma_uncertainty / other.quantity)* 
            (other.sigma_uncertainty / other.quantity));



        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        *self = Self {
            quantity: calculated_quantity,
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> Mul for GummyFloat<T> {

    type Output = Self;

    fn mul(self, other: Self) -> Self {

        let calculated_quantity = 
            self.quantity * other.quantity;
        
        let calculated_variance = 
            calculated_quantity*calculated_quantity * (
            (self.sigma_uncertainty / self.quantity)* 
            (self.sigma_uncertainty / self.quantity) +
            (other.sigma_uncertainty / other.quantity)* 
            (other.sigma_uncertainty / other.quantity));



        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        Self {
            quantity: calculated_quantity,
            sigma_uncertainty: calculated_sigma,
        }
    }
}

impl<T: Float> MulAssign for GummyFloat<T> {

    fn mul_assign(&mut self, other: Self) {


        
        let calculated_quantity = 
            self.quantity * other.quantity;
        
        let calculated_variance = 
            calculated_quantity*calculated_quantity * (
            (self.sigma_uncertainty / self.quantity)* 
            (self.sigma_uncertainty / self.quantity) +
            (other.sigma_uncertainty / other.quantity)* 
            (other.sigma_uncertainty / other.quantity));



        // hard to do this for quantity eh...
        // typing kind of makes it hard to do sqrt...
        let calculated_sigma = 
            calculated_variance.sqrt();

        *self = Self {
            quantity: calculated_quantity,
            sigma_uncertainty: calculated_sigma,
        }
    }
}
