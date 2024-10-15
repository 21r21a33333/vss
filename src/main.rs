use num_bigint::BigInt;
use num_traits::pow;
use num_traits::ToPrimitive;
use rand::Rng;
struct FeldmanVSS {
    prime: BigInt,
    generator: i32,
    num_participants: i32,
    threashold: u32,
    coefficients: Vec<BigInt>,
    commitments: Vec<BigInt>,
    shares: Vec<BigInt>,
}

fn mod_exp(base: i128, exponent: BigInt, modulus: BigInt) -> BigInt {
    let mut result = BigInt::from(1);
    let mut base = BigInt::from(base);
    let mut exponent = exponent.clone();
    while exponent > BigInt::from(0) {
        if exponent.clone() % BigInt::from(2) == BigInt::from(1) {
            result = (result * base.clone()) % modulus.clone();
        }
        exponent = exponent / BigInt::from(2);
        base = (base.clone() * base.clone()) % modulus.clone();
    }
    return result;
}

fn eval_polynomial(coefficients: &Vec<BigInt>, x: BigInt, prime: BigInt) -> BigInt {
    let mut result = BigInt::from(0);
    let mut power_of_x = BigInt::from(1);
    for i in 0..coefficients.len() {
        result = (result + (coefficients[i].clone() * power_of_x.clone())%prime.clone()) % prime.clone();
        power_of_x = (power_of_x.clone() * x.clone()) % prime.clone();
    }
    return result;
}

impl FeldmanVSS {
    fn new(prime: BigInt, generator: i32, num_participants: i32, threashold: u32) -> FeldmanVSS {
        FeldmanVSS {
            prime: prime,
            generator: generator,
            num_participants: num_participants,
            threashold: threashold,
            coefficients: Vec::new(),
            commitments: Vec::new(),
            shares: Vec::new(),
        }
    }

    fn generate_secret_shares(&mut self, secret: BigInt) {
        self.coefficients = vec![BigInt::from(0); self.threashold as usize];
        self.coefficients[0] = secret;

        for i in 1..self.threashold {
            let mut p: BigInt = BigInt::from(0);
            while p == BigInt::ZERO {
                // assigning random number to the polynomial as a coefficient
                p = rand::thread_rng().gen_range(BigInt::from(1)..BigInt::from(1000));
            }
            self.coefficients[i as usize] = p;
        }

        self.commitments = vec![BigInt::from(0); self.threashold as usize];

        for j in 0..self.threashold {
            self.commitments[j as usize] = mod_exp(
                self.generator.into(),
                self.coefficients[j as usize].clone(),
                self.prime.clone(),
            );
        }

        self.shares = vec![BigInt::from(0); self.num_participants as usize];
        for i in 1..self.num_participants+1 {
            self.shares[i as usize - 1] =
                eval_polynomial(&self.coefficients, BigInt::from(i), self.prime.clone());
        }
    }

    fn verify_share(&mut self, i: i32, share: BigInt) -> bool {
        let lhs = mod_exp(self.generator.into(), share.clone(), self.prime.clone());

        let mut rhs = BigInt::from(1);

        for j in 0..self.threashold {
            rhs =
                (rhs * mod_exp(
                    self.commitments[j as usize].to_i128().unwrap(),
                    BigInt::from(pow::<i128>(i.into(), j as usize)),
                    self.prime.clone(),
                )) % self.prime.clone();
        }
        println!("lhs: {}", lhs);
        println!("rhs: {}", rhs);
        lhs == rhs
    }

    fn print_info(&self) {
        println!("Prime: {}", self.prime);
        println!("Generator: {}", self.generator);
        println!("Number of Participants: {}", self.num_participants);
        println!("Threashold: {}", self.threashold);
        println!("Coefficients: {:?}", self.coefficients);
        println!("Commitments: {:?}", self.commitments);
        println!("Shares: {:?}", self.shares);
    }
}

fn main() {
    let prime = BigInt::from(2089);
    let generator = 2;
    let num_participants = 5;
    let threashold = 3;
    let secret = BigInt::from(1234);

    let mut vss = FeldmanVSS::new(prime, generator, num_participants, threashold);
    vss.generate_secret_shares(secret.clone());
    vss.print_info();

    for i in 1..num_participants +1 {
        if vss.verify_share(i, vss.shares[i as usize - 1].clone()) {
            println!("share {} is verified", i);
        } else {
            println!("share {} is not verified i.e invalid", i);
        }
    }
}