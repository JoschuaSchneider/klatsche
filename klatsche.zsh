function preexec_check_klatsche() {
  alias | "${KLATSCHE_HOME}"/target/release/klatsche "$1"
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec preexec_check_klatsche
