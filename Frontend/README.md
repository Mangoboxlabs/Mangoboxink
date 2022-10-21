# Setup mangobox Front-end

## Install `Polkadot JS Extension`

Please install `Polkadot JS Extension` before you start. You can get it from here https://polkadot.js.org/extension/

### Get source code

Please get the code from `git@github.com:Mangoboxlabs/Mangoboxink.git`

```
git clone git@github.com:Mangoboxlabs/Mangoboxink.git
```

## Project setup

cd contracts/interfaces

```
npm install
```

### Compiles and hot-reloads for development

```
npm run serve
```

### Compiles and minifies for production

```
npm run build
```

### docker

```
npm run docker:build
docker run -p8080:80 mangobox/ui1:v1
```

## Way 2: Local Node Test

### Config front-end

Please find the correct address for `src/api/connectContract.js`, and update the correct address in `src/api/connectContract.js`. And replace `src/api/httpConfig.js connectPath` to your connect path.

it should be `ws://127.0.0.1:9900` by default.



### gas

1. Open https://polkadot.js.org/apps/
2. Add localhost path(like `ws://127.0.0.1:9900` ) to local node
3. Use an account(like Alice) to wire money into a `Polkadot JS Extension` wallet

## Way 2: Use the online version front-end test deployed contract

### entrance:

```
https://www.mangobox.io/
```

### gas

1. Open https://polkadot.js.org/apps
2. Add wss://rpc.mangobox.xyz/ to local node
3. Use an account(like Alice) to wire money into a `Polkadot JS Extension` wallet

##### Then

You can use `https://www.mangobox.io/` to create token/multisign wallet. Test Protocol Management.

