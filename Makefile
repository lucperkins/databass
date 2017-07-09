PROTOBUF_VERSION = 3.3.0

docs:
	cargo doc && open target/doc/lib/index.html

install-protobuf:
	curl -sL https://github.com/google/protobuf/releases/download/v${PROTOBUF_VERSION}/protobuf-cpp-${PROTOBUF_VERSION}.tar.gz | tar xz
	(cd protobuf-${PROTOBUF_VERSION} && ./configure --prefix=/home/travis && make -j2 && make install)