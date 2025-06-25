load('ext://dotenv', 'dotenv')
dotenv()

# Spin up database
k8s_yaml("bees-postgres/k8s.yaml")
k8s_resource("bees-postgres", port_forwards=5432)
