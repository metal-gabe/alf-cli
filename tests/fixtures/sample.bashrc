# Sample Bash configuration for testing

# System shortcuts
alias ll='ls -lah'
alias ..='cd ..'

# Search aliases
alias grep='grep --color=auto'

# Function example
function extract() {
    if [ -f $1 ]; then
        case $1 in
            *.tar.bz2) tar xjf $1 ;;
            *.tar.gz) tar xzf $1 ;;
            *.zip) unzip $1 ;;
            *) echo "'$1' cannot be extracted" ;;
        esac
    fi
}
