'''
Author: why
Date: 2021-07-18 11:00:18
LastEditTime: 2021-08-12 20:03:39
LastEditors: why
Description: 
FilePath: /sa/language_libs/python/faas-storage-agent/api.py

'''
from logging import critical, setLoggerClass
import grpc
from requests.api import head
from requests.sessions import PreparedRequest
import faas_storage_agent_pb2
import faas_storage_agent_pb2_grpc
import os
import base64
import requests, json
import random

class Agent:

    def __init__(self, ns_name = "default_namespace"):
        self.agent_url = self.get_agent_url()
        self.auth_url = self.get_auth_url()
        # TODO: STL support
        self.channel = grpc.insecure_channel(self.agent_url)
        self.stub = faas_storage_agent_pb2_grpc.faas_storage_agentStub(self.channel)
        self.token = self.get_ns_auth()
        print("token : ", self.token)


    def get_ns_auth(self):
        # TODO: Using a temp user-token pair now, and the real auth api will come soon.
        client_id = os.environ['sa_client_id']
        client_secret = os.environ['sa_client_secret']
        credential = "{0}:{1}".format(client_id,client_secret)
        credential_base64 = base64.b64encode(credential.encode("utf-8"))
        data = {'grant_type':'client_credentials'}
        authorization_content = 'Basic ' + credential_base64.decode()
        headers = {'Authorization':authorization_content, 'Cache-Control':'no-cache'}
        resp = requests.post(self.auth_url,data=data, headers=headers)
        token = json.loads(resp.text).get('access_token')
        return token

    def get_agent_url(self):
        # TODO: Using a remote storage agent now, and the local agnet will come soon.
        url = os.environ['agent_url']
        return url

    def get_auth_url(self):
        url = os.environ['auth_url']
        return url
# databse operating interfaces    

    def connect_ns(self, ns_name):
        req = faas_storage_agent_pb2.ns_req(name = ns_name, token =  self.token)
        resp = self.stub.connect_ns(req)
        print(resp.err_info)
        if resp.err_code == 0: 
            self.ns = ns_name
        return resp.err_code

    def create_ns(self, ns_name):
        req = faas_storage_agent_pb2.ns_req(name = ns_name, token =  self.token)
        resp = self.stub.create_ns(req)
        print(resp.err_info)
        return resp.err_code

    def delete_ns(self, ns_name):
        req = faas_storage_agent_pb2.ns_req(name = ns_name, token =  self.token)
        resp = self.stub.delete_ns(req)
        print(resp.err_info)
        return resp.err_code

# data operating interfaces
    def set(self, key, value):
        req = faas_storage_agent_pb2.data_req(key = key, value = value, token = self.token)
        resp = self.stub.set(req)
        print(resp.err_info)
        return resp.err_code

    def get(self, key):
        req = faas_storage_agent_pb2.data_req(key = key, token = self.token)
        resp = self.stub.get(req)
        print(resp.err_info)
        return resp.err_code, resp.value
    
    def delete(self, key):
        req = faas_storage_agent_pb2.data_req(key = key, token = self.token)
        resp = self.stub.delete(req)
        print(resp.err_info)
        return resp.err_code

    def exists(self, key):
        req = faas_storage_agent_pb2.data_req(key = key, token = self.token)
        resp = self.stub.exists(req)
        print(resp.err_info)
        return resp.err_code

if __name__ == '__main__':
    a = Agent()
    ns_name = "test_ns"
    a.create_ns(ns_name)
    a.connect_ns(ns_name)
    a.connect_ns(ns_name)
    rand = str(random.randint(1,1000))
    key = "test_key_" + rand
    value = ("test_value" + rand).encode()
    a.set(key, value)
    a.exists(key)
    c,v = a.get(key)
    print(v.decode())
    a.delete(key)
    a.exists(key)
    a.delete_ns(ns_name)