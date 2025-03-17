!/usr/bin/bash 
# Lists all projects with a directory pipes it into fzf

cd "$(tracker dirs | awk -F'|' "{ print $2 }" | fzf)"
