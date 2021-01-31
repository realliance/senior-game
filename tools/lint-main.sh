#!/bin/sh

dotnet gendarme --config rules.xml --set default Library/ScriptAssemblies/Networking.dll
