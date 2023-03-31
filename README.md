# klatsche

Gives you a ⚡️ blazingly-fast ⚡️ slap on the wrist if you forget to use your terminal aliases.

## Installation

### Prerequisites

- `rust` installed
- `zsh` installed and configured
- optional: `zimfw` setup for use with `zmodule`

### Building
Clone the repository and run:
```bash
cargo build --release
```

### Installing ZSH plugin

#### Install using `zmodule`

Add the following to your `.zimfw` file:

```bash
zmodule /Path/to/klatsche --source klatsche.zsh
```

Run `zimfw install` and restart terminal session.

#### Install using `source`

Add the following to your `.zshrc` file:

```bash
source /Path/to/klatsche/klatsche.zsh
```

Restart terminal session.

## Usage

Type any long form of an alias, you get a *slap*  in the terminal 🎉