
use statrs::distribution::{ContinuousCDF, Normal};

fn main() {

    let a = OptionContract{
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.4,
    };
    let call = a.calc_call_option();
    let put = a.calc_put_option();

    println!("The price of a call option on the stock is: {}",call);
    println!("The price of a put option on the stock is: {}",put);




}

struct OptionContract {
    underlying: f64,
    strike: f64,
    rate: f64,
    time: f64,
    volatility: f64,
}
impl OptionContract {
    fn calc_d1(&self) -> f64 {
        let numerator_1 = (self.underlying / self.strike).ln();
        let numerator_2 = (self.rate + (self.volatility.powf(2.0)/2.0))* self.time;
        let denominator = self.volatility * self.time.sqrt();

        let d1 = (numerator_1 + numerator_2) / denominator;
        d1
    }
    fn calc_d2(&self) -> f64 {
        let d1 = self.calc_d1();
        let d2 = d1 - (self.volatility * self.time.sqrt());

        d2
    }
    fn calc_call_option(&self) -> f64 {
        println!("Calculating call option price ...");
        let normal = Normal::new(0.0, 1.0).unwrap();

        let d1 = self.calc_d1();
        let d2 = self.calc_d2();

        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        let call_option_price = self.underlying * nd1 - (self.strike * (-self.rate * self.time).exp() * nd2);
        call_option_price
    }
    fn calc_put_option(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();

        let d1 = self.calc_d1();
        let d2 = self.calc_d2();

        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);

        let put_option_price = (self.strike * (-self.rate * self.time).exp() * nd2) -  self.underlying * nd1;
        put_option_price
    }

}
/*
fn calc_d1(s: f64, k: f64, r:f64, t: f64,vol:f64) -> f64 {
    let ln_s_k = (s/k).ln();
    let part_two = (r+(vol.powi(2)/2.0)) * t;
    let divisor = vol * (t.sqrt());

    let d1 = (ln_s_k/part_two)/divisor;
    return d1

}
fn calc_d2(d1:f64,vol:f64,t:f64) -> f64 {
    let d2 = d1 - (vol * t.sqrt());
    return d2
}
fn calc_option(s:f64, d1:f64,d2:f64,k:f64,r:f64,t:f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let nd1 = normal.cdf(d1);
    let nd2 = normal.cdf(d2);

    let option_price = s * nd1 - (k * (E.powf(-r*t)) * nd2);
    return option_price

}


 */
