p() {
  if [ -z "$@" ]; then
    local project=$(projects)
    [[ ! -z "$project" ]] && cd $project
  else
    eval "projects $@";
  fi
}