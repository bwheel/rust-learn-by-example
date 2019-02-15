#!/usr/bin/env bash
watch -n 0.5 "rustc $1.rs &&./$1"
