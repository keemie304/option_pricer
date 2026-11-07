use option_pricer::OptionContract;

#[test]
fn test_calc_call_option() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.4,
    };
    let actual = option.calc_call_option();

    let expected =24.57454590260616;

    assert_eq!(actual, expected);
}
#[test]
fn test_calc_put_option() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.4,
    };
    let actual = option.calc_put_option();

    let expected =2.599338864872788;

    assert_eq!(actual, expected);
}
