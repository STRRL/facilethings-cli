# Rust API client for facilethings-openapi

This is the OpenAPI for `app.facilethings.com`.

For more information, please visit [https://github.com/STRRL/facilethings-cli](https://github.com/STRRL/facilethings-cli)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.1
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `facilethings-openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
facilethings-openapi = { path = "./facilethings-openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.facilethings.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**get_stuff_list**](docs/DefaultApi.md#get_stuff_list) | **GET** /v1/stuff/get_list | 
*DefaultApi* | [**oauth_token**](docs/DefaultApi.md#oauth_token) | **POST** /oauth/token | login with OAuth


## Documentation For Models

 - [OAuthFlowPasswordRequestBody](docs/OAuthFlowPasswordRequestBody.md)
 - [OAuthFlowPasswordResponseBody](docs/OAuthFlowPasswordResponseBody.md)
 - [StuffWrapper](docs/StuffWrapper.md)
 - [StuffWrapperStuff](docs/StuffWrapperStuff.md)
 - [StuffWrapperStuffProject](docs/StuffWrapperStuffProject.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

im@strrl.dev

