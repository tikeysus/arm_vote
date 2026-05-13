use crate::errors::CryptoError;
use crate::modint::ConstModInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group<const P: u64, const Q: u64> {
    pub generator: ConstModInt<P>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecretKey<const Q: u64> {
    value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey<const P: u64> {
    pub value: ConstModInt<P>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keypair<const P: u64, const Q: u64> {
    pub public: PublicKey<P>,
    pub secret: SecretKey<Q>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ciphertext<const P: u64> {
    pub c1: ConstModInt<P>,
    pub c2: ConstModInt<P>,
}

impl<const P: u64, const Q: u64> Group<P, Q> {
    pub fn new(generator: ConstModInt<P>) -> Result<Self, CryptoError> {
        if Q == 0 || generator.value() == 0 || generator.value() == 1 {
            return Err(CryptoError::InvalidGroupParameters);
        }

        if generator.pow(Q)?.value() != 1 {
            return Err(CryptoError::InvalidGroupParameters);
        }

        Ok(Self { generator })
    }

    pub fn keypair_from_secret(&self, secret: u64) -> Result<Keypair<P, Q>, CryptoError> {
        let secret = SecretKey::<Q>::new(secret)?;
        let public = PublicKey {
            value: self.generator.pow(secret.value())?,
        };

        Ok(Keypair { public, secret })
    }

    pub fn encrypt_with_nonce(
        &self,
        public: &PublicKey<P>,
        message: u64,
        nonce: u64,
    ) -> Result<Ciphertext<P>, CryptoError> {
        let nonce = normalize_nonzero::<Q>(nonce, CryptoError::InvalidNonce)?;
        let encoded_message = self.generator.pow(message % Q)?;

        Ok(Ciphertext {
            c1: self.generator.pow(nonce)?,
            c2: public.value.pow(nonce)?.mul(encoded_message)?,
        })
    }

    pub fn decrypt(
        &self,
        secret: &SecretKey<Q>,
        ciphertext: &Ciphertext<P>,
    ) -> Result<ConstModInt<P>, CryptoError> {
        let shared_secret = ciphertext.c1.pow(secret.value())?;
        ciphertext.c2.mul(shared_secret.inverse()?)
    }

    pub fn combine(
        &self,
        left: &Ciphertext<P>,
        right: &Ciphertext<P>,
    ) -> Result<Ciphertext<P>, CryptoError> {
        Ok(Ciphertext {
            c1: left.c1.mul(right.c1)?,
            c2: left.c2.mul(right.c2)?,
        })
    }

    pub fn decode_bounded_tally(
        &self,
        encoded_tally: ConstModInt<P>,
        max_tally: u64,
    ) -> Result<Option<u64>, CryptoError> {
        let mut current = ConstModInt::<P>::new(1)?;

        for tally in 0..=max_tally {
            if current == encoded_tally {
                return Ok(Some(tally));
            }
            current = current.mul(self.generator)?;
        }

        Ok(None)
    }
}

impl<const Q: u64> SecretKey<Q> {
    pub fn new(value: u64) -> Result<Self, CryptoError> {
        Ok(Self {
            value: normalize_nonzero::<Q>(value, CryptoError::InvalidSecretKey)?,
        })
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

fn normalize_nonzero<const Q: u64>(value: u64, error: CryptoError) -> Result<u64, CryptoError> {
    if Q == 0 {
        return Err(CryptoError::InvalidGroupParameters);
    }

    let value = value % Q;
    if value == 0 {
        return Err(error);
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestGroup = Group<23, 11>;
    type F = ConstModInt<23>;

    fn group() -> TestGroup {
        TestGroup::new(F::new(2).unwrap()).unwrap()
    }

    #[test]
    fn rejects_generator_outside_expected_subgroup() {
        assert_eq!(
            TestGroup::new(F::new(5).unwrap()),
            Err(CryptoError::InvalidGroupParameters)
        );
    }

    #[test]
    fn derives_public_key_from_secret_key() {
        let group = group();
        let keypair = group.keypair_from_secret(3).unwrap();

        assert_eq!(keypair.public.value.value(), 8);
        assert_eq!(keypair.secret.value(), 3);
    }

    #[test]
    fn encrypts_and_decrypts_encoded_vote() {
        let group = group();
        let keypair = group.keypair_from_secret(3).unwrap();
        let ciphertext = group.encrypt_with_nonce(&keypair.public, 1, 4).unwrap();

        let decrypted = group.decrypt(&keypair.secret, &ciphertext).unwrap();

        assert_eq!(decrypted, group.generator.pow(1).unwrap());
    }

    #[test]
    fn combines_ciphertexts_homomorphically() {
        let group = group();
        let keypair = group.keypair_from_secret(3).unwrap();
        let no_vote = group.encrypt_with_nonce(&keypair.public, 0, 4).unwrap();
        let yes_vote = group.encrypt_with_nonce(&keypair.public, 1, 7).unwrap();

        let combined = group.combine(&no_vote, &yes_vote).unwrap();
        let decrypted = group.decrypt(&keypair.secret, &combined).unwrap();
        let tally = group.decode_bounded_tally(decrypted, 2).unwrap();

        assert_eq!(tally, Some(1));
    }

    #[test]
    fn decodes_small_tally_by_trial_exponentiation() {
        let group = group();
        let encoded = group.generator.pow(3).unwrap();

        assert_eq!(group.decode_bounded_tally(encoded, 5).unwrap(), Some(3));
        assert_eq!(group.decode_bounded_tally(encoded, 2).unwrap(), None);
    }

    #[test]
    fn rejects_zero_secret_and_zero_nonce() {
        let group = group();

        assert_eq!(
            group.keypair_from_secret(0),
            Err(CryptoError::InvalidSecretKey)
        );

        let keypair = group.keypair_from_secret(3).unwrap();
        assert_eq!(
            group.encrypt_with_nonce(&keypair.public, 1, 0),
            Err(CryptoError::InvalidNonce)
        );
    }
}
