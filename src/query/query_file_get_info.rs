use crate::{
    crypto::PublicKey,
    id::FileId,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, ErrorKind, PreCheckCode,
};
use chrono::{DateTime, Utc};
use failure::Error;
use std::convert::{TryFrom, TryInto};

pub struct FileInfo {
    pub file_id: FileId,
    pub size: i64,
    pub expiration_time: DateTime<Utc>,
    pub deleted: bool,
    pub keys: Vec<PublicKey>,
}

impl TryFrom<proto::FileGetInfo::FileGetInfoResponse_FileInfo> for FileInfo {
    type Error = Error;

    fn try_from(mut info: proto::FileGetInfo::FileGetInfoResponse_FileInfo) -> Result<Self, Error> {
        Ok(Self {
            file_id: info.take_fileID().into(),
            size: info.get_size(),
            expiration_time: info.take_expirationTime().into(),
            deleted: info.get_deleted(),
            keys: info
                .take_keys()
                .take_keys()
                .into_iter()
                .map(|k| k.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub struct QueryFileGetInfo {
    file: FileId,
}

impl QueryFileGetInfo {
    pub fn new(client: &Client, file: FileId) -> Query<FileInfo> {
        Query::new(client, Self { file })
    }
}

impl QueryInner for QueryFileGetInfo {
    type Response = FileInfo;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_fileGetInfo();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => response.take_fileInfo().try_into(),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::FileGetInfo::FileGetInfoQuery::new();
        query.set_header(header);
        query.set_fileID(self.file.to_proto()?);

        Ok(Query_oneof_query::fileGetInfo(query))
    }
}
