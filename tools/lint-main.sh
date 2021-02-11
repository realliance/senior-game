#!/bin/sh

dotnet gendarme --config rules.xml --set default Library/ScriptAssemblies/Networking.dll Library/ScriptAssemblies/UI.dll Library/ScriptAssemblies/Global.dll Library/ScriptAssemblies/WWW.dll
