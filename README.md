# Certbot DNS Auth Hook

Note: This is an unofficial software that I developed to solve a problem that I have. You're welcome to contribute since it's designed to be extensible.

## Introduction

For [letsencrypt](https://letsencrypt.org/) and its Certbot's challenge of type DNS-01, where a TXT record is required to be added to the DNS zone of the domain, this script automates the process of adding and removing the TXT record.

In Certbot, the `--manual-auth-hook` and `--manual-cleanup-hook` options are used to specify the script to be executed before and after the challenge. This program is designed to be called in these scripts.

I developed this program because I love for my tools to be reusable, testable and to be written in such a way where errors are impossible to happen. I hate writing random bash scripts to do this, so I spent a few hours to create the first version of this.

### Capabilities

This program is very simple by design. It can do two things:

- Add/Remove/List DNS host records using https calls
- Test the implementation of a specific DNS provider automatically (for both testing the health of your setup and for adding new DNS providers)

## Available DNS Providers

I started this for myself, where I use Epik.com, but you're welcome to add more services and add it to the list.

- Epik.com

## Usage

To see the available command line arguments:

```bash
cargo run -- --help
```


This program isn't meant to be installed. I don't mind cloning it from github and running it directly.

You can just clone it and run it with these commands:

### Configuration file

Start by renaming the `config.yaml.example` file to `config.yaml` and edit it to match your setup.

#### What problem does the configuration file solve?

Every service provider has its own way of authenticating and authorizing API calls. This program is designed to be extensible, so you can add more providers. The configuration file is used to specify the provider and the required credentials.

### Test-run

```bash
cargo run -- --test-run
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
5. At this point you're good to start adding configurations in the config.yaml file and test your implementation using `cargo run -- --test-run`. If the tests pass, that means your implementation is correct.

## License

This program is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
