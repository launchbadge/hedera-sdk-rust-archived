use crate::{
    id::FileId,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, ErrorKind, PreCheckCode,
};
use failure::Error;
use std::convert::{TryFrom, TryInto};

pub type QueryFileGetContentsResponse = Vec<u8>;

impl TryFrom<proto::FileGetContents::FileGetContentsResponse_FileContents>
    for QueryFileGetContentsResponse
{
    type Error = Error;

    fn try_from(
        mut contents: proto::FileGetContents::FileGetContentsResponse_FileContents,
    ) -> Result<Self, Error> {
        Ok(contents.take_contents())
    }
}

pub struct QueryFileGetContents {
    file: FileId,
}

impl QueryFileGetContents {
    pub fn new(client: &Client, file: FileId) -> Query<QueryFileGetContentsResponse> {
        Query::new(client, Self { file })
    }
}

impl QueryInner for QueryFileGetContents {
    type Response = QueryFileGetContentsResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_fileGetContents();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.take_fileContents().try_into()?),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::FileGetContents::FileGetContentsQuery::new();
        query.set_header(header);
        query.set_fileID(self.file.to_proto()?);

        Ok(Query_oneof_query::fileGetContents(query))
    }
}
