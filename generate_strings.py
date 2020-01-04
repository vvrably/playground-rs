"""
Script used to create random names.
"""

from faker import Faker

fake = Faker()
Faker.seed(1)


for x in range(7):
    n = 10**x
    path = "data/names-" + str(n) + ".txt"
    with open(path, "w") as f:
        l = [fake.name() + "\n" for _ in range(n)]
        f.writelines(l)
