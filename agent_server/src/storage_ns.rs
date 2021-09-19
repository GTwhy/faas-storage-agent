/*
 * @Author: why
 * @Date: 2021-08-06 10:09:29
 * @LastEditTime: 2021-08-12 16:33:29
 * @LastEditors: why
 * @Description: 
 * @FilePath: /sa/agent_server/src/storage_ns.rs
 * 
 */
use std::collections::HashMap;
use crate::{redis_sa::{*, self}};
use redis::*;

#[derive(PartialEq,Clone,Debug)]
pub enum Backend {
    Redis(redis_sa::Metadata),
}

#[derive(PartialEq,Clone,Debug)]
pub struct Namespace {
    pub client_id: String,
    pub name: String,
    pub backend: Backend
}

pub struct NsManager{
    connection: redis::Connection,
}

impl NsManager {
    pub fn new(md: &Metadata) -> NsManager{
        let url = md.get_remote_url();
        let client = redis::Client::open(url).unwrap();
        let connection = client.get_connection().unwrap();
        NsManager{connection}
    }
    
    pub fn new_backend_ns(&mut self, client_id: &str, ns_name: &str, backend: Backend) -> bool {
        let key = client_id.to_string() + ns_name;
        let mut itms = vec![
            ("client_id".to_string(), client_id.to_string()),
            ("name".to_string(), ns_name.to_string()),
        ];
        match backend {
            Backend::Redis(metadata) => {
                itms.push(("backend_type".to_string(), String::from("Redis")));
                itms.append(&mut metadata.to_backend_ns_args());
            },
        }
        let rv = self.connection.hset_multiple(key, &itms);
        match rv {
            Ok(value) => {
                if value {
                    return true
                }else {
                    println!("hset_multiple failed");
                    return false
                }
            },
            Err(err) => {
                println!("new_backend_ns hset multiple error rv: {:#?}", err);
                return false
            }
        }
    }
    
    // pub fn delete_backend_ns(&self, client_id: String, backend_ns: String) -> Result<bool, RedisError> {
    //     Ok(true);
    // }
    
    pub fn get_backend_ns(&mut self, client_id: &str, ns_name: &str) -> Result<Namespace, bool> {
        let key = client_id.to_string() + ns_name;
        let rv = self.connection.hgetall(key);
        match rv {
            Ok(value) => {
                if let Ok(hm) = from_redis_value::<HashMap<String, String>>(&value){
                    let keys:[String; 2] = ["client_id".to_string(), "name".to_string()];
                    let mut values = ["default".to_string(), "default".to_string()];
                    let mut i = 0;
                    while i < 2 {
                        if let Some(v) = hm.get(&keys[i]) {
                            values[i] = v.to_string();
                        }
                        i += 1;
                    }
                    let ns: Namespace;
                    let backend: Backend;
                    if let Some(backend_type) = hm.get("backend_type") {
                        match backend_type.as_str() {
                            "Redis" => {
                                backend = Metadata::from_hashmap_to_backend(&hm);
                                ns = Namespace{
                                    client_id: values[0].clone(),
                                    name: values[1].clone(),
                                    backend,
                                };
                            }
                            _ => {
                                return Err(false);
                            }
                        }
                        return Ok(ns);
                    } else {
                        println!("can't get backend_type from hashmap");
                        return Err(false)
                    }
                }else{
                    println!("get hashmap from redis value failed.");
                    return Err(false)
                }
            },
            Err(err) => {
                println!("hget_all err : {:?}", err);
                return Err(false)
            }
        }        
    }

    pub fn delete_backend_ns(&mut self, ns: &Namespace) -> bool {
        let key = ns.client_id.to_string() + ns.name.as_str();
        //TODO: err handling and multi-backend support
        let rv = self.connection.del(key);
        match rv {
            Ok(value) => {
                if value {
                    return true;
                }else {
                    println!("backend_ns doesn't exist.");
                    return false;
                }
            },
            Err(err) => {
                println!{"delete_backend_ns failed. rv: {:?}", err};
                return false
            }
        }
    }
}


pub trait BackendNamespace {
    fn to_backend_ns_args(&self) -> Vec<(String, String)>;
    fn from_hashmap_to_backend(map: &HashMap<String, String>) -> Backend;
}
