use statrs::distribution::{Continuous,ContinuousCDF, Normal};


pub struct OptionContract {
    pub underlying: f64,
    pub strike: f64,
    pub rate: f64,
    pub time: f64,
    pub volatility: f64,
}
impl OptionContract {
    pub fn calc_d_values(&self) -> (f64,f64){
        let numerator_1 = (self.underlying / self.strike).ln();
        let numerator_2 = (self.rate + (self.volatility.powf(2.0)/2.0))* self.time;
        let denominator = self.volatility * self.time.sqrt();

        let d1 = (numerator_1 + numerator_2) / denominator;

        let d2 = d1 - denominator;

        (d1, d2)
    }
    pub fn calc_call_option(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();

        let (d1, d2) = self.calc_d_values();

        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        let call_option_price = self.underlying * nd1 - (self.strike * (-self.rate * self.time).exp() * nd2);
        call_option_price
    }
    pub fn calc_put_option(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();

        let (d1, d2) = self.calc_d_values();

        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);

        let put_option_price = (self.strike * (-self.rate * self.time).exp() * nd2) -  self.underlying * nd1;
        put_option_price
    }
    //calculating the greeks
    pub fn calc_call_delta(&self) -> f64 {
        let d1 = self.calc_d_values().0;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let delta = normal.cdf(d1);
        delta
    }
    pub fn calc_put_delta(&self) -> f64 {
        let d1 = self.calc_d_values().0;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let delta = normal.cdf(d1) -1.0;
        delta
    }
    pub fn calc_gamma(&self) -> f64 {
        let d1 = self.calc_d_values().0;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let numerator = normal.pdf(d1);
        let denominator = self.underlying * self.volatility * self.time.sqrt();

        let gamma = numerator/denominator;
        gamma
    }
    pub fn calc_vega (&self) -> f64 {
        let d1 = self.calc_d_values().0;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let vega = self.underlying * (normal.pdf(d1)) * self.time.sqrt();
        vega
    }
    pub fn calc_call_theta(&self) -> f64 {
        let (d1,d2) = self.calc_d_values();
        let normal = Normal::new(0.0, 1.0).unwrap();

        let front_equation = (self.underlying * normal.pdf(d1) * self.volatility)/ (2.0 * self.time.sqrt());
        let back_equation = self.rate * self.strike * (-self.rate*self.time).exp() * normal.cdf(d2);



        let call_theta = -front_equation - back_equation;
        call_theta
    }
    pub fn calc_put_theta(&self) -> f64 {
        let (d1,d2) = self.calc_d_values();
        let normal = Normal::new(0.0, 1.0).unwrap();

        let front_equation = (self.underlying * normal.pdf(d1) * self.volatility)/ (2.0 * self.time.sqrt());
        let back_equation = self.rate * self.strike * (-self.rate*self.time).exp() * normal.cdf(-d2);

        let put_theta = -front_equation + back_equation;
        put_theta
    }
    pub fn calc_call_rho (&self) -> f64 {
        let d2 = self.calc_d_values().1;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let call_rho = self.strike * self.time * (-self.rate*self.time).exp() * normal.cdf(d2);
        call_rho
    }
    pub fn calc_put_rho(&self) -> f64 {
        let d2 = self.calc_d_values().1;
        let normal = Normal::new(0.0, 1.0).unwrap();

        let put_rho = -self.strike * self.time * (-self.rate*self.time).exp() * normal.cdf(-d2);
        put_rho
    }
}

pub fn check_datatype(input: &str) -> Result<f64, std::num::ParseFloatError> {

    let number: f64 = match input.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(number)
}