// Module for post-quantum cryptography
pub mod ml_kem;
pub mod ml_dsa;

pub fn pqc_init() {
    println!("Initializing PQC modules");
    ml_kem::ml_kem_encrypt();
    ml_dsa::ml_dsa_sign();
}
