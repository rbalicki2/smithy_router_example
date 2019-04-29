# smithy_router_example

> An example app that uses Smithy to build a simple hash router

See it in action [here](https://www.smithy.rs/examples/router/)!

## How to install and run locally

Clone the following repositories into subfolders of the same folder:

* `git@github.com:rbalicki2/smithy_router_example.git` (this one)
* `git@github.com:rbalicki2/smithy-app-server`

### Start the app

In the `smithy-app-server` folder, open two terminals and run

```sh
TARGET=../smithy_router_example/ npm run serve
```

and

```sh
TARGET=../smithy_router_example/ npm run watch
```

And now, navigate to `localhost:8080`!
