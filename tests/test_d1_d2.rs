use option_pricer::OptionContract;

#[test]
fn test_d1_d2() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.4,
    };

    let (actual_d1,actual_d2) = option.calc_d_values();
    let expected_d1 = 
}
#[test]
fn test_d2() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.,
    };

    let actual = option.calc_d2();

    let expected =0.735898582972639;

    assert_eq!(actual, expected);
}
