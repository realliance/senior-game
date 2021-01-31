#!/bin/sh

dotnet gendarme --config rules.xml --set unit-test --ignore default.ignore Library/ScriptAssemblies/Tests.dll
