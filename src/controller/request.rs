use uuid::{Uuid};
use crate::controller::manifest::Manifest;
use crate::controller::request::RequestData::DownloadResource;
use crate::controller::request::RequestState::{COMPLETED, ERROR, PENDING};

pub enum RequestState {
    PENDING,
    COMPLETED(RequestData),
    ERROR(String)
}
#[derive(Clone, Debug)]
pub enum RequestType {
    DownloadResource
}

impl RequestType {
    fn get_request_data(&self, data: &Vec<u8>) -> Option<RequestData> {

        match self {
            RequestType::DownloadResource => {
               Some(DownloadResource(Manifest::from_bytes(data)))
            }
        }
    }
}

pub enum RequestData {
    DownloadResource(Manifest),
}

#[derive(Clone, Debug)]
pub struct Request {
    id: u8,
    request_type: RequestType,
    len: Option<u64>,
    data: Vec<u8>,
}

impl Request {

    pub fn pending_bytes(&self) -> usize {

        self.len.unwrap() as usize - self.data.len()
    }

    pub fn add(&mut self, data: &Vec<u8>) -> RequestState {
        self.data.extend_from_slice(data);
        if self.pending_bytes() > 0 {
            return PENDING;
        }

        match self.request_type.get_request_data(&self.data) {
            Some(request_data) => {COMPLETED(request_data)}
            None => {ERROR("Failed to add data to request data!".to_string())}
        }
    }

    pub fn get_length(&self) -> Option<usize> {
        self.len.map(|len| {len as usize})
    }

    pub fn init_length(&mut self, length: u64) -> usize {
        if self.len.is_none() {
            self.len = Some(length);
        }
        self.len.unwrap() as usize
    }

}

pub static REQUESTS: &[Request] = &[
    Request { id: 0, request_type: RequestType::DownloadResource, len: None, data: vec![]},
];

pub struct RequestFactory;

impl RequestFactory {
    pub fn get_request(id: u8) -> Option<Request> {
        REQUESTS.iter().find(|r| r.id == id).cloned()
    }
}