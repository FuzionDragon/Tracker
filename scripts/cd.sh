!/usr/bin/bash 

cd "$(tracker marked | awk -F'|' "{ print $2 }") | fzf"
