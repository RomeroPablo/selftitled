#!/bin/bash
wasm-pack build --target web --out-dir ./web/pkg
cd web
npx serve .
