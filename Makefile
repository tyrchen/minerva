ASSETS = assets.tar.gz
LOG_GROUP_NAME = /aws/lambda/ds-api-f801bed

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

download-clickhouse:
	@wget https://github.com/ClickHouse/ClickHouse/releases/download/v23.11.4.24-stable/clickhouse-macos-aarch64
	@mv clickhouse-macos-aarch64 clickhouse && chmod +x clickhouse

lambda-log:
	@aws logs get-log-events --profile $(SANDBOX_PROFILE) --log-group-name $(LOG_GROUP_NAME) --log-stream-name '$(shell aws logs describe-log-streams --profile $(SANDBOX_PROFILE) --log-group-name $(LOG_GROUP_NAME) | jq -r '.logStreams | sort_by(.creationTime) | .[-1].logStreamName')' | jq -r '.events[] | select(has("message")) | ((.timestamp/1000 | strflocaltime("%Y-%m-%dT%H:%M:%S %Z")) + ": " + .message)'

.PHONY: validate update-smithy build-smithy watch client gen-key build-lambda
