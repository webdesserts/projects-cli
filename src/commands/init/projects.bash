projects() {
  if [ -z "$@" ]; then
    local project=$(projects select)
    [[ ! -z "$project" ]] && cd $project
  else
    eval "projects $@";
  fi
}