
pub struct DiophantineEq {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub x: i32,
    pub y: i32,
    pub gcd: i32, // greatest common divisor
    pub solvable: bool,
    pub general_solution: String,
    pub all_solutions: Vec<(i32, i32)>,

}

impl DiophantineEq {
    
    pub fn new(a:i32, b:i32, c:i32) -> DiophantineEq {
        let mut eq = DiophantineEq {
            a: a,
            b: b,
            c: c,
            x: 0,
            y: 0,
            gcd: 0,
            solvable: false,
            general_solution: String::new(),
            all_solutions: Vec::new(),

        };
        eq.compute();
        eq
    }

    pub fn compute(&mut self) {

        // solve for gcd, check if solvable
        if self.solve_gcd() % self.c != 0 {
            self.solvable = false;
            return;
        } else {
            self.solvable = true;
        }

        if self.solvable {
            // solve for bezout coefficients for a particular solution
            self.solve_bezout();
            // solve homogenous equation, add to particular solution 

            // set general solution to this & add first 100 solutions to all_solutions

        }
        
    }

    pub fn solve_gcd(&mut self) -> i32 {
        // use euclidean algorithm
        let mut a = self.a;
        let mut b = self.b;
        let mut r = 0;
        while b != 0 {
            r = a % b;
            a = b;
            b = r;
        }
        self.gcd = a;
        self.gcd
    }

    pub fn solve_bezout(&mut self) { //gives particular solution
        // use extended euclidean algorithm
        let mut r = 0;
        let mut s = 0;
        let mut t = 0;
        let mut old_r = self.a;
        let mut old_s = 1;
        let mut old_t = 0;
        let mut new_r = self.b;
        let mut new_s = 0;
        let mut new_t = 1;
        while new_r != 0 {
            let quotient = old_r / new_r;
            r = old_r - quotient * new_r;
            old_r = new_r;
            new_r = r;
            s = old_s - quotient * new_s;
            old_s = new_s;
            new_s = s;
            t = old_t - quotient * new_t;
            old_t = new_t;
            new_t = t;
        }
        self.x = old_s;
        self.y = old_t;
    }

}
