use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bigdecimal::BigDecimal;
use num::{bigint::ToBigInt, BigUint, BigInt};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Used to serialize BigUint as radix 10 string.
#[derive(Clone, Debug)]
pub struct BigUintSerdeAsRadix10Str;

impl BigUintSerdeAsRadix10Str {
    pub fn serialize<S>(val: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let big_dec = BigDecimal::from(val.to_bigint().unwrap());
        BigDecimal::serialize(&big_dec, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        BigDecimal::deserialize(deserializer).and_then(|bigdecimal| {
            let big_int = bigdecimal
                .to_bigint()
                .ok_or_else(|| Error::custom("Expected integer value"))?;
            big_int
                .to_biguint()
                .ok_or_else(|| Error::custom("Expected positive value"))
        })
    }
}

pub struct VecBigUintSerdeAsRadix10Str;

impl VecBigUintSerdeAsRadix10Str {
    pub fn serialize<S>(operations: &[BigUint], ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut res = Vec::with_capacity(operations.len());
        for val in operations.iter() {
            let big_dec = BigDecimal::from(val.to_bigint().unwrap());
            res.push(big_dec);
        }
        Vec::serialize(&res, ser)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BigUint>, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        let str_vec: Vec<BigDecimal> = Vec::deserialize(deserializer)?;
        let mut res = Vec::with_capacity(str_vec.len());
        for s in str_vec.into_iter() {
            let v = s.to_bigint()
                .ok_or_else(|| Error::custom("Expected integer value"))?;
            res.push(v
                .to_biguint()
                .ok_or_else(|| Error::custom("Expected positive value"))?);
        }
        Ok(res)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigUintSerdeWrapper(#[serde(with = "BigUintSerdeAsRadix10Str")] pub BigUint);

impl From<BigUint> for BigUintSerdeWrapper {
    fn from(uint: BigUint) -> BigUintSerdeWrapper {
        BigUintSerdeWrapper(uint)
    }
}

impl From<BigInt> for BigUintSerdeWrapper {
    fn from(big_int: BigInt) -> BigUintSerdeWrapper {
        BigUintSerdeWrapper(big_int.to_biguint().unwrap())
    }
}
impl Deref for BigUintSerdeWrapper{
    type Target = BigUint;

    fn deref(&self) -> &BigUint {
        &self.0
    }
}

impl DerefMut for BigUintSerdeWrapper{
    fn deref_mut(&mut self) -> &mut BigUint {
        &mut self.0
    }
}

impl  BigUintSerdeWrapper {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Trait for specifying prefix for bytes to hex serialization
pub trait Prefix {
    fn prefix() -> &'static str;
}

/// "sync-bl:" hex prefix
pub struct SyncBlockPrefix;
impl Prefix for SyncBlockPrefix {
    fn prefix() -> &'static str {
        "sync-bl:"
    }
}

/// "0x" hex prefix
pub struct ZeroxPrefix;
impl Prefix for ZeroxPrefix {
    fn prefix() -> &'static str {
        "0x"
    }
}

/// "sync-tx:" hex prefix
pub struct SyncTxPrefix;
impl Prefix for SyncTxPrefix {
    fn prefix() -> &'static str {
        "sync-tx:"
    }
}

/// Used to annotate `Vec<u8>` fields that you want to serialize like hex-encoded string with prefix
/// Use this struct in annotation like that `[serde(with = "BytesToHexSerde::<T>"]`
/// where T is concrete prefix type (e.g. `SyncBlockPrefix`)
pub struct BytesToHexSerde<P> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Prefix> BytesToHexSerde<P> {
    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // First, serialize to hexadecimal string.
        let hex_value = format!("{}{}", P::prefix(), hex::encode(value));

