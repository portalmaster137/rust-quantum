use rand::Rng;

//Qubit is a struct that represents a qubit. It has a state, which is a complex number, and a name, which is a string.
pub struct Qubit {
    pub theta: f64,
    pub phi: f64,
}

impl Default for Qubit {
    fn default() -> Self {
        return Qubit {
            theta: 0.0,
            phi: 0.0,
        };
    }
}

impl Qubit {
    pub fn measure(qubit: &mut Qubit) -> i32 {
        let mut rng = rand::thread_rng();
        let rn = rng.gen::<f64>();
        if rn < (qubit.theta / 2.0).cos().powf(2.0) {
            qubit.theta = 0.0;
            return 0;
        }
        qubit.theta = std::f64::consts::PI;
        return 1;
    }
}
