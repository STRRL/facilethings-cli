#!/usr/bin/env bash
docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v $PWD:/local openapitools/openapi-generator-cli generate \
    -i /local/openapi.yaml \
    --package-name facilethings-openapi \
    -g rust \
    -o /local/facilethings-openapi