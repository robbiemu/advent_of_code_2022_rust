.DEFAULT_GOAL := part_1

clean: clean-cargo clean-gcloud
	rm Dockerfile

clean-cargo:
	cargo clean

clean-gcloud:
	gcloud beta code clean-up

part_1:
	cargo build --bin part_1
	cp part_1.dockerfile Dockerfile

part_2:
	cargo build --bin part_2
	cp part_2.dockerfile Dockerfile

local: Dockerfile
	gcloud beta code dev

install:
	gcloud run deploy