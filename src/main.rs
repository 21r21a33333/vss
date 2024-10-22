mod utils;
mod feldman_verifiability;
use feldman_verifiability::{reconstruct_secret, verify_share, FeldmanVSSParams};
use num_bigint::ToBigUint;
use utils::generate_prime;


fn main(){
     // Feldman's Verifiable Secret Sharing (VSS)
    // Secret to be shared
    let secret = 986743267.to_biguint().unwrap();
    let threshold = 10;
    let num_shares = 15;
    
     let g = 3.to_biguint().unwrap();
    let q = generate_prime(256);


    let params = FeldmanVSSParams::new(g, q);
    println!("{:?}",params);

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
    use std::str::FromStr;

    use num_bigint::BigUint;

    use super::*;

    #[test]
    #[should_panic]
    fn test_feldman_vss() {
        // Feldman's Verifiable Secret Sharing (VSS)
        // Secret to be shared
        let secret = 986743267.to_biguint().unwrap();
        let threshold = 2;
        let num_shares = 5;

        let g = 3.to_biguint().unwrap();
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

    #[test]
    fn interactive_feldman_vss() {
        let shares = vec![
            (9550.to_biguint().unwrap(), BigUint::from_str("18919602278323673031046851760634693089117").unwrap()),
            (24571.to_biguint().unwrap(), BigUint::from_str("36327887789328451003212201782115387342483493").unwrap()),
            (25933.to_biguint().unwrap(), BigUint::from_str("55934357380513682591568081080354454850198681").unwrap()),
            (68519.to_biguint().unwrap(), BigUint::from_str("132841689833329350976110287839060563757719983677").unwrap()),
            (83331.to_biguint().unwrap(), BigUint::from_str("635766873914960083864607408973982106190173553973").unwrap()),
            (69931.to_biguint().unwrap(), BigUint::from_str("156388223167164532559319605168774853012564848773").unwrap()),
            (63668.to_biguint().unwrap(), BigUint::from_str("73827200827679711151479550212387225767223347831").unwrap()),
            (98037.to_biguint().unwrap(), BigUint::from_str("2333267208719916331893291708962359562461031958313").unwrap()),
            (8913.to_biguint().unwrap(), BigUint::from_str("10891294502686836777382457274393259980481").unwrap()),
            (49756.to_biguint().unwrap(), BigUint::from_str("10270989938358300332627341680612225590405522623").unwrap()),
            (80651.to_biguint().unwrap(), BigUint::from_str("489466949230035041645701935679697121654450264133").unwrap()),
            (73409.to_biguint().unwrap(), BigUint::from_str("230589914975321730063972438418175389153326506017").unwrap()),
            (62653.to_biguint().unwrap(), BigUint::from_str("64920494536977701702442817440197491822626514681").unwrap()),
            (65862.to_biguint().unwrap(), BigUint::from_str("96811251235585944050484408222911856035702886613").unwrap()),
            (57085.to_biguint().unwrap(), BigUint::from_str("30833552411738175997637326741290713764838927737").unwrap()),
        ];

        let commitments = vec![
            BigUint::from_str("13126420383068338128945497109713047802042211444418566693982109672322157867506").unwrap(),
            BigUint::from_str("11608252019833344481067501514889404975802366901069732424381596350173677867166").unwrap(),
            BigUint::from_str("74848086454058703152874126878808099257028914799352738012045221827888506519856").unwrap(),
            BigUint::from_str("32747611984764819058159202615507510078184872987161563325651774231999858591519").unwrap(),
            BigUint::from_str("38498891942037509122470071137942652677743017376337639159073899908721954514711").unwrap(),
            BigUint::from_str("77318421827468798149829158931660979124799921968196212784738277802917759270837").unwrap(),
            BigUint::from_str("21763044245426738464849354678324532039471513180336697777445611573883236270916").unwrap(),
            BigUint::from_str("48124709334983853620945668779764531788162387636878529269667956732660229186497").unwrap(),
            BigUint::from_str("65399536914080467172370801603907012699409604681638958120018197966562541542478").unwrap(),
        ];

        let g = 3.to_biguint().unwrap();    
        let q = BigUint::from_str("89715053225915147086466643337824135904976077704764787688030218510402096592169").unwrap();
        let params = FeldmanVSSParams::new(g, q);

        for (i, &(ref x, ref y)) in shares.iter().enumerate() {
            assert!(verify_share(x, y, &commitments, &params), "Share {} failed verification", i + 1);
        }
}
}




