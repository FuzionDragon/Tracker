!/usr/bin/bash 

cd "$(tracker hooked | awk -F'|' "{ print $2 }")"
