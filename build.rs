/*
 * @Author: why
 * @Date: 2021-08-01 11:45:48
 * @LastEditTime: 2021-08-02 15:01:14
 * @LastEditors: why
 * @Description: 
 * @FilePath: /slssa/agent_server/build.rs
 * 
 */

 fn main(){
     let proto_root = "../protos";
     let out_dir = "src/";
     println!("cargo:rerun-if-changed={}", proto_root);
     protoc_grpcio::compile_grpc_protos(&["faas_storage_agent.proto"], &[proto_root], &out_dir, None)
        .expect("Failed to compile gRPC definitions!");
 }