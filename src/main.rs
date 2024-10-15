mod utils;
mod feldman_verifiability;
use feldman_verifiability::{reconstruct_secret, verify_share, FeldmanVSSParams};
use num_bigint::ToBigUint;
use utils::generate_prime;


fn main(){
     // Feldman's Verifiable Secret Sharing (VSS)
    // Secret to be shared
    let secret = 986743267.to_biguint().unwrap();
    let threshold = 2;
    let num_shares = 5;
    
     let g = 2.to_biguint().unwrap();
    let q = generate_prime(256);

    let params = FeldmanVSSParams::new(g, q);

    let (shares, mut commitments) = params.generate_shares(&secret, threshold, num_shares);


    for (i, &(ref x, ref y)) in shares.iter().enumerate() {
        assert!(verify_share(x, y, &commitments, &params), "Share {} failed verification", i + 1);
    }

    let reconstructed_secret = reconstruct_secret(&shares[..threshold], &params.q).unwrap();
    println!("Feldman's Verifiable Secret Sharing (VSS):");
    println!("Original Secret: {}", secret);
    println!("Reconstructed Secret: {}", reconstructed_secret);

}




#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::One;

    #[test]
    #[should_panic]
    fn test_feldman_vss() {
        // Feldman's Verifiable Secret Sharing (VSS)
        // Secret to be shared
        let secret = 986743267.to_biguint().unwrap();
        let threshold = 2;
        let num_shares = 5;

        let g = 2.to_biguint().unwrap();
        let q = generate_prime(256);

        let params = FeldmanVSSParams::new(g, q);

        let (shares, mut commitments) = params.generate_shares(&secret, threshold, num_shares);

            let corrupt_data: Vec<_> = commitments.iter().map(|c| c + 1.to_biguint().unwrap()).collect();

        for (i, &(ref x, ref y)) in shares.iter().enumerate() {
            assert!(verify_share(x, y, &corrupt_data, &params), "Share {} failed verification", i + 1);
        }

        let reconstructed_secret = reconstruct_secret(&shares[..threshold], &params.q).unwrap();
        assert_eq!(secret, reconstructed_secret);
    }
}