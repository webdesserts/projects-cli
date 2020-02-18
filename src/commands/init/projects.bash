projects() {
  if [ -z "$@" ]; then
    local project=$(PROJECT_CLI_PATH select);
    [[ ! -z "$project" ]] && cd "$project";
  else
    eval "PROJECT_CLI_PATH $@";
  fi
};

alias p="projects"
