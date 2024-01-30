ASSETS = assets.tar.gz
LOG_GROUP_NAME = /aws/lambda/$(MINERVA_LOG_GROUP)
CH_VERSION=23.11.4.24
CH_TGZ=clickhouse-common-static-$(CH_VERSION)-amd64.tgz
SMITHY_TS=smithy/build/smithy/source/typescript-client-codegen

ui:
	@cd web && yarn dev

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
	@wget https://github.com/ClickHouse/ClickHouse/releases/download/v$(CH_VERSION)-stable/clickhouse-macos-aarch64
	@mv clickhouse-macos-aarch64 clickhouse && chmod +x clickhouse

download-clickhouse-gh:
	@curl -LO https://github.com/ClickHouse/ClickHouse/releases/download/v$(CH_VERSION)-stable/$(CH_TGZ)
	@mkdir -p /tmp/clickhouse_bin
	@tar xzf $(CH_TGZ) -C /tmp/clickhouse_bin --strip-components=1
	@mv /tmp/clickhouse_bin/usr/bin/clickhouse ./clickhouse
	@chmod +x ./clickhouse

lambda-log:
	@aws logs get-log-events --profile $(SANDBOX_PROFILE) --log-group-name $(LOG_GROUP_NAME) --log-stream-name '$(shell aws logs describe-log-streams --profile $(SANDBOX_PROFILE) --log-group-name $(LOG_GROUP_NAME) | jq -r '.logStreams | sort_by(.creationTime) | .[-1].logStreamName')' | jq -r '.events[] | select(has("message")) | ((.timestamp/1000 | strflocaltime("%Y-%m-%dT%H:%M:%S %Z")) + ": " + .message)'

build-ui-smithy-js:
	@cd $(SMITHY_TS); yarn && yarn build
	@cd web; yarn add ../$(SMITHY_TS) && yarn

build-ui:
	@cd web && yarn build

upload-ui: build-ui
	@aws s3 sync --profile $(SANDBOX_PROFILE) web/dist s3://$(MINERVA_WEB_BUCKET)

.PHONY: validate update-smithy build-smithy watch client gen-key build-lambda download-clickhouse lambda-log build-ui-smithy-js build-ui upload-ui
