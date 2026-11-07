use option_pricer::OptionContract;

#[test]
fn test_d1() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.4,
    };

    let actual = option.calc_d1();

    let expected = 1.018741295447258;

    assert_eq!(actual, expected);
}
#[test]
fn test_d2() {
    let option = OptionContract {
        underlying: 100.0,
        strike: 80.0,
        rate: 0.05,
        time: 0.5,
        volatility: 0.05,
    };

    let actual = option.calc_d2();

    let expected =0.735898582972639;

    assert_eq!(actual, expected);
}
