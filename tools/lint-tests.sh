#!/bin/sh

dotnet gendarme --config rules.xml --set unit-test Library/ScriptAssemblies/Tests.dll
