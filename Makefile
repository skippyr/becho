name=becho
prefix_directory=/usr/local
binaries_directory=${prefix_directory}/bin
manuals_directory=${prefix_directory}/share/man/man1

install:
	cargo build -r
	sudo cp -f target/release/${name} ${binaries_directory}
	sudo mkdir -p ${manuals_directory}
	sudo cp -f manuals/${name}.1 ${manuals_directory}

uninstall:
	sudo rm -f {${binaries_directory}/${name},${manuals_directory}/${name}.1}

.PHONY: install uninstall
