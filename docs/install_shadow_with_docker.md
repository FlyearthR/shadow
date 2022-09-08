# Run from the Dockerfile

**Note:** You do not need to follow these steps to run Shadow within Docker.
These steps are an experimental method for creating a Shadow image.

1. Install docker from <https://docs.docker.com/engine/install/>.
2. Build container:
```
git clone https://github.com/shadow/shadow.git
cd shadow
docker build . -t shadow --shm-size="1g"
```
3. Run tests:
```
docker run --tmpfs /dev/shm:rw,nosuid,nodev,exec,size=1024g --privileged --rm --entrypoint /src/ci/container_scripts/test.sh shadow
```

## Run a simulation

To be able to run a simulation you have to mount a volume with the simulation
dependencies (configurations and binaries). This will generate a log directory
owned by `root`.

For example, the next command runs the `shadow.config.yaml` simulation present in
the current path:
```
docker run --tmpfs /dev/shm:rw,nosuid,nodev,exec,size=1024g --privileged --rm --log-driver=none -v $(pwd):/src/ shadow -l debug shadow.config.yaml
```
