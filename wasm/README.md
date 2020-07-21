A npm bundle of [simple-html-template](https://github.com/dakom/simple-html-template)

Doesn't export all the caching functionality, rather it provides an inline function. 

Basically the intent of this lib is to serve as development interop between js and rust, not runtime production use in js.

See the typescript types for comments and signatures. Example:

```javascript
import {render_template} from "simple-html-template-wasm";

const template = "<h1>Hello ${target}!</h1>";

const output = render_template(template, {target: "<b>world</b>"});
```
