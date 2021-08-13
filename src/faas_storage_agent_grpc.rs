// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_FAAS_STORAGE_AGENT_CREATE_NS: ::grpcio::Method<super::faas_storage_agent::ns_req, super::faas_storage_agent::ns_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/create_ns",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_DELETE_NS: ::grpcio::Method<super::faas_storage_agent::ns_req, super::faas_storage_agent::ns_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/delete_ns",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_CONNECT_NS: ::grpcio::Method<super::faas_storage_agent::ns_req, super::faas_storage_agent::ns_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/connect_ns",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_SET: ::grpcio::Method<super::faas_storage_agent::data_req, super::faas_storage_agent::data_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/set",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_GET: ::grpcio::Method<super::faas_storage_agent::data_req, super::faas_storage_agent::data_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_DELETE: ::grpcio::Method<super::faas_storage_agent::data_req, super::faas_storage_agent::data_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FAAS_STORAGE_AGENT_EXISTS: ::grpcio::Method<super::faas_storage_agent::data_req, super::faas_storage_agent::data_resp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/faas_storage_agent.faas_storage_agent/exists",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct FaasStorageAgentClient {
    client: ::grpcio::Client,
}

impl FaasStorageAgentClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        FaasStorageAgentClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_ns_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_CREATE_NS, req, opt)
    }

    pub fn create_ns(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.create_ns_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_ns_async_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_CREATE_NS, req, opt)
    }

    pub fn create_ns_async(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.create_ns_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_ns_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_DELETE_NS, req, opt)
    }

    pub fn delete_ns(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.delete_ns_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_ns_async_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_DELETE_NS, req, opt)
    }

    pub fn delete_ns_async(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.delete_ns_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_ns_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_CONNECT_NS, req, opt)
    }

    pub fn connect_ns(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<super::faas_storage_agent::ns_resp> {
        self.connect_ns_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_ns_async_opt(&self, req: &super::faas_storage_agent::ns_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_CONNECT_NS, req, opt)
    }

    pub fn connect_ns_async(&self, req: &super::faas_storage_agent::ns_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::ns_resp>> {
        self.connect_ns_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_SET, req, opt)
    }

    pub fn set(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_async_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_SET, req, opt)
    }

    pub fn set_async(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_GET, req, opt)
    }

    pub fn get(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exists_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.client.unary_call(&METHOD_FAAS_STORAGE_AGENT_EXISTS, req, opt)
    }

    pub fn exists(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<super::faas_storage_agent::data_resp> {
        self.exists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exists_async_opt(&self, req: &super::faas_storage_agent::data_req, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.client.unary_call_async(&METHOD_FAAS_STORAGE_AGENT_EXISTS, req, opt)
    }

    pub fn exists_async(&self, req: &super::faas_storage_agent::data_req) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::faas_storage_agent::data_resp>> {
        self.exists_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait FaasStorageAgent {
    fn create_ns(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::ns_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::ns_resp>);
    fn delete_ns(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::ns_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::ns_resp>);
    fn connect_ns(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::ns_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::ns_resp>);
    fn set(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::data_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::data_resp>);
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::data_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::data_resp>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::data_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::data_resp>);
    fn exists(&mut self, ctx: ::grpcio::RpcContext, req: super::faas_storage_agent::data_req, sink: ::grpcio::UnarySink<super::faas_storage_agent::data_resp>);
}

pub fn create_faas_storage_agent<S: FaasStorageAgent + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_CREATE_NS, move |ctx, req, resp| {
        instance.create_ns(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_DELETE_NS, move |ctx, req, resp| {
        instance.delete_ns(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_CONNECT_NS, move |ctx, req, resp| {
        instance.connect_ns(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_SET, move |ctx, req, resp| {
        instance.set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_FAAS_STORAGE_AGENT_EXISTS, move |ctx, req, resp| {
        instance.exists(ctx, req, resp)
    });
    builder.build()
}
