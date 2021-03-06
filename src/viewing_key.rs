use core::{mem, fmt};

use cosmwasm_std::{Env};

use crate::rand::{Prng, sha_256};
use crate::utils::{create_hashed_password, ct_slice_compare};

pub const API_KEY_LENGTH: usize = 44 + 8;

#[derive(Clone)]
pub struct ViewingKey(pub String);

impl ViewingKey {
    pub fn check_viewing_key(&self, hashed_pw: &[u8]) -> bool {

        let mine_hashed = create_hashed_password(&self.0);

        ct_slice_compare(mine_hashed.to_vec().as_slice(), hashed_pw)
    }

    pub fn new(env: &Env, seed: &[u8], entropy: &[u8]) -> Self {

        let mut rng_entropy: Vec<u8> = vec![];
        rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
        rng_entropy.extend_from_slice(&env.block.time.to_be_bytes());
        rng_entropy.extend_from_slice(&env.message.sender.as_slice());
        rng_entropy.extend_from_slice(entropy);

        let mut rng = Prng::new(seed, &*rng_entropy);

        let key = sha_256(unsafe { mem::transmute::<[u32; 8], [u8; 32]>(rng.rand_slice()) }.as_ref() );

        Self("api_key_".to_string() + &base64::encode(key))
    }

    pub fn to_hashed(&self) -> [u8; 24] {
        create_hashed_password(&self.0)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn is_valid(&self) -> bool {
        if self.0.len() != API_KEY_LENGTH {
            return false;
        }
        return true;
    }
}

impl fmt::Display for ViewingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}