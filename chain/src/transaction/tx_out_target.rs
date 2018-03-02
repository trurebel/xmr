use transaction::{TxOutToKey, TxOutToScript, TxOutToScriptHash};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

const TO_KEY: u8 = 0x2;
const TO_SCRIPT: u8 = 0x0;
const TO_SCRIPT_HASH: u8 = 0x1;

/// Transaction output target.
#[derive(Debug, Clone)]
pub enum TxOutTarget {
    ToKey(TxOutToKey),
    ToScript(TxOutToScript),
    ToScriptHash(TxOutToScriptHash),
}

impl Deserialize for TxOutTarget {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        let tag = deserializer.get_u8()?;
        let target = match tag {
            TO_KEY => {
                TxOutTarget::ToKey(deserializer.get_deserializable()?)
            },
            TO_SCRIPT => {
                TxOutTarget::ToScript(deserializer.get_deserializable()?)
            },
            TO_SCRIPT_HASH => {
                TxOutTarget::ToScriptHash(deserializer.get_deserializable()?)
            },
            n => return Err(Error::custom(format!("unknown variant tag: {:X}", n))),
        };

        Ok(target)
    }
}

impl Serialize for TxOutTarget {
    fn serialize(&self, mut serializer: SerializerStream) {
        match *self {
            TxOutTarget::ToKey(ref v) => {
                serializer.put_u8(TO_KEY);
                serializer.put_serializable(v);
            },
            TxOutTarget::ToScript(ref v) => {
                serializer.put_u8(TO_SCRIPT);
                serializer.put_serializable(v);
            },
            TxOutTarget::ToScriptHash(ref v) => {
                serializer.put_u8(TO_SCRIPT_HASH);
                serializer.put_serializable(v);
            },
        }
    }
}
