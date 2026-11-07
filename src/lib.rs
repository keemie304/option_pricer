use statrs::distribution::{ContinuousCDF, Normal};


pub struct OptionContract {
    pub underlying: f64,
    pub strike: f64,
    pub rate: f64,
    pub time: f64,
    pub volatility: f64,
}
impl OptionContract {
    pub fn calc_d1(&self) -> f64 {
        let numerator_1 = (self.underlying / self.strike).ln();
        let numerator_2 = (self.rate + (self.volatility.powf(2.0)/2.0))* self.time;
        let denominator = self.volatility * self.time.sqrt();

        let d1 = (numerator_1 + numerator_2) / denominator;
        d1
    }
    pub fn calc_d2(&self) -> f64 {
        let d1 = self.calc_d1();
        let d2 = d1 - (self.volatility * self.time.sqrt());

        d2
    }
    pub fn calc_call_option(&self) -> f64 {
        println!("Calculating call option price ...");
        let normal = Normal::new(0.0, 1.0).unwrap();

        let d1 = self.calc_d1();
        let d2 = self.calc_d2();

        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        let call_option_price = self.underlying * nd1 - (self.strike * (-self.rate * self.time).exp() * nd2);
        call_option_price
    }
    pub fn calc_put_option(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();

        let d1 = self.calc_d1();
        let d2 = self.calc_d2();

        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);

        let put_option_price = (self.strike * (-self.rate * self.time).exp() * nd2) -  self.underlying * nd1;
        put_option_price
    }
}