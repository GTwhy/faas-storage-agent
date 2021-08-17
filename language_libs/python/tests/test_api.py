from faassa import api
import random
def test():
    a = api.Agent()
    ns_name = "test_ns"
    a.create_ns(ns_name)
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