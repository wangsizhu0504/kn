use crate::display::StyledOutput;

pub fn handle(shell: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let shell_name = shell.unwrap_or_else(|| {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/bash".to_string())
            .split('/')
            .last()
            .unwrap_or("bash")
            .to_string()
    });

    StyledOutput::header(&format!("Shell Completion for {}", shell_name));
    println!();

    match shell_name.as_str() {
        "bash" => print_bash_completion(),
        "zsh" => print_zsh_completion(),
        "fish" => print_fish_completion(),
        _ => {
            StyledOutput::error(&format!("Unsupported shell: {}", shell_name));
            StyledOutput::info("Supported shells: bash, zsh, fish");
            return Ok(());
        }
    }

    println!();
    StyledOutput::section_title("Installation Instructions");

    match shell_name.as_str() {
        "bash" => {
            println!("Add to ~/.bashrc:");
            println!("  eval \"$(kn completion bash)\"");
        }
        "zsh" => {
            println!("Add to ~/.zshrc:");
            println!("  eval \"$(kn completion zsh)\"");
        }
        "fish" => {
            println!("Save to ~/.config/fish/completions/kn.fish:");
            println!("  kn completion fish > ~/.config/fish/completions/kn.fish");
        }
        _ => {}
    }

    Ok(())
}

fn print_bash_completion() {
    println!(
        r#"
_kn_completion() {{
    local cur prev commands
    COMPREPLY=()
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"

    commands="install i run r uninstall upgrade update clean-install ci agent list info history alias fuzzy stats parallel clean analyze watch doctor outdated search size completion help"

    if [ $COMP_CWORD -eq 1 ]; then
        COMPREPLY=( $(compgen -W "$commands" -- $cur) )
        return 0
    fi

    case "$prev" in
        run|r)
            # Complete with npm scripts
            if [ -f package.json ]; then
                local scripts=$(node -pe "try {{ Object.keys(require('./package.json').scripts || {{}}).join(' ') }} catch(e) {{ '' }}" 2>/dev/null)
                COMPREPLY=( $(compgen -W "$scripts" -- $cur) )
            fi
            ;;
        install|i|uninstall)
            # Complete with installed packages
            if [ -d node_modules ]; then
                local packages=$(ls node_modules 2>/dev/null | grep -v "^\." | tr '\n' ' ')
                COMPREPLY=( $(compgen -W "$packages" -- $cur) )
            fi
            ;;
        completion)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- $cur) )
            ;;
    esac
}}

complete -F _kn_completion kn
"#
    );
}

fn print_zsh_completion() {
    println!(
        r#"
#compdef kn

_kn() {{
    local -a commands
    commands=(
        'install:Install packages'
        'i:Install packages (alias)'
        'run:Run npm scripts'
        'r:Run npm scripts (alias)'
        'uninstall:Uninstall packages'
        'upgrade:Upgrade dependencies'
        'update:Upgrade dependencies (alias)'
        'clean-install:Clean install'
        'ci:Clean install (alias)'
        'agent:Run package manager'
        'list:List available scripts'
        'info:Show package manager info'
        'history:Show command history'
        'alias:Manage script aliases'
        'fuzzy:Fuzzy search scripts'
        'stats:Show performance statistics'
        'parallel:Run scripts in parallel'
        'clean:Clean node_modules'
        'analyze:Analyze dependencies'
        'watch:Watch mode'
        'doctor:Health check'
        'outdated:Check for updates'
        'search:Search packages'
        'size:Analyze package sizes'
        'completion:Generate shell completion'
        'help:Show help'
    )

    if (( CURRENT == 2 )); then
        _describe 'command' commands
    elif (( CURRENT == 3 )); then
        case "$words[2]" in
            run|r)
                if [[ -f package.json ]]; then
                    local -a scripts
                    scripts=($(node -pe "try {{ Object.keys(require('./package.json').scripts || {{}}).join('\n') }} catch(e) {{ '' }}" 2>/dev/null))
                    _describe 'script' scripts
                fi
                ;;
            completion)
                _values 'shell' 'bash' 'zsh' 'fish'
                ;;
        esac
    fi
}}

_kn "$@"
"#
    );
}

fn print_fish_completion() {
    println!(
        r#"
# kn completion for fish shell

# Main commands
complete -c kn -f -n "__fish_use_subcommand" -a "install" -d "Install packages"
complete -c kn -f -n "__fish_use_subcommand" -a "i" -d "Install packages (alias)"
complete -c kn -f -n "__fish_use_subcommand" -a "run" -d "Run npm scripts"
complete -c kn -f -n "__fish_use_subcommand" -a "r" -d "Run npm scripts (alias)"
complete -c kn -f -n "__fish_use_subcommand" -a "uninstall" -d "Uninstall packages"
complete -c kn -f -n "__fish_use_subcommand" -a "upgrade" -d "Upgrade dependencies"
complete -c kn -f -n "__fish_use_subcommand" -a "update" -d "Upgrade dependencies (alias)"
complete -c kn -f -n "__fish_use_subcommand" -a "clean-install" -d "Clean install"
complete -c kn -f -n "__fish_use_subcommand" -a "ci" -d "Clean install (alias)"
complete -c kn -f -n "__fish_use_subcommand" -a "agent" -d "Run package manager"
complete -c kn -f -n "__fish_use_subcommand" -a "list" -d "List available scripts"
complete -c kn -f -n "__fish_use_subcommand" -a "info" -d "Show package manager info"
complete -c kn -f -n "__fish_use_subcommand" -a "history" -d "Show command history"
complete -c kn -f -n "__fish_use_subcommand" -a "alias" -d "Manage script aliases"
complete -c kn -f -n "__fish_use_subcommand" -a "fuzzy" -d "Fuzzy search scripts"
complete -c kn -f -n "__fish_use_subcommand" -a "stats" -d "Show performance statistics"
complete -c kn -f -n "__fish_use_subcommand" -a "parallel" -d "Run scripts in parallel"
complete -c kn -f -n "__fish_use_subcommand" -a "clean" -d "Clean node_modules"
complete -c kn -f -n "__fish_use_subcommand" -a "analyze" -d "Analyze dependencies"
complete -c kn -f -n "__fish_use_subcommand" -a "watch" -d "Watch mode"
complete -c kn -f -n "__fish_use_subcommand" -a "doctor" -d "Health check"
complete -c kn -f -n "__fish_use_subcommand" -a "outdated" -d "Check for updates"
complete -c kn -f -n "__fish_use_subcommand" -a "search" -d "Search packages"
complete -c kn -f -n "__fish_use_subcommand" -a "size" -d "Analyze package sizes"
complete -c kn -f -n "__fish_use_subcommand" -a "completion" -d "Generate shell completion"
complete -c kn -f -n "__fish_use_subcommand" -a "help" -d "Show help"

# Completion for shell types
complete -c kn -f -n "__fish_seen_subcommand_from completion" -a "bash zsh fish"

# Run command: complete with npm scripts
complete -c kn -f -n "__fish_seen_subcommand_from run r" -a "(test -f package.json; and node -pe 'try {{ Object.keys(require(\"./package.json\").scripts || {{}}).join(\"\\n\") }} catch(e) {{ \"\" }}' 2>/dev/null)"

# Install/uninstall: complete with installed packages
complete -c kn -f -n "__fish_seen_subcommand_from install i uninstall" -a "(test -d node_modules; and ls node_modules 2>/dev/null | grep -v '^\.')"
"#
    );
}
