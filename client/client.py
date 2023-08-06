"""
Example Test
"""
import json
from http import client


def main():
    conn = client.HTTPConnection('127.0.0.1', 11122)
    conn.request("POST", "/", json.dumps({
        "method": "concat",
        "args": {
            "a": 1,
            "b": "2"
        },
        "id": 2
    }))

    response = conn.getresponse()
    data = response.read()

    js = json.loads(data)
    print(js)

    conn.close()


if __name__ == '__main__':
    main()
