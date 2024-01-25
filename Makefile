ASSETS = assets.tar.gz

validate:
	@cd smithy && smithy validate

update-smithy:
	@gh release download -R tyrchen/smithy-assets -p '$(ASSETS)'
	@rm -rf $HOME/.m2
	@tar -xzf $(ASSETS) -C $(HOME) --strip-components=2
	@rm $(ASSETS)

build-smithy:
	@cd smithy && smithy build

watch:
	@watchexec --restart --exts rs -- cargo run --bin echo-server

client:
	@cargo run --bin echo-client

gen-key:
	@openssl genpkey -algorithm ed25519 -out /tmp/sk.pem
	@openssl pkey -in /tmp/sk.pem -pubout -out /tmp/pk.pem

build-lambda:
	@cargo lambda build --release --arm64 --output-format zip
	@cp ~/.target/lambda/minerva-server/bootstrap.zip ../minerva-infra/lambda/ds-api.zip
