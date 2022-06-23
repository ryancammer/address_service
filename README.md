# Address Service

__NOTE__: This was written for an interview question at Lob, where
the question was vague. I don't have a copy of the original question
anymore. Basically, the interviewer didn't like my answer because it
wasn't some RESTful service for an individual's address book. However,
the requirements never specified that an individual address book was
what they were looking for. I think my solution for a generic address 
service is more interesting :)

Address Service provides a service for searching known addresses
using a hint, which then returns a list of addresses. An address
file, addresses.json, provides all of the known addresses. There
are no mechanisms for updating addresses, because if there are
new addresses, they are simply added to addresses.json and the
service is redeployed 
([see cows versus pets](https://www.hava.io/blog/cattle-vs-pets-devops-explained)).

## Installation

Address Service is written in Rust. It uses [Rocket](https://rocket.rs/), 
a Rust web framework that is similar to all of the other standard
web frameworks.

The standard rust installation is explained on the 
[Rust Installation Page](https://www.rust-lang.org/tools/install). However,
Rocket requires the use of Rust nightly builds.
Rocket's [getting started page](https://rocket.rs/v0.4/guide/getting-started/) 
explains how to install the nightly build in
the directory of this project (`rustup override set nightly`).

## Usage

This assumes you have already cloned the 
[Address Service git repository](https://github.com/ryancammer/address_service).

### Local

To build Address Service locally, you'll need to install Rust following the
latest [installation instructions](https://www.rust-lang.org/tools/install).

Inside the project directory, you'll want to enable rust nightly:

```bash
rustup override set nightly
```

Run the Address Service:

```bash
cargo run
```

The Address Service will run on port 8000.

#### Docker Compose

This assumes you have Docker Compose installed. Otherwise, 
[install it first](https://docs.docker.com/compose/install/). The
container image lives in
[docker.io](https://hub.docker.com/repository/docker/ryancammer/address_service).

Run the Address Service using Docker Compose:

```bash
docker-compose up -d
```

The Address Service will run on port 8000.

#### Helm

This assumes you have helm and kubectl installed. Otherwise,
[install helm](https://helm.sh/docs/helm/helm_install/) and
[kubectl](https://kubernetes.io/docs/tasks/tools/).



## License
[MIT](https://choosealicense.com/licenses/mit/)

## TODO
- [ ] Consider ditching the temp file for the index.
- [ ] Come up with a strategy for handling all of the `unwrap()`s.
- [ ] Write a custom build for incorporating the json file, so that
  the addresses.json string doesn't remain in memory.
- [ ] Change the addresses.json file to a zip file to reduce footprint.
- [ ] Delete anything from memory that we don't need.
