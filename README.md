# Chatmat
A chat app written in Rust and Next.js. In this project I want use distributed environment to have a chat application.

## Tech Stack

### Back-end
- Rust
- Tonic
- Actix
### Front-end
- Nexj.js
- TailwindCss
- React-Hook-Form

## Contents
- [Setup](#setup)
- [Todos](#todos)
- [License](#license)

## Setup
1. Install Nix: [installation-guide](https://nixos.org/download/#nix-install-linux)
2. Install Direnv:
   ```sh
   nix-env -i direnv
   # or
   nix-env -iA nixpkgs.direnv
   ```
### Run Backend
```bash
$ make dev-back

# use grpccurl to test
$ grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' 'localhost:9090' helloworld.Greeter/SayHello
{
  "message": "Hello Tonic"
}


# run envoy
make dev-envoy

# send it the message through envoy
$ grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' 'localhost:8080' helloworld.Greeter/SayHello


```
### Run Front
```bash
# npm/yarn install can be issued
pnpm install
pnpm dev
```

## Todos
- [x] Make GitHub Project
- [x] Set up Simple Server
- [ ] Create Schema
- [ ] Login using Google
- [ ] JWT authentication with refresh token
- [ ] Layout for chat UI
- [ ] Create CI/CD
- [ ] End-to-end testing for back & front
- [ ] Unit testing

## License
**This project is licensed under the terms of the MIT license.**
