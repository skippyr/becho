name=becho
prefix_directory=/usr/local
binaries_directory=${prefix_directory}/bin
manuals_directory=${prefix_directory}/share/man/man1

install:
	cargo build -r
	cp -f target/release/${name} ${binaries_directory}
	mkdir -p ${manuals_directory}
	cp -f manuals/${name}.1 ${manuals_directory}

uninstall:
	rm -f {${binaries_directory}/${name},${manuals_directory}/${name}.1}

.PHONY: install uninstall
