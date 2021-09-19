from faassa import api
import random

def handle(req):
    a = api.Agent()
    ns_name = "test_ns"
    rand = str(random.randint(1,1000))
    key = "test_key_" + rand
    value = ("test_value" + rand).encode()
    count_ok = 10

    c, i = a.create_ns(ns_name)
    if c != 0:
        count_ok -= 1
        print("create_ns test ... failed. err_info:", i)
    print("create_ns test ... ok")
    
    c, i = a.connect_ns(ns_name)
    if c != 0:
        count_ok -= 1
        print("connect_ns test ... failed. err_info:", i)
    print("connect_ns test ... ok")

    c, i = a.set(key, value)
    if c != 0:
        count_ok -= 1
        print("set test ... failed. err_info:", i)
    print("set test ... ok")

    c, i = a.exists(key)
    if c != 0:
        count_ok -= 1
        print("exists test ... failed. err_info:", i)
    print("exists test ... ok")

    c, i, v = a.get(key)
    if c != 0:
        print("get test ... failed. err_info:", i)
    if v != value:
        count_ok -= 1
        print("get test ... failed. err value : ", v.decode())
    print("get test ... ok")

    c, i = a.delete(key)
    if c != 0:
        count_ok -= 1
        print("delete test ... failed. err_info:", i)
    print("delete test ... ok")

    c, i = a.exists(key)
    if c == 0:
        count_ok -= 1
        print("exists test ... failed. err_info:", i)
    print("exists test ... ok")

    c, i, v = a.get(key)
    if c == 0:
        count_ok -= 1
        print("get test ... failed. err_info:", i)
    print("get test ... ok")

    c, i = a.delete_ns(ns_name)
    if c != 0:
        count_ok -= 1
        print("delete_ns test ... failed. err_info:", i)
    print("delete_ns test ... ok")

    c, i = a.connect_ns(ns_name)
    if c == 0:
        count_ok -= 1
        print("connect_ns test ... failed. err_info:", i)
    print("connect_ns test ... ok")
    
    if count_ok == 10 :
        res = "ok"
    else :
        res = "failed"
    print("test result: {0}. {1} passed; {2} failed", res, count_ok, 10 - count_ok)
    return req
