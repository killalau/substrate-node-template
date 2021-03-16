#!/usr/bin/env bash

gcc -o ./lib/test.o -c ./lib/test.c
ar rcs ./lib/libtest.a ./lib/test.o
