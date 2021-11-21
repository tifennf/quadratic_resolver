use std::env;

fn main() {
    let args: Vec<f64> = env::args()
        .skip(1)
        .map(|arg| arg.trim().parse::<f64>().unwrap())
        .collect();

    let a = args[0];
    let b = args[1];
    let c = args[2];

    let polynome = SecondPolynome::new(a, b, c);
    let delta = Delta::new(&polynome);

    let roots = two_root(&delta, &polynome);

    if polynome.is_two_roots_valid(roots) {
        println!("Solution for {}x^2 + {}x + {} = 0", &a, &b, &c);
        println!("Delta: {}", delta.0);
        println!("Roots:  x1 = {} ; x2 = {}", roots.0, roots.1);
    }
}

fn two_root(delta: &Delta, polynome: &SecondPolynome) -> (f64, f64) {
    if !delta.is_positive() {
        panic!("delta does not have 2 roots");
    }

    let divider = 2.0 * polynome.a;
    let delta_sqrt = delta.sqrt();

    let x1_numerator = -polynome.b - delta_sqrt;
    let x1 = x1_numerator / divider;

    let x2_numerator = -polynome.b + delta_sqrt;

    let x2 = x2_numerator / divider;

    (x1, x2)
}

struct SecondPolynome {
    a: f64,
    b: f64,
    c: f64,
}

impl SecondPolynome {
    fn new(a: f64, b: f64, c: f64) -> Self {
        SecondPolynome { a, b, c }
    }

    fn is_two_roots_valid(&self, roots: (f64, f64)) -> bool {
        let Self { a, b, c } = self;

        let (x1, x2) = roots;

        let a1 = a * x1 * x1;
        let b1 = b * x1;

        let first_solution = a1 + b1 + c;

        let a = a * x2 * x2;
        let b = b * x2;

        let second_solution = a + b + c;

        first_solution == 0.0 && second_solution == 0.0
    }
}

struct Delta(f64);

impl Delta {
    fn sqrt(&self) -> f64 {
        libm::sqrt(self.0)
    }

    fn is_positive(&self) -> bool {
        self.0 > 0.0
    }

    fn is_equal_zero(&self) -> bool {
        self.0 == 0.0
    }

    fn is_negative(&self) -> bool {
        self.0 < 0.0
    }

    fn new(polynome: &SecondPolynome) -> Self {
        let SecondPolynome { a, b, c } = polynome;

        Self(b * b - 4.0 * a * c)
    }
}
