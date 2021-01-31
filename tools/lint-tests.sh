#!/bin/sh

gendarme --config rules.xml --set unit-test --ignore default.ignore Library/ScriptAssemblies/Tests.dll
