
test:
	cargo test
	java -jar ~/Downloads/batik-1.8/batik-rasterizer-1.8.jar ~/rust/rex/samples/*.svg
