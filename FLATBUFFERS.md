# Install flatc

flatc in homebrew is an outdated version. We need to compile.

```
git clone git@github.com:google/flatbuffers.git
git checkout v22.9.29
CC=/usr/bin/clang CXX=/usr/bin/clang++ cmake -G "Unix Makefiles"
make
sudo make install
which flatc
```