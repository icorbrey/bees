load("ext://dotenv", "dotenv")
dotenv()

# Spin up database
k8s_yaml("bees-postgres/k8s.yaml")
k8s_resource("bees-postgres", port_forwards=5432)

# Spin up webhook
docker_build("bees-webhook", context=".", dockerfile="bees-webhook/Dockerfile")
k8s_yaml("bees-webhook/k8s.yaml")
k8s_resource("bees-webhook", port_forwards=3000)
