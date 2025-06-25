# Bees

Bees is a prototype agentic work pool that aims to be able to implement work
defined in GitHub issues, ensure the resulting changes are high-quality, safe,
and correct, then submit the changes in a pull request and incorporate
feedback.

## Getting started

Bees is a containerized system running in Kubernetes. As a prerequisite, you
will need to have a Docker daemon configured on your machine.

```sh
git clone https://github.com/icorbrey/bees.git
cd bees

# This is a Nix-first project. If you don't have Nix installed or don't want to
# use it, you will need to stand up your own Kubernetes cluster and ensure Tilt
# and sqlx-cli are installed.
nix develop

tilt up
```

## License

Bees is distributed under [GPL v3](./LICENSE.md).
