projects() {
  local src="PROJECT_CLI_PATH";
  if [ $# -eq 0 ]; then
    local project=$($src select);
    [[ ! -z "$project" ]] && cd "$project";
  else
    eval "$src" "$@";
  fi
};

alias p="projects";
