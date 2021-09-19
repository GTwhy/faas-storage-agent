/*
 * @Author: why
 * @Date: 2021-07-31 16:58:16
 * @LastEditTime: 2021-08-15 04:19:53
 * @LastEditors: why
 * @Description: 
 * @FilePath: /master/agent_server/src/agent_server.rs
 * 
 */

mod storage_ns;
mod redis_sa;
mod faas_storage_agent;
mod faas_storage_agent_grpc;
use std::collections::HashMap;
use std::process::exit;
use std::sync::{Mutex, Arc};
use std::time::{Duration, SystemTime};
use std::thread;
use faas_storage_agent::*;
use faas_storage_agent_grpc::*;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{Environment, Error, RpcContext, ServerBuilder, UnarySink};
use hyper::{Body, Client, Method, Request, Response};
use json::{self, JsonValue};
use storage_ns::Namespace;
use crate::storage_ns::Backend;

#[derive(PartialEq,Clone,Debug)]
struct ClientInfo{
    token: String,
    client_id: String,
    //Unix timestamp
    auth_expires: u64,
    cache_expires: u64,
    scope: Scope,
    current_ns: Namespace,
    clear: bool
}

#[derive(PartialEq,Default,Clone,Debug)]
struct DataOpt{
    set: bool,
    get: bool,
    delete: bool,
    exists: bool,
}

#[derive(PartialEq,Default,Clone,Debug)]
struct NsOpt{
    create_ns: bool,
    delete_ns: bool,
}
#[derive(PartialEq,Default,Clone,Debug)]
struct Scope{
    ns_opt: NsOpt,
    data_opt: HashMap<String, DataOpt>
}

#[derive(PartialEq,Default,Clone,Debug)]
struct AuthenticationInfo{
    client_id: String,
    expires: u64,
    scope: Scope
}

#[derive(Clone,Default,Debug)]
struct AgentService{
    backend_name: String,
    client_cache: Arc<Mutex<HashMap<String, ClientInfo>>>,
}

impl AgentService {
    fn set_backend_name(&mut self, name: String) {
        self.backend_name = name;
    }

    fn cache_client_info(&mut self, token: &str, auth_info: AuthenticationInfo, ns: &Namespace) {
        let mut map = self.client_cache.lock().unwrap();
        map.insert(token.to_string(), ClientInfo{
            token: token.to_string(),
            client_id: auth_info.client_id,
            auth_expires: auth_info.expires,
            cache_expires: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 10,
            scope: auth_info.scope,
            current_ns: ns.clone(),
            clear: false
        });
    }

    fn update_client_cache(&mut self,mut client_info: ClientInfo, new_ns_name: &str) ->Result<ClientInfo, bool> {
        let md = redis_sa::Metadata::default();
        let mut nsm = storage_ns::NsManager::new(&md);
        if let Ok(new_ns) = nsm.get_backend_ns(client_info.client_id.as_str(),  new_ns_name){
            client_info.current_ns = new_ns;
            let mut map = self.client_cache.lock().unwrap();
            map.remove(&client_info.token);
            map.insert(client_info.token.clone(), client_info.clone());
            Ok(client_info)
        }
        else {
            Err(false)
        }
    }

    fn clear_client_cache(&mut self, token: &str) {
        let mut map = self.client_cache.lock().unwrap();
        map.remove(token);
    }

    fn check_get_client_cache(&mut self, token: &str, ns_name: &str) -> Result<ClientInfo, bool> {
        if let Ok(client_info) = self.is_client_alive(token){
            if client_info.current_ns.name != *ns_name {
                //Replace the current namespace with new namespace
                
                Ok(self.update_client_cache(client_info, ns_name)?)
            }
            else {
                Ok(client_info)
            }
        }
        else {
            Err(false)
        }
    }
    
    fn get_client_cache(&mut self, token: &str) -> Result<ClientInfo, bool> {
        if let Ok(client_info) = self.is_client_alive(token){
            return Ok(client_info)
        }
        Err(false)
    }

    fn is_client_alive(&mut self, token: &str) -> Result<ClientInfo, bool> {
        let mut map = self.client_cache.lock().unwrap();
        if let Some(client) = map.get(&token.to_string()){
            if let Ok(dur) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
                let now = dur.as_secs();
                if now < client.auth_expires && now < client.cache_expires && !client.clear {
                    return Ok(client.to_owned())
                }
                else {
                    map.remove(token);
                }
            }
        }
        Err(false)
    }

}

