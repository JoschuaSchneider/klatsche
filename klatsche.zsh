function preexec_check_klatsche() {
  alias | klatsche "$1"
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec preexec_check_klatsche
