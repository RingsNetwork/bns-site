OS = $(shell uname -s)

default:
	make build
	make run
watch:
ifeq ($(OS), Darwin)
	@echo Watching for changes...
	fswatch -0 -e ".*" -i ".rs" ./src | xargs -0 -n 1 -I {} make build
endif

build:
	wasm-pack build --target web --out-name wasm --out-dir ./static
run:
	miniserve ./static --index index.html --port 8964

dev:
	make -j 2 watch ruN

release-build:
	wasm-pack build --release --target web --out-name wasm --out-dir ./static

pubblish:
	cd static && ipfs add -r -w *
