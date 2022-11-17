mod qubit;
use qubit::Qubit;
use std::f64;

fn main() {
    for _n in 0..1000 {
        let mut q = qubit::Qubit {
            theta: std::f64::consts::PI / 2.0,
            phi: 0.0,
        };
        let res = qubit::Qubit::measure(&mut q);
        print!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_qubit() {
        let qubit: Qubit = Default::default();
        assert_eq!(qubit.theta, 0.0);
        assert_eq!(qubit.phi, 0.0);
    }

    #[test]
    fn test_neasure_default_qubit() {
        let mut qubit: Qubit = Default::default();

        let want = 0;
        let got = Qubit::measure(&mut qubit);
        assert_eq!(got, want);
    }

    #[test]
    fn test_measure_configured_as_one_qubit() {
        let mut qubit = Qubit {
            theta: f64::consts::PI,
            phi: 0.0,
        };

        let want = 1;
        let got = Qubit::measure(&mut qubit);
        assert_eq!(got, want);
    }

    #[test]
    fn test_collapes_of_state() {
        let mut qubit = Qubit {
            theta: f64::consts::PI / 2.0,
            phi: 0.0,
        };

        let want = Qubit::measure(&mut qubit);

        for _n in 0..100 {
            let got = Qubit::measure(&mut qubit);
            assert_eq!(got, want);
        }
    }
}
