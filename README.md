# Certbot DNS Auth Hook

Note: This is an unofficial software that I developed to solve a problem that I had. You're welcome to contribute since it's designed to be extensible.

## Introduction

For [letsencrypt](https://letsencrypt.org/) and its Certbot's challenge of type DNS-01 to sign Wildcard certificates for domains, a TXT record is required to be added to the DNS records of the domain, this script automates the process of adding and removing the TXT record.

In Certbot, the `--manual-auth-hook` and `--manual-cleanup-hook` options are used to specify the script to be executed before and after the challenge. This program is designed to be called in these scripts.

I developed this program because I love for my tools to be reusable, testable and to be written in such a way where errors are impossible to happen. I hate writing random bash scripts to do this with weird curl commands, so I spent a few hours to create the first version of this.

This program also has some extra services for DNS control. The default subcommand is `certbot`, but feel free to try the others. It can be used for any other automation tasks that require DNS control.

### Capabilities

This program is very simple by design. It can do two things:

- Modify DNS records as per certbot requirements through the supported add/remove/list DNS host records using https calls
- Test the implementation of a specific DNS provider automatically (for both testing the health of your setup and for adding new DNS providers)

With the test functionality, you can have a cron job that will daily call this for you to test that your DNS calls are working correctly, and on failure, send you an email to notify you of the error. When the time comes for your certbot renewal, you can be sure that your DNS provider calls are working correctly.

## Available DNS Providers

I started this for myself, where I use Epik.com as domain provider, but you're welcome to add more services and add it to the list.

- Epik.com

## Usage

First step in using this program, is to input your configuration of your domains in the `config.yaml` file. See the [Configuration file](#configuration-file) section for more details. Once you do, you can test your configuration using the `test` command (`cargo run -- test`). See the [Test-run](#test-run) section for more details. If the tests pass, you're ready to use this program with certbot.

This program isn't meant to be installed. I don't mind cloning it from github and running it directly.

In order to use this program with certbot, you can clone the repository, and then add the hook scripts provided as follows, in the directory where you cloned the repository:

```bash
certbot ${MoreArgumentsForYourDomain} --manual-auth-hook ./dns_auth_hook.sh --manual-cleanup-hook dns_cleanup_hook.sh
```

If you'd like to use a proxy, define the environment variable `PROXY_FOR_CERTBOT_DNS_HOOK` to be the proxy URL. For example:

```bash
export PROXY_FOR_CERTBOT_DNS_HOOK=socks5://1.2.3.4:1080
certbot ${MoreArgumentsForYourDomain} --manual-auth-hook ./dns_auth_hook.sh --manual-cleanup-hook dns_cleanup_hook.sh
```

These scripts are in this repository. They are made to be used as is with the source code. Most likely you won't need to change anything in them. You should have [Rust installed](https://www.rust-lang.org/tools/install) so that cargo works.

**SECURITY NOTE**: It is not recommended to run this program as root. This is because cargo downloads dependencies and compiles them. While it's extremely unlikely that any of the dependencies have malicious code, I can't guarantee that for you and I'm paranoid by nature. So, it's better to run this program as a normal user. After all the security trade-offs are up to you.

### Low level usage

To see the available command line arguments:

```bash
cargo run -- certbot --help
```

### Configuration file

Start by renaming the `config.yaml.example` file to `config.yaml` and edit it to match your setup.

#### What problem does the configuration file solve?

Every service provider has its own way of authenticating and authorizing API calls. This program is designed to be extensible, so you can add more providers. The configuration file is used to specify the provider and the required credentials.

### Test-run

```bash
cargo run -- test
```

or with some socks5 proxy (to test your DNS provider from a whitelisted IP):

```bash
cargo run -- test --proxy socks5://1.2.3.4:1080
```

This will test all the domains in the configuration file.

## How to contribute

You're welcome to contribute to add your own DNS providers to use this program as your DNS hook.

#### How to add a new provider? (for developers)

To add new DNS providers, you need to:

1. Add a module in services/ directory, with a struct that represents the provider (let's call it the DNS provider struct). All authentication details + domain name variable should be stored in this struct. (See how epik.rs is implemented). This struct should also implement Serialize/Deserialize traits from serde. so that it can be used in the configuration file.
2. Implement the DomainController trait for the DNS provider struct. (See how epik.rs is implemented). This trait is used to add/remove/list DNS records.
3. Add the DNS provider struct deserialization to the `Config` struct in the config module. (See how epik.rs is implemented). All configurations that are listed must be deserialized into Vec<DNSProvider>, just like it's done for Vec<Epik>.
4. The method `Config::into_domain_controllers()` should be able to add your configuration to the list of domain controllers.
5. At this point you're good to start adding configurations in the config.yaml file and test your implementation using `cargo run -- test` (with or without proxy, depending on your DNS provider configuration and IP whitelisting). If the tests pass, that means your implementation is correct.

## License

This program is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
