import requests
import json

TEST_CACHE = {}


def test_create_user():
    r = requests.post(
        "http://localhost:3000/users",
        headers={"content-type": "application/json"},
        data=json.dumps(
            {
                "username": "David",
                "password": "password",
                "email": "david@gmail.com",
                "organization": "Org INC",
                "role": "Admin",
            }
        ),
    )
    print(json.loads(r.content))
    user_id = json.loads(r.content)["_id"]
    TEST_CACHE["user_id"] = user_id
    assert r.status_code == 200


def test_authenticate_user():
    r = requests.post(
        "http://localhost:3000/authenticate",
        headers={"content-type": "application/json"},
        data=json.dumps({"username": "David", "password": "password",}),
    )
    print(r.content)

    token = json.loads(r.content)["token"]
    TEST_CACHE["token"] = token
    assert r.status_code == 200


def test_update_user_with_token():
    user_id = TEST_CACHE["user_id"]
    token = TEST_CACHE["token"]
    print(token)
    r = requests.put(
        f"http://localhost:3000/users/{user_id}",
        headers={
            "content-type": "application/json",
            "authorization": f"bearer {token}",
        },
        data=json.dumps({"email": "mynewemail@email.com",}),
    )
    print(r.content)
    assert r.status_code == 200


def test_get_user_with_token():
    user_id = TEST_CACHE["user_id"]
    token = TEST_CACHE["token"]
    print(token)
    r = requests.get(
        f"http://localhost:3000/users/{user_id}",
        headers={
            "content-type": "application/json",
            "authorization": f"bearer {token}",
        },
    )
    print(r.content)
    data = json.loads(r.content)
    assert r.status_code == 200
    assert data["email"] == "mynewemail@email.com"


def test_delete_user():
    user_id = TEST_CACHE["user_id"]
    token = TEST_CACHE["token"]
    print(token)
    r = requests.delete(
        f"http://localhost:3000/users/{user_id}",
        headers={
            "content-type": "application/json",
            "authorization": f"bearer {token}",
        },
    )
    print(r.content)
    data = json.loads(r.content)
    assert r.status_code == 200
    assert data["success"]
