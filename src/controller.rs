// Main file for obstacle avoidance

const MINSPEED = 1e-6;
const FLOATEQUAL = 1e-6;

struct DynamicController{
    _damping: f64;
}

struct Controller{
    _damping: f64;
    _basis: f64;
    _damping_eigenval: f64;
    control_output_: f64;
}

impl Controller {
    fn compute_control(&self) -> f64 {
        0.0
    }

    fn update_damping(&mut self) -> f64 {
        0.0
    }
}




