# Bees

Bees is a prototype agentic work pool that aims to be able to implement work
defined in GitHub issues, ensure the resulting changes are high-quality, safe,
and correct, then submit the changes in a pull request and incorporate
feedback.

## Getting started

Bees is a containerized system running in Kubernetes. If you don't want to
utilize Nix, you will need to stand up your own Docker service and Kubernetes
cluster locally for development.

```sh
git clone https://github.com/icorbrey/bees.git
nix develop
tilt up
```

## License

Bees is distributed under [GPL v3](./LICENSE.md).
