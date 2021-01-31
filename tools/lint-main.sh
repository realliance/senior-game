#!/bin/sh

dotnet gendarme --config rules.xml --set default --ignore default.ignore Library/ScriptAssemblies/Networking.dll
