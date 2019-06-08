# orion-holochain

set it up:
[todo]

  * install holochain:

  * generate the keys:
```
hc keygen
```

..* install Nodejs dependecies:

todo:
```
npm install @tap
```



run the tests:

```
hc test
```

#### notes

  * some functions aren't permitted in WASM. They'll throw a runtime error when called from Rust code. For instance:

```
  SystemTime::now().duration_since(UNIX_EPOCH);
```

will
