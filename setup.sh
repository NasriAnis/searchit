mkdir -p deps && cd deps

# Fetching deps
git clone https://github.com/ArtifexSoftware/mupdf.git && cd mupdf && git submodule update --init --depth 1 && make -j$(nproc)