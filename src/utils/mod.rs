use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_prime::{PrimalityTestConfig, RandPrime};
use num_traits::{One, Zero};
use rand::thread_rng;


pub struct Polynomial {
    pub coefficients: Vec<BigUint>,
}

impl Polynomial {
    
    /// Creates a new random polynomial of the given degree.
    pub fn new_for_shamir(threshold: usize, secret_bits: usize, secret: &BigUint) -> Self {
        let mut rng = thread_rng();
        let mut coefficients = vec![secret.clone()];

        for _ in 1..threshold {
            let coef = rng.gen_biguint_range(&BigUint::one(), &(BigUint::one() << secret_bits));
            coefficients.push(coef);
        }

        Polynomial { coefficients }
    }
    
    // evaluate the polynomial
    pub fn evaluate(&self, x: &BigUint) -> BigUint {
        let mut result = BigUint::zero();
        let mut x_pow = BigUint::one();

        for coef in &self.coefficients {
            result += coef * &x_pow;
            x_pow *= x;
        }
        result
    }

    
}


// modular exponentiation
pub fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exponent, modulus)
}

// genarate prime number of bit_size
pub fn generate_prime(bit_size: usize) -> BigUint {
    let mut rng = thread_rng();
    let config = PrimalityTestConfig::default();
    rng.gen_prime(bit_size, Some(config))
}

// Extended Euclidean Algorithm gcd
// ( ax + by = g )
pub fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b, Zero::zero(), One::one())
    } else {
        let (g, x, y) = egcd(b.clone() % a.clone(), a.clone());
        (g, y - (b / a.clone()) * x.clone(), x)
    }
}

// modular inverse
pub fn mod_inv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (g, x, _) = egcd(a.to_bigint().unwrap(), m.to_bigint().unwrap());
    if g == One::one() {
        let x_mod_m = ((x % m.to_bigint().unwrap()) + m.to_bigint().unwrap()) % m.to_bigint().unwrap();
        
        Some(x_mod_m.to_biguint().unwrap())
    } else {
        None
    }
}



// finding  S from the polynomial
pub fn lagrange_interpolation_zero(points: &[(BigUint, BigUint)], modulus: &BigUint) -> Option<BigUint> {
    let mut secret = BigUint::zero();

    for (i, (x_i, y_i)) in points.iter().enumerate() {
        let mut numerator = BigUint::one();
        let mut denominator = BigUint::one();

        for (j, (x_j, _)) in points.iter().enumerate() {
            if i != j {
                let x_diff = (modulus - x_j) % modulus;
                numerator = (numerator * x_diff) % modulus;
                denominator = (denominator * (x_i + modulus - x_j) % modulus) % modulus;
            }
        }
        let inv_denominator = mod_inv(&denominator, modulus)?;
        let term = (y_i * &numerator * inv_denominator) % modulus;        
        secret = (secret + term) % modulus;
    }
    Some(secret)
}