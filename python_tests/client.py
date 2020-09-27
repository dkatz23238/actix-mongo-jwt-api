import requests
import json

times = []
ids = []
for i in range(100):
    r = requests.post(
        "http://localhost:3000/",
        headers={"content-type": "application/json"},
        data=json.dumps(
            {
                "username": "David" + str(i),
                "password": "password",
                "email": "email" + str(i) + "@gmail.com",
                "organization": "Org INC",
                "role": "Admin",
            }
        ),
    )
    print(json.loads(r.content))
    user_id = json.loads(r.content)["_id"]
    ids.append(user_id)
    times.append(r.elapsed.total_seconds())

times = []

for k in range(1000):

    for j, i in enumerate(ids):
        r = requests.post(
            "http://localhost:3000/authenticate",
            headers={"content-type": "application/json"},
            data=json.dumps({"username": "David" + str(j), "password": "password",}),
        )
        print(r.content)

        token = json.loads(r.content)["token"]

        r = requests.get(
            f"http://localhost:3000/{i}",
            headers={
                "content-type": "application/json",
                "authorization": f"bearer {token}",
            },
        )
        times.append(r.elapsed.total_seconds())
