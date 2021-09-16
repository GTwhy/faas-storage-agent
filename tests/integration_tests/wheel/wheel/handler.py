from faassa import api
import random

def handle(req):
    a = api.Agent()
    ns_name = "test_ns"
    rand = str(random.randint(1,1000))
    key = "test_key_" + rand
    value = ("test_value" + rand).encode()

    c, i = a.create_ns(ns_name)
    if c != 0:
        print("create_ns err_info:", i)
        return req

    c, i = a.connect_ns(ns_name)
    if c != 0:
        print("connect_ns err_info:", i)
        return req

    c, i = a.set(key, value)
    if c != 0:
        print("set err_info:", i)
        return req

    c, i = a.exists(key)
    if c != 0:
        print("exists err_info:", i)
        return req

    c, i, v = a.get(key)
    if c != 0:
        print("get err_info:", i)
        return req

    if v != value:
        print("get err value : ", v.decode())
        return req

    c, i = a.delete(key)
    if c != 0:
        print("delete err_info:", i)
        return req

    c, i = a.exists(key)
    if c == 0:
        print("exists err_info:", i)
        return req

    c, i = a.delete_ns(ns_name)
    if c != 0:
        print("delete_ns err_info:", i)
        return req
    
    print("ok")
    return req