fn new_err_ns_resp(err_code: i32, err_info: &str) ->ns_resp {
    let mut resp = ns_resp::new();
    resp.set_err_code(err_code);
    resp.set_err_info(err_info.to_string());
    resp
}

fn new_err_data_resp(err_code: i32, err_info: &str) ->data_resp {
    let mut resp = data_resp::new();
    resp.set_err_code(err_code);
    resp.set_err_info(err_info.to_string());
    resp
}

impl FaasStorageAgent for AgentService {

    fn connect_ns(&mut self, ctx:RpcContext<'_>, req: ns_req, sink: UnarySink<ns_resp>){
        println!("connect_ns = {:?}", req);
        let resp: ns_resp;
        //Hit cache
        if let Ok(client_info) = self.check_get_client_cache(req.get_token(), req.get_name()) {
            match self.backend_name.as_str() {
                "Redis" => resp = redis_sa::connect_ns(client_info.current_ns),
                _ => resp = new_err_ns_resp(4, "Error backen_name"),
            }
        }
        //Cache miss
        else {
            //Validate token successfully
            if let Ok(auth_info) = validate_token(req.get_token()) {
                let md = redis_sa::Metadata::default();
                let mut nsm = storage_ns::NsManager::new(&md);
                //Cache ns info
                if let Ok(ns) = nsm.get_backend_ns(auth_info.client_id.as_str(),  req.get_name()){
                    self.cache_client_info(req.get_token(), auth_info, &ns);
                    resp = redis_sa::connect_ns(ns)
                }
                else {
                    resp = new_err_ns_resp(1, "can not find this namespace")
                }
            }
            else {
                resp = new_err_ns_resp(2, "Client validation failed")
            }
        }
        let f = sink
        .success(resp)
        .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn create_ns(&mut self, ctx:RpcContext<'_>, req: ns_req, sink: UnarySink<ns_resp>){
        println!("create_ns = {:?}", req);
        let mut resp = ns_resp::default();
        let backend = Backend::Redis(redis_sa::Metadata::default());
        let md = redis_sa::Metadata::default();
        let mut nsm = storage_ns::NsManager::new(&md);
        //Hit cache
        if let Ok(client_info) = self.check_get_client_cache(req.get_token(), req.get_name()) {
            if check_ns_scope( &client_info.scope, "create_ns"){
                match self.backend_name.as_str() {
                    "Redis" => {
                        if nsm.new_backend_ns(client_info.client_id.as_str(),  req.get_name(), backend){
                            resp.set_err_code(0);
                            resp.set_err_info("Create ns successfully".to_string());
                        }
                        else {
                            resp = new_err_ns_resp(3, "can not create new namespace")
                        }
                    },
                    _ => resp = new_err_ns_resp(4, "Error backen_name"),
                }
            }else{
                resp = new_err_ns_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            //Validate token successfully
            if let Ok(auth_info) = validate_token(req.get_token()) {
                if check_ns_scope(&auth_info.scope, "create_ns"){
                    if nsm.new_backend_ns(auth_info.client_id.as_str(),  req.get_name(), backend){
                        resp.set_err_code(0);
                        resp.set_err_info("create ns successfully".to_string());
                    }
                    else {
                        resp = new_err_ns_resp(3, "can not create this namespace")
                    }
                }else {
                    resp = new_err_ns_resp(5, "Err scope");
                }
            }
            else {
                resp = new_err_ns_resp(2, "Client validation failed")
            }
        }
        let f = sink
        .success(resp)
        .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
        .map(|_| ());
        ctx.spawn(f)
    }
    
    fn delete_ns(&mut self, ctx:RpcContext<'_>, req: ns_req, sink: UnarySink<ns_resp>){
        println!("delete_ns = {:?}", req);
        let mut resp = ns_resp::default();
        let md = redis_sa::Metadata::default();
        let mut nsm = storage_ns::NsManager::new(&md);
        //Hit cache
        if let Ok(client_info) = self.check_get_client_cache(req.get_token(), req.get_name()) {
            if check_ns_scope(&client_info.scope, "delete_ns"){
                nsm.delete_backend_ns(&client_info.current_ns);
                self.clear_client_cache(&client_info.token);
            }else {
                resp = new_err_ns_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            //Validate token successfully
            if let Ok(auth_info) = validate_token(req.get_token()) {
                if check_ns_scope(&auth_info.scope, "delete_ns"){
                    //Cache ns info
                    if let Ok(ns) = nsm.get_backend_ns(auth_info.client_id.as_str(),  req.get_name()){
                        self.cache_client_info(req.get_token(), auth_info.clone(), &ns);
                        if nsm.delete_backend_ns(&ns) {
                            resp.set_err_code(0);
                            resp.set_err_info("delete ns successfully".to_string());
                        }
                        else {
                            resp = new_err_ns_resp(3, "can delete this namespace")
                        }
                    }
                    else {
                        resp = new_err_ns_resp(1, "can not find this namespace")
                    }
                }else {
                    resp = new_err_ns_resp(5, "Err scope");
                }
            }
            else {
                resp = new_err_ns_resp(2, "Client validation failed")
            }
        }
        let f = sink
        .success(resp)
        .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn set(&mut self, ctx:RpcContext<'_>, req: data_req, sink: UnarySink<data_resp>){
        println!("set = {:?}", req);
        let resp: data_resp;
        //Hit cache
        if let Ok(client_info) = self.get_client_cache(req.get_token()) {
            if check_data_scope(client_info.current_ns.name.as_str(), &client_info.scope, "set"){
                match self.backend_name.as_str() {
                    "Redis" => resp = redis_sa::set(&req, client_info.current_ns),
                    _ => resp = new_err_data_resp(4, "Error backen_name"),
                }
            }else {
                resp = new_err_data_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            resp = new_err_data_resp(2, "Connect to the namespace first.");
        }
        let f = sink
            .success(resp)
            .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn get(&mut self, ctx:RpcContext<'_>, req: data_req, sink: UnarySink<data_resp>){
        println!("get = {:?}", req);
        let resp: data_resp;
        //Hit cache
        if let Ok(client_info) = self.get_client_cache(req.get_token()) {
            if check_data_scope(client_info.current_ns.name.as_str(), &client_info.scope, "get"){
                match self.backend_name.as_str() {
                    "Redis" => resp = redis_sa::get(&req, client_info.current_ns),
                    _ => resp = new_err_data_resp(4, "Error backen_name"),
                }
            }else {
                resp = new_err_data_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            resp = new_err_data_resp(2, "Connect to the namespace first.");
        }
        let f = sink
            .success(resp)
            .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx:RpcContext<'_>, req: data_req, sink: UnarySink<data_resp>){
        println!("delete = {:?}", req);
        let resp: data_resp;
        //Hit cache
        if let Ok(client_info) = self.get_client_cache(req.get_token()) {
            if check_data_scope(client_info.current_ns.name.as_str(), &client_info.scope, "delete"){
                match self.backend_name.as_str() {
                    "Redis" => resp = redis_sa::delete(&req, client_info.current_ns),
                    _ => resp = new_err_data_resp(4, "Error backen_name"),
                }
            }else {
                resp = new_err_data_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            resp = new_err_data_resp(2, "Connect to the namespace first.");
        }
        let f = sink
            .success(resp)
            .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn exists(&mut self, ctx:RpcContext<'_>, req: data_req, sink: UnarySink<data_resp>){
        println!("exists = {:?}", req);
        let resp: data_resp;
        //Hit cache
        if let Ok(client_info) = self.get_client_cache(req.get_token()) {
            if check_data_scope(client_info.current_ns.name.as_str(), &client_info.scope, "exists"){
                match self.backend_name.as_str() {
                    "Redis" => resp = redis_sa::exists(&req, client_info.current_ns),
                    _ => resp = new_err_data_resp(4, "Error backen_name"),
                }
            }else {
                resp = new_err_data_resp(5, "Err scope");
            }
        }
        //Cache miss
        else {
            resp = new_err_data_resp(2, "Connect to the namespace first.");
        }
        let f = sink
            .success(resp)
            .map_err(move |e: Error| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

}

fn main() {
    println!("Hello, world!");
    let mut agent_service = AgentService::default();
    agent_service.set_backend_name("Redis".to_string());
    let svc = create_faas_storage_agent(agent_service);
    let env = Arc::new(Environment::new(1));
    let mut server = ServerBuilder::new(env)
        .register_service(svc)
        .bind("0.0.0.0", 10086)
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    ctrlc::set_handler(move || {
        println!("bye...");
        let _ = block_on(server.shutdown());
        exit(0);
    }).expect("Error setting Ctrl-C handler");
    loop {
        thread::sleep(Duration::from_secs(10000));
    }
}

fn check_ns_scope(client_scope: &Scope, required_scope: &str) -> bool{
    match required_scope {
        "create_ns" => return client_scope.ns_opt.create_ns,
        "delete_ns" => return client_scope.ns_opt.delete_ns,
        _ => return false
    }
}

fn check_data_scope(ns_name: &str, client_scope: &Scope, required_scope: &str) -> bool{
    if let Some(data_opt) = client_scope.data_opt.get(ns_name){   
        match required_scope {
            "get" => return data_opt.get,
            "set" => return data_opt.set,
            "delete" => return data_opt.delete,
            "exists" => return data_opt.exists,
            _ => return false
        }
    }else {
        return false
    }
}

fn line_to_vec(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}

fn get_scope_from_lines(parsed: &JsonValue) -> Scope{
    println!("{}", parsed["scope"]);
    let mut scope = Scope::default();
    let lines = parsed["scope"].as_str().unwrap().lines();
    for line in lines {
        let v = line_to_vec(line);
        if v[0] == "ns" {
            if line.contains("create_ns") {
                scope.ns_opt.create_ns = true;
            }
            if line.contains("delete_ns") {
                scope.ns_opt.delete_ns = true;
            }
        }else if v[0] == "data" {
            let mut data_opt = DataOpt::default();
            let ns_name = v[1].to_string();
            if line.contains("get"){
                data_opt.get = true;
            }
            if line.contains("set"){
                data_opt.set = true;
            }
            if line.contains("delete"){
                data_opt.delete = true;
            }
            if line.contains("exists"){
                data_opt.exists = true;
            }
            scope.data_opt.insert(ns_name, data_opt);
        }else{
            println!("Err Scope");
        }
       
    }
    scope
}

fn get_auth_info_from_response(res: Response<Body>) -> Result<AuthenticationInfo, bool>{
    let bytes = block_on(hyper::body::to_bytes(res)).unwrap();
    let result = String::from_utf8(bytes.into_iter().collect()).unwrap();
    let parsed = json::parse(result.as_str()).unwrap();
    let scope = get_scope_from_lines(&parsed);
    let auth_info = AuthenticationInfo{
        client_id: parsed["client_id"].to_string(),
        expires: parsed["exp"].as_u64().unwrap(),
        scope
    };
    println!("{:?}", auth_info);
    Ok(auth_info)
}

#[tokio::main]
async fn validate_token(_token: &str) -> Result<AuthenticationInfo, bool> {
    let client_id = env!("sas_client_id");
    let client_secret = env!("sas_client_secret");
    let credential= format!("{}:{}", client_id, client_secret);
    let credential_base64 = base64::encode(credential);
    let auth_content = "Basic ".to_string() + &credential_base64;
    let body = format!("token={}", _token);
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://39.105.134.149:10087/o/introspect/")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .header("Authorization", auth_content)
        .body(Body::from(body))
        .expect("request builder");
    let req_client = Client::new();
    let res = block_on(req_client.request(req)).expect("err resp");
    get_auth_info_from_response(res)
}

#[test]
fn validate_token_test(){
    validate_token("test_token");
}

#[test]
fn scope_check_test(){
    let mut test_scope = Scope::default();
    test_scope.data_opt.insert("test_ns".to_string(), DataOpt::default());
    assert!(!check_data_scope("test_ns", &test_scope, "set"));
    assert!(!check_data_scope("test_ns", &test_scope, "get"));
    assert!(!check_data_scope("test_ns", &test_scope, "delete"));
    assert!(!check_data_scope("test_ns", &test_scope, "exists"));
    assert!(!check_ns_scope(&test_scope, "create_ns"));
    assert!(!check_ns_scope(&test_scope, "delete_ns"));
    test_scope.ns_opt.create_ns = true;
    test_scope.ns_opt.delete_ns = true;
    let dpt = DataOpt{
        get: true,
        set: true,
        delete: true,
        exists: true
    };
    test_scope.data_opt.remove("test_ns");
    test_scope.data_opt.insert("test_ns".to_string(), dpt);
    assert!(check_data_scope("test_ns", &test_scope, "set"));
    assert!(check_data_scope("test_ns", &test_scope, "get"));
    assert!(check_data_scope("test_ns", &test_scope, "delete"));
    assert!(check_data_scope("test_ns", &test_scope, "exists"));
    assert!(check_ns_scope(&test_scope, "create_ns"));
    assert!(check_ns_scope(&test_scope, "delete_ns"));
}