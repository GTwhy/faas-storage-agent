use std::collections::HashMap;

/*
 * @Author: why
 * @Date: 2021-08-03 21:38:29
 * @LastEditTime: 2021-08-13 19:49:47
 * @LastEditors: why
 * @Description: 
 * @FilePath: /master/agent_server/src/redis_sa.rs
 * 
 */
 
use redis::Commands;
use crate::{faas_storage_agent::*};
use crate::storage_ns::{Backend, BackendNamespace, Namespace};

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Metadata {
    username: String,
    password: String,
    hostname: String,
    port: u16
}

fn get_prefix(ns: &Namespace) -> String {
    ns.client_id.to_string()
}

impl Metadata {
    pub fn default() -> Metadata {
        Metadata{
            username: String::from("openfaas"),
            password: String::from("openfaas"),
            hostname: env!("redis_hostname").to_string(),
            //hostname: String::from("127.0.0.1"),
            port: 6379
        }
    }
    pub fn get_local_url(&self) -> String{
        let url = format!("redis://{}:{}", self.hostname, self.port.to_string());
        url
    }

    pub fn get_remote_url(&self) -> String{
        // url format redis://[<username>][:<password>@]<hostname>[:port][/<db>]
        let url = format!("redis://{}:{}@{}:{}", self.username, self.password, self.hostname, self.port.to_string());
        url
    }
}

impl BackendNamespace for Metadata {
    fn to_backend_ns_args(&self) -> Vec<(String, String)> {
        let args: Vec<(String, String)> = vec![
            ("user".to_string(), self.username.clone()),
            ("password".to_string(), self.password.clone()),
            ("endpoint".to_string(), self.hostname.clone()),
            ("port".to_string(), self.port.to_string()),
            ];
        args
    }

    fn from_hashmap_to_backend(map: &HashMap<String, String>) -> Backend {
        let keys:[String; 4] = ["user".to_string(), "password".to_string(), "endpoint".to_string(), "port".to_string()];
        let mut values = ["admin".to_string(),"admin".to_string(),"localhost".to_string(),"6379".to_string()];
        let mut i = 0;

        while i < 4 {
            if let Some(v) = map.get(&keys[i]) {
                values[i] = v.to_string();
            }
            i += 1;
        }
        
        let md = Metadata{
            username: values[0].to_string(),
            password: values[1].to_string(),
            hostname: values[2].to_string(),
            port: values[3].to_string().parse::<u16>().unwrap()
        };
        Backend::Redis(md)
    }
}

fn get_metadata_from_ns(ns: Namespace) -> Metadata {
    match ns.backend {
        Backend::Redis(md) => md,
    }
}

pub fn connect_ns(ns: Namespace) -> ns_resp {
    let md = get_metadata_from_ns(ns);
    let _client = redis::Client::open(md.get_remote_url()).unwrap();
    let mut resp = ns_resp::default();
    resp.set_err_code(0);
    resp.set_err_info(String::from("connect successfully"));
    resp
}

pub fn set(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut client = redis::Client::open(url).unwrap();
    let _: () = client.set(get_prefix(&_ns) + req.get_key(), req.get_value()).unwrap();
    let mut resp = data_resp::default();
    resp.set_err_code(0);
    resp.set_err_info(String::from("set successfully"));
    resp
}

pub fn get(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut client = redis::Client::open(url).unwrap();
    let value : String = client.get(get_prefix(&_ns) + req.get_key()).unwrap();
    let mut resp = data_resp::default();
    resp.set_err_code(0);
    resp.set_value(value.into_bytes());
    resp.set_err_info(String::from("get successfully"));
    resp
}

pub fn exists(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut client = redis::Client::open(url).unwrap();
    let rv: bool = client.exists(get_prefix(&_ns) + req.get_key()).unwrap();
    let mut resp = data_resp::default();
    if rv {
        resp.set_err_code(0);
        resp.set_err_info(String::from("It exists"));
    }
    else{
        resp.set_err_code(1);
        resp.set_err_info(String::from("It doesn't exist"));
    }
    resp
}

pub fn delete(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut client = redis::Client::open(url).unwrap();
    let rv: bool = client.del(get_prefix(&_ns) + req.get_key()).unwrap();
    let mut resp = data_resp::default();
    if rv {
        resp.set_err_code(0);
        resp.set_err_info(String::from("delete successfully"));
    }
    else{
        resp.set_err_code(1);
        resp.set_err_info(String::from("It doesn't exist"));
    }
    resp
}
