#!/bin/sh

wasm-pack build -t no-modules -d www/martian-engine/ --release
rm www/martian-engine/*.ts

cat www/src/martian_engine.js >> www/martian-engine/martian_engine.js

cp www/src/index.html www/martian-engine/index.html
cp www/src/index.js www/martian-engine/index.js
cp www/src/styles.css www/martian-engine/styles.css

cp -r www/assets www/martian-engine
