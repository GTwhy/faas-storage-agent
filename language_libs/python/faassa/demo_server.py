'''
Author: why
Date: 2021-07-28 14:06:51
LastEditTime: 2021-07-28 14:13:32
LastEditors: why
Description: 
FilePath: /slssa/language_libs/python/faas-storage-agent/test_server.py

'''
import logging
from typing import SupportsComplex
import grpc
from requests.api import head
from requests.exceptions import SSLError
import faas_storage_agent_pb2
import faas_storage_agent_pb2_grpc
from concurrent import futures
import requests, json
import base64

def validate_token(token, scopes):
    # TODO fill
    client_id = 'IUkCQnAo8Un4pTVfWNf1a0LlKbD7neBdVwmMeqLy'
    client_secret = 'Hp8IH6BL7iRkKdbwTvVs17A7pUIkIMhc0TU9sq40cHoxpkPPFqJwe865HG1IZhtXDRekIdWuOp3UmwPBKWq6L0TBYgXQmTFhW5UG7FPfp23Otff4gtsBCAmXzmtYRwh7'
    credential = "{0}:{1}".format(client_id,client_secret)
    credential_base64 = base64.b64encode(credential.encode("utf-8"))
    data = {'token':token}
    authorization_content = 'Basic ' + credential_base64.decode()
    headers = {'Authorization':authorization_content}
    resp = requests.post('http://127.0.0.1:10087/o/introspect/', data=data, headers=headers)
    print(resp.text)
    token = json.loads(resp.text).get('access_token')
    # TODO:Check scope
    if json.loads(resp.text).get('active') == True:
        return True
    else:
        return False

class AgentServicer(faas_storage_agent_pb2_grpc.faas_storage_agentServicer):

    # databse operating interfaces    

    def connect_ns(self, request, context):
        print(request)
        # channel info test
        print(context.details())
        scopes = {request.name, 'read'}
        if validate_token(request.token, scopes):
            # valicate successfully
            return faas_storage_agent_pb2.ns_resp(err_code = 0)
        else:
            # TODO: Improve error handling.
            return faas_storage_agent_pb2.ns_resp(err_code = -1, err_info = "Token validaton failed")

#     def create_ns(self, request, context):

#     def delete_ns(self, request, context):

# # data operating interfaces
#     def set(self, request, context):

#     def get(self, request, context):
    
#     def delete(self, request, context):

#     def exists(self, request, context):



def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    faas_storage_agent_pb2_grpc.add_faas_storage_agentServicer_to_server(AgentServicer(), server)
    server.add_insecure_port('[::]:10086')
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    serve()