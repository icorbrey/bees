load("ext://dotenv", "dotenv")

# Load environment variables
dotenv()
local_resource("bees-secrets",
               cmd='kubectl delete secret bees-secrets --ignore-not-found && kubectl create secret generic bees-secrets --from-env-file=.env',
               deps=['.env'],
               labels=["secrets"])

# Load private keys
local_resource("github-private-key",
               cmd="kubectl delete secret github-private-key --ignore-not-found && kubectl create secret generic github-private-key --from-file=key.pem=./bees-broker/github.pem",
               deps="./bees-broker/github.pem",
               labels=["secrets"])

# Spin up database
k8s_yaml("bees-postgres/k8s.yaml")
k8s_resource("bees-postgres", port_forwards=5432, labels=["database"])

# Spin up broker
bees_webhook_port = os.getenv("BEES_WEBHOOK_PORT")
if bees_webhook_port == None:
    fail("BEES_WEBHOOK_PORT must be set.")

docker_build("bees-broker", context=".", dockerfile="bees-broker/Dockerfile")
k8s_yaml("bees-broker/k8s.yaml")
k8s_resource("bees-broker", port_forwards=int(bees_webhook_port), labels="broker")
