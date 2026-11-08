

use option_pricer::OptionContract;

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

    let d1 = a.calc_d_values().0;
    println!("D1 is {:.3}",d1);
    let d2 = a.calc_d_values().1;
    println!("D2 is {:.3}",d2);


    println!("The price of a call option on the stock is: {:.3}",call);
    println!("The price of a put option on the stock is: {:.3}",put);

    println!("Calculating the Greeks...");

    let call_delta = a.calc_call_delta();
    let put_delta = a.calc_put_delta();
    let gamma = a.calc_gamma();
    let vega = a.calc_vega();
    let call_theta =  a.calc_call_theta();
    let put_theta = a.calc_put_theta();
    let call_rho = a.calc_call_rho();
    let put_rho = a.calc_put_rho();

    println!("The greeks are:\n
    Call Delta: {:.3}\n
    Put Delta: {:.3}\n
    Gamma: {:.3}\n
    Vega: {:.3}\n
    Call Theta: {:.3}\n
    Put Theta: {:.3}\n
    Call Rho: {:.3}\n
    Put Rho: {:.3}", call_delta, put_delta, gamma, vega, call_theta,put_theta,call_rho,put_rho);

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
