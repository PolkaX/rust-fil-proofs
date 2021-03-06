use ff::Field;
use paired::bls12_381::Fr;

use crate::hasher::Domain;

pub fn encode<T: Domain>(key: T, value: T) -> T {
    let mut result: Fr = value.into();
    let key: Fr = key.into();

    result.add_assign(&key);
    result.into()
}

pub fn decode<T: Domain>(key: T, value: T) -> T {
    let mut result: Fr = value.into();
    let key: Fr = key.into();

    result.sub_assign(&key);
    result.into()
}
