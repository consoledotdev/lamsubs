# Mailchimp App for LaMetric

An HTTP endpoint for polling Mailchimp subscriber numbers for an [LaMetric
Time](https://lametric.com/en-US/time/overview) device. Designed to run on
Azure Functions uusing a [Custom Request
Handler](https://docs.microsoft.com/en-us/azure/azure-functions/functions-custom-handlers)
which receives HTTP requests and responds like a web app.

## Methods
* `get_subscribers`: returns the latest stats from Mailchimp in JSON format as
  [an LaMetric indicator app](https://lametric-documentation.readthedocs.io/en/latest/guides/first-steps/first-lametric-indicator-app.html#id1).

## Development

Uses [Rocket](https://rocket.rs/) so [requires Rust
nightly](https://rocket.rs/v0.4/guide/getting-started/). Set up a nightly
override on the directory once cloned:

```zsh
rustup override set nightly
```

Set environment variables (see below) with the relevant config then run:

```zsh
cargo run
```

to start the web server. Issue requests to the endpoints e.g.:

```zsh
curl -v http://localhost:3000/api/get_subscribers
```

### Unit tests

```zsh
cargo test
```

### Testing with Azure Functions

Requirements:

* [Azure Functions Core
  Tools](https://docs.microsoft.com/en-us/azure/azure-functions/functions-run-local#v2)

```zsh
# Create the local.settings.json file which will set the environment variables
# Only required on a fresh clone when the file doesn't exist
func azure functionapp fetch-app-settings lamsubs
func start
```

#### Trigger HTTP function

```zsh
curl -v http://localhost:7071/api/get_subscribers
```

## Environment variables

* `LAMSUBS_MAILCHIMP_APIKEY`: Mailchimp API key.
* `LAMSUBS_MAILCHIMP_LIST_ID`: Mailchimp list ID string.
* `LAMSUBS_PRODUCTION`: Set to any value when running in production.

## Deployment

Create a new tag with the name format release-vx.y.z where x.y.z is a semantic
versioning formatted version string e.g. release-v0.1.4

[Azure
uses](https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Clinux#compile-the-custom-handler-for-azure)
the `x86_64-unknown-linux-musl` platform. Builds are done through [a dedicated
Docker container](https://github.com/clux/muslrust) that has various C
libraries built against musl.

Azure resources defined in [`main.bicep`](main.bicep).

### Manual

Requirements:

* Docker e.g. `sudo pacman install docker`
* [Bicep](https://github.com/Azure/bicep/blob/main/docs/installing.md)
* [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli)
* [Azure Functions Core
  Tools](https://docs.microsoft.com/en-us/azure/azure-functions/functions-run-local#v2)

```zsh
docker pull clux/muslrust
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
mkdir bin # host.json configured to expect binary here
cp target/x86_64-unknown-linux-musl/release/lamsubs bin/
bicep build ./main.bicep # generates main.json
az login
az deployment group create -f ./main.json -g lamsubs
func azure functionapp publish lamsubs
```

### Automatic

Uses [Azure
ARM](https://github.com/marketplace/actions/deploy-azure-resource-manager-arm-template)
and [Login](https://github.com/marketplace/actions/azure-login) GitHub actions
to deploy.

`AZURE_CREDENTIALS` created as per [the service principal
instructions](https://github.com/marketplace/actions/azure-login#configure-deployment-credentials).

```zsh
az ad sp create-for-rbac --name "lamsubs - GitHub" --sdk-auth --role contributor \
    --scopes /subscriptions/SUBSCRIPTIONID/resourceGroups/lamsubs
```