        // Then, serialize it using `Serialize` trait implementation for `String`.
        String::serialize(&hex_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized_string = String::deserialize(deserializer)?;

        if let Some(deserialized_string) = deserialized_string.strip_prefix(P::prefix()) {
            hex::decode(&deserialized_string).map_err(de::Error::custom)
        } else {
            Err(de::Error::custom(format!(
                "string value missing prefix: {:?}",
                P::prefix()
            )))
        }
    }
}

pub type ZeroPrefixHexSerde = BytesToHexSerde<ZeroxPrefix>;

/// Used to annotate `Option<Vec<u8>>` fields that you want to serialize like hex-encoded string with prefix
/// Use this struct in annotation like that `[serde(with = "OptionBytesToHexSerde::<T>"]`
/// where T is concrete prefix type (e.g. `SyncBlockPrefix`)
pub struct OptionBytesToHexSerde<P> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Prefix> OptionBytesToHexSerde<P> {
    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // First, serialize to hexadecimal string.
        let hex_value = value
            .as_ref()
            .map(|val| format!("{}{}", P::prefix(), hex::encode(val)));

        // Then, serialize it using `Serialize` trait implementation for `String`.
        Option::serialize(&hex_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize a string value. It is expected to be a
        // hexadecimal representation of `Vec<u8>`.
        let optional_deserialized_string: Option<String> = Option::deserialize(deserializer)?;

        optional_deserialized_string
            .map(|s| {
                if let Some(hex_str) = s.strip_prefix(P::prefix()) {
                    hex::decode(hex_str).map_err(de::Error::custom)
                } else {
                    Err(de::Error::custom(format!(
                        "string value missing prefix: {:?}",
                        P::prefix()
                    )))
                }
            })
            .transpose()
    }
}

/// Used to serialize BigUint as radix 10 string.
#[derive(Clone, Debug)]
pub struct OptionBigUintSerdeAsRadix10Str;

impl OptionBigUintSerdeAsRadix10Str {
    pub fn serialize<S>(val: &Option<BigUint>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut big_dec = None;
        if let Some(v) = val.to_owned() {
            big_dec = Some(BigDecimal::from(v.to_bigint().unwrap()));
        };
        Option::serialize(&big_dec, serializer)

    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BigUint>, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        let optional_deserialized_string: Option<String> = Option::deserialize(deserializer)?;
        let mut res = None;
        if let Some(s) = optional_deserialized_string {
            let big_int = BigDecimal::from_str(&s).unwrap().to_bigint()
                .ok_or_else(|| Error::custom("Expected integer value"))?;
            res = big_int.to_biguint()
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use num::rational::Ratio;
    use super::*;

    /// Tests that `Ratio` serializer works correctly.
    #[test]
    fn test_ratio_serialize_as_decimal() {
        #[derive(Clone, Serialize, Deserialize)]
        struct RatioSerdeWrapper(
            #[serde(with = "UnsignedRatioSerializeAsDecimal")] pub Ratio<BigUint>,
        );
        // It's essential that this number is a finite decimal, otherwise the precision will be lost
        // and the assertion will fail.
        let expected = RatioSerdeWrapper(Ratio::new(
            BigUint::from(120315391195132u64),
            BigUint::from(1250000000u64),
        ));
        let value =
            serde_json::to_value(expected.clone()).expect("cannot serialize Ratio as Decimal");
        let ratio: RatioSerdeWrapper =
            serde_json::from_value(value).expect("cannot deserialize Ratio from Decimal");
        assert_eq!(expected.0, ratio.0);
    }

    /// Tests that `BigUint` serializer works correctly.
    #[test]
    fn test_serde_big_uint_wrapper() {
        let expected = BigUint::from(u64::MAX);
        let wrapper = BigUintSerdeWrapper::from(expected.clone());
        let value = serde_json::to_value(wrapper).expect("cannot serialize BigUintSerdeWrapper");
        let uint: BigUintSerdeWrapper =
            serde_json::from_value(value).expect("cannot deserialize BigUintSerdeWrapper");
        assert_eq!(uint.0, expected);
    }
}
