
use crate::utils::{Polynomial, mod_exp,lagrange_interpolation_zero};
use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

/// Represents the public parameters for the Feldman VSS scheme.
#[derive(Debug, Clone)]
pub struct FeldmanVSSParams {
    pub g: BigUint, // Generator of the group G
    pub q: BigUint, // Prime order of the group G
}

impl FeldmanVSSParams {

    /// Initializes Feldman VSS parameters with a generator and prime order.
    pub fn new(g: BigUint, q: BigUint) -> Self {
        FeldmanVSSParams { g, q }
    }


    /// Generates shares from the polynomial generated with random coefficients.
    pub fn generate_shares(&self, secret: &BigUint, threshold: usize, num_shares: usize) -> (Vec<(BigUint, BigUint)>, Vec<BigUint>) {
        let poly = Polynomial::new_for_shamir(threshold - 1, secret.bits() as usize, secret);
        let mut shares = Vec::with_capacity(num_shares);

        // Generate shares using the polynomial, similar to Shamir's scheme
        for i in 1..=num_shares {
            // let x = i.to_biguint().unwrap();
            // let x = BigUint::from(rand::random::<u64>()); // Generate a random BigUint
            let x = BigUint::from(rand::random::<u64>() % 100000 + 1); // Generate a random BigUint in the range of 1 to 1000
            let y = poly.evaluate(&x) % &self.q; // Ensure the evaluation is done modulo q
            shares.push((x, y));
        }

        println!("Shares: {:?}", shares);

        // Generate commitments for the polynomial's coefficients for verifiability
        let commitments = self.generate_commitments(&poly);
        println!("Commitments: {:?}", commitments);

        (shares, commitments)
    }
    
    // Generate commitments for the polynomial's coefficients for verifiability
    fn generate_commitments(&self, polynomial: &Polynomial) -> Vec<BigUint> {
        polynomial.coefficients.iter().map(|coef| {
            mod_exp(&self.g, coef, &self.q) // Compute g^coef mod q for each coefficient
        }).collect()
    }

}

/// Verifies a share using the public commitments and the Feldman VSS parameters.
pub fn verify_share(
    i: &BigUint, // Share index
    share: &BigUint, // Share value
    commitments: &[BigUint], // Public commitments
    params: &FeldmanVSSParams, // VSS parameters
) -> bool {
    // Calculate the left-hand side (LHS) as g^share mod q
    let lhs = mod_exp(&params.g, share, &params.q);

    // Calculate the right-hand side (RHS) as the product of commitments raised to the power of the share index
    let mut rhs = BigUint::one();
    for (j, commitment) in commitments.iter().enumerate() {
        let exponent = i.modpow(&BigUint::from(j), &params.q);
        rhs = (rhs * mod_exp(commitment, &exponent, &params.q)) % &params.q;
    }

    lhs == rhs
}

// secreat reconstruction
pub fn reconstruct_secret(shares: &[(BigUint, BigUint)], modulus: &BigUint) -> Option<BigUint> {
    lagrange_interpolation_zero(shares, modulus)
}


