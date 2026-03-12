# Radl

An Adelaide aggregator for news and current events

```bash
sudo mkdir /opt/libtorch
sudo curl https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.4.0%2Bcpu.zip -o /tmp/libtorch.zip
sudo unzip /tmp/libtorch.zip -d /tmp/libtorch-tmp
sudo mv /tmp/libtorch-tmp/libtorch /opt/libtorch
sudo rm -rf /tmp/libtorch-tmp /tmp/libtorch.zip

export LIBTORCH=/opt/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
export RUSTFLAGS="-C linker=gcc"
```
