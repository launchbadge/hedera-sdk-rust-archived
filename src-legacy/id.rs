macro_rules! define_id {
    ($field:ident, $name:ident, $proto:ident, $method_set:ident, $method_get:ident) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        #[repr(C)]
        pub struct $name {
            pub shard: i64,
            pub realm: i64,
            pub $field: i64,
        }

        impl $name {
            pub fn new(shard: i64, realm: i64, $field: i64) -> Self {
                Self {
                    shard,
                    realm,
                    $field,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}:{}:{}", self.shard, self.realm, self.$field)
            }
        }

        impl std::str::FromStr for $name {
            type Err = failure::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::ErrorKind::Parse;
                use itertools::Itertools;

                let (shard, realm, $field) = s
                    .split(&[':', '.'][..])
                    .map(str::parse)
                    .next_tuple()
                    .ok_or_else(|| Parse("{shard}:{realm}:{num}"))?;

                Ok(Self::new(shard?, realm?, $field?))
            }
        }

        impl From<crate::proto::BasicTypes::$proto> for $name {
            fn from(pb: crate::proto::BasicTypes::$proto) -> Self {
                Self {
                    shard: pb.get_shardNum(),
                    realm: pb.get_realmNum(),
                    $field: pb.$method_get(),
                }
            }
        }

        impl crate::proto::ToProto<crate::proto::BasicTypes::$proto> for $name {
            fn to_proto(&self) -> Result<crate::proto::BasicTypes::$proto, failure::Error> {
                let mut proto = crate::proto::BasicTypes::$proto::new();
                proto.set_shardNum(self.shard);
                proto.set_realmNum(self.realm);
                proto.$method_set(self.$field);

                Ok(proto)
            }
        }
    };
}

define_id!(
    account,
    AccountId,
    AccountID,
    set_accountNum,
    get_accountNum
);

define_id!(file, FileId, FileID, set_fileNum, get_fileNum);

define_id!(
    contract,
    ContractId,
    ContractID,
    set_contractNum,
    get_contractNum
);
