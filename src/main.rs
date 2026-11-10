
use option_pricer::OptionContract;
use option_pricer::read_number;

fn main() {

    let underlying_price = read_number("Enter the underlying asset price:");
    let strike_price = read_number("Enter the option's strike price:");
    let rf_rate = read_number("Enter the risk-free rate:");
    let exp_time = read_number("Enter the option's time to expiration:");
    let implied_volatility = read_number("Enter the option's implied volatility:");

    let a = match OptionContract::new(underlying_price,strike_price,rf_rate,exp_time,implied_volatility){
        Ok(contract) => contract,
        Err(error) => {
            println!("Invalid option contract: {}",error);
            return;
        }
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

