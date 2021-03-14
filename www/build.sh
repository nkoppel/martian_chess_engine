#!/bin/sh

wasm-pack build -t no-modules -d www/martian-engine/ --release

cd www/
rm martian-engine/*.ts

cat src/martian_engine.js >> martian-engine/martian_engine.js

cp src/index.html          martian-engine/index.html
cp src/index.js            martian-engine/index.js
cp src/jquery-3.6.0.min.js martian-engine/jquery-3.6.0.min.js
cp src/styles.css          martian-engine/styles.css

mkdir martian-engine/assets
cd martian-engine/assets
../../src/gen_assets.sh
