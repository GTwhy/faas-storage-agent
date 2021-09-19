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
use crate::faas_storage_agent::*;
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

    // pub fn get_local_url(&self) -> String{
    //     let url = format!("redis://{}:{}", self.hostname, self.port.to_string());
    //     url
    // }

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
    let rv = redis::Client::open(md.get_remote_url());
    let mut resp = ns_resp::default();
    match rv {
        Ok(client) => {
            resp.set_err_code(0);
            resp.set_err_info(String::from("connect successfully"));
        },
        Err(err_info) => {
            println!("connect_ns err: {:?}", err_info);
            resp.set_err_code(1);
            resp.set_err_info(String::from("connect failed"));
        }
    }
    resp
}

pub fn set(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut resp = data_resp::default();
    let mut client = redis::Client::open(url).unwrap();
    let rv = client.set(get_prefix(&_ns) + req.get_key(), req.get_value());
    match rv {
        Ok(value) => {
            if value {
                resp.set_err_code(0);
                resp.set_err_info(String::from("set successfully"));
            }else{
                resp.set_err_code(1);
                resp.set_err_info(String::from("set failed"));
            }
        },
        Err(err) => {
            println!("{:?}", err);
            resp.set_err_code(2);
            resp.set_err_info(String::from("set failed"));
        }
    }
    resp
}

pub fn get(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut resp = data_resp::default();
    let mut client = redis::Client::open(url).unwrap();
    let rv = client.get(get_prefix(&_ns) + req.get_key());
    match rv {
        Ok(value) => {
            match value {
                Some(vec) => {
                    resp.set_err_code(0);
                    resp.set_value(vec);
                    resp.set_err_info(String::from("get successfully"));
                },
                None => {
                    resp.set_err_code(1);
                    resp.set_err_info(String::from("get None"));
                }
            }
        },
        Err(err) => {
            println!("{:?}", err);
            resp.set_err_code(2);
            resp.set_err_info(String::from("get failed"));
        }
    }
    resp
}

pub fn exists(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut resp = data_resp::default();
    let mut client = redis::Client::open(url).unwrap();
    let rv = client.exists(get_prefix(&_ns) + req.get_key());
    match rv {
        Ok(value) => {
            if value {
                resp.set_err_code(0);
                resp.set_err_info(String::from("it exists"));
            } else {
                resp.set_err_code(1);
                resp.set_err_info(String::from("it doesn't exist"));
            }
        },
        Err(err) => {
            println!("{:?}", err);
            resp.set_err_code(2);
            resp.set_err_info(String::from("exists failed"));
        }
    }
    resp
}

pub fn delete(req: &data_req, _ns: Namespace) -> data_resp {
    let md = Metadata::default();
    let url = md.get_remote_url();
    let mut client = redis::Client::open(url).unwrap();
    let rv = client.del(get_prefix(&_ns) + req.get_key());
    let mut resp = data_resp::default();
    match rv {
        Ok(value) => {
            if value {
                resp.set_err_code(0);
                resp.set_err_info(String::from("delete successfully"));
            }
            else{
                resp.set_err_code(1);
                resp.set_err_info(String::from("it doesn't exist"));
            }
        },
        Err(err) => {
            println!("{:?}", err);
            resp.set_err_code(2);
            resp.set_err_info(String::from("delete failed"));
        }
    }

    resp
}


#[cfg(test)]
mod tests {

    use crate::redis_sa::*;
    use crate::storage_ns::*;
    use crate::faas_storage_agent;

    fn get_test_data_req(key: &str, value: &str) -> data_req{
        let mut req = data_req::new();
        req.set_key(key.to_string());
        req.set_value(value.as_bytes().to_vec());
        return req;
    }

    #[test]
    fn connect_ns_test() {
        let md = Metadata::default();
        let mut nsm = NsManager::new(&md);
        let backend = Backend::Redis(Metadata::default());
        nsm.new_backend_ns("connect_test_client_id", "connect_test_ns_name", backend);
        let mut resp = ns_resp::new();
        if let Ok(ns) = nsm.get_backend_ns("connect_test_client_id", "connect_test_ns_name"){
            resp = connect_ns(ns.clone());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            nsm.delete_backend_ns(&ns);
        }else{
            panic!("NsManager get backend namespace failed");
        }
    }

    #[test]
    fn set_get_test(){
        let md = Metadata::default();
        let mut nsm = NsManager::new(&md);
        let backend = Backend::Redis(Metadata::default());
        nsm.new_backend_ns("set_get_test_client_id", "set_get_test_ns_name", backend);
        let mut resp = data_resp::default();
        let req = get_test_data_req("set_get_test_key", "set_get_test_value");
        if let Ok(ns) = nsm.get_backend_ns("set_get_test_client_id", "set_get_test_ns_name"){
            resp = set(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            resp = get(&req, ns.clone());
            assert_eq!(resp.get_value(), "set_get_test_value".as_bytes(), "err value:{:?}", resp.get_value());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            nsm.delete_backend_ns(&ns);
        }else{
            panic!("NsManager get backend namespace failed");
        }
    }

    #[test]
    fn exists_delete_test(){
        let md = Metadata::default();
        let mut nsm = NsManager::new(&md);
        let backend = Backend::Redis(Metadata::default());
        nsm.new_backend_ns("exists_delete_test_client_id", "exists_delete_test_ns_name", backend);
        let mut resp = data_resp::default();
        let req = get_test_data_req("exists_delete_test_key", "exists_delete_test_value");
        if let Ok(ns) = nsm.get_backend_ns("exists_delete_test_client_id", "exists_delete_test_ns_name"){
            resp = set(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            resp = exists(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            resp = delete(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 0, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            resp = exists(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 1, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            resp = delete(&req, ns.clone());
            assert_eq!(resp.get_err_code(), 1, "err_code:{}  err_info:{}", resp.get_err_code(), resp.get_err_info());
            nsm.delete_backend_ns(&ns);
        }else{
            panic!("NsManager get backend namespace failed");
        }
    }
}