# Sample Bash configuration for testing

# System shortcuts
#@: list files in long format with hidden files :f#
alias ll='ls -lah'
#@: go up one directory :f#
alias ..='cd ..'

# Search aliases
# alf
# search with color highlighting
# fla
alias grep='grep --color=auto'

# Function example
# alf
# extract archives of various formats
# fla
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
