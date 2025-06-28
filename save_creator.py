import json
import random

x = {
  'bodies': []
}

with open("save.json", "w") as outfile:
    for body_index in range(1000):
        x['bodies'].append({
            "position": [
                random.random() * 10000 - 5000,
                random.random() * 10000 - 5000
            ],
            "radius": random.random() * 1 + 30,
            "mass": random.random() * 1 + 3000000,
            "velocity": [
                random.random() * 3 - 1,
                random.random() * 3 - 1
            ]
        })

    outfile.write((json.dumps(x, indent=2) + "\n"))
