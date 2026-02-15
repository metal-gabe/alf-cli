# Sorting and Grouping Keybindings in Terminal Tools

Research into keybinding patterns used for sorting and grouping operations across popular terminal-based tools.

## Summary of Patterns

### Common Patterns Identified

1. **`s` key family**: Most common for sort operations
2. **`o` key**: Used in ranger and similar tools (mnemonic: "order")
3. **`z` prefix**: Used for view/display toggles (yazi, ranger)
4. **Function keys**: F6-F9 commonly used for sorting in system monitors
5. **No native sorting**: vim, fzf lack built-in sort keybinds (manual commands only)

### Modifiers
- **Uppercase letter**: Often reverses sort order (e.g., `on` vs `oN` in ranger)
- **Shift key**: Typically reverses direction or toggles
- **Ctrl key**: Less common for sorting, more for system operations

---

## 1. vim/neovim

### Sorting Support
**No built-in keybindings** for sorting operations.

### Manual Sorting
- `:sort` - Sort lines in visual selection
- `:sort!` - Sort in reverse order  
- `:sort u` - Sort and remove duplicates
- `:sort n` - Numeric sort

### Grouping/View Modes
- No native view mode cycling
- Folding operations use `z` prefix: `za`, `zc`, `zo`, `zM`, `zR`

### Notes
- Vim philosophy: Commands > keybinds for complex operations
- Users typically create custom mappings for frequently used sorts
- External plugins provide more sophisticated sorting

---

## 2. yazi (file manager)

### Sorting Keybindings
**Primary command**: `sort [by]`

**Key sequences** (two-key combinations starting with `o`):
- `oa` - Sort **a**lphabetically
- `ob` - Sort by **b**irth time (btime)
- `oc` - Sort by **c**time (change time)  
- `oe` - Sort by **e**xtension
- `om` - Sort by **m**time (modified time)
- `on` - Sort **n**aturally (1.md < 2.md < 10.md)
- `os` - Sort by **s**ize
- `ot` - Sort by file **t**ype
- `oz` - Sort by random (**z** for random)

### Sort Modifiers
- `or` - **R**everse order (toggle)
- `od` - **D**irectories first (toggle)
- Uppercase second letter reverses: `oN` reverses natural sort

### Additional Flags
- `--reverse` - Display in reverse order
- `--dir-first` - Display directories first
- `--translit` - Transliterate for sorting

### Linemode (Display Mode)
**Command**: `linemode [mode]`

**Hotkey**: `M` (uppercase) opens menu

Available modes:
- `none` - No line mode
- `size` - Show file size
- `mtime` - Show modified time
- `btime` - Show birth time
- `permissions` - Show permissions (Unix)
- `owner` - Show owner (Unix)

### Pattern
- **Two-key sequences** starting with `o` (mnemonic: "order")
- **Mnemonic second key** matches sort criterion
- **Toggle, not cycle** - each key sequence is its own toggle
- **Uppercase reverses** - `on` vs `oN`

---

## 3. ranger (file manager)

### Sorting Keybindings
**Very similar to yazi** (ranger was the inspiration for yazi)

**Key sequences** starting with `o`:
- `on` - Sort **n**aturally
- `ob` - Sort by **b**asename  
- `os` - Sort by **s**ize
- `ot` - Sort by **t**ype
- `oe` - Sort by **e**xtension
- `om` - Sort by **m**time
- `oc` - Sort by **c**time
- `oa` - Sort by **a**time

### Sort Modifiers
- `or` - **R**everse order
- Uppercase second letter reverses order

### Linemode
**Hotkey**: `M` (single key press cycles through modes)

Modes:
- `filename` - Display basename and size
- `permissions` - Display permissions, owner, group
- `mtime` - Display modification time
- `fileinfo` - Display file(1) output

### Pattern
- **Two-key sequences** with `o` prefix
- **Uppercase reverses** sort direction
- **Mnemonic keys** for sort criteria
- **Single `M` key** for linemode cycling

---

## 4. lf (file manager)

### Sorting Keybindings
**Command-based**, not default keybinds (must configure manually)

**Command**: `set sortby [criterion]`

Available criteria:
- `natural` - Natural sort
- `name` - Alphabetical by name
- `size` - By file size
- `time` - By modification time
- `atime` - By access time
- `ctime` - By change time  
- `ext` - By extension

### Additional Settings
- `set reverse` - Reverse sort order
- `set dirfirst` - Directories first
- `set hidden` - Show/hide hidden files
- `set info` - Set info display (size, time, etc.)

### Example Custom Keybinds
Users typically configure their own:
```
map sn :set sortby natural
map ss :set sortby size
map st :set sortby time
```

### Pattern
- **No default sort keybindings**
- **Command-based** configuration
- Users create **custom `s`-prefix mappings**
- Follows set/toggle pattern

---

## 5. fzf (fuzzy finder)

### Sorting Keybindings
**No built-in keybindings** for toggling sort.

### Sort Behavior
- Default: **Score-based** (relevance to query)
- Can specify `--tac` to reverse input order
- Can use `--sort` flag to change algorithm

### Keybindings (navigation only)
- `Ctrl-J`/`Ctrl-K` - Navigate down/up
- `Ctrl-N`/`Ctrl-P` - Navigate down/up (alternate)
- `Alt-J`/`Alt-K` - Navigate preview down/up
- `Tab`/`Shift-Tab` - Select/deselect + move

### Pattern
- **No sort keybindings** - sort is query-driven
- Focus on **navigation** and **selection**
- Can bind custom actions via `--bind`

---

## 6. lazygit

### Sorting/Grouping
**Context-dependent** - different views have different sort options.

### Commits View
- **No universal sort keybind**
- Commits shown chronologically by default
- Can filter/search with `/`

### Files View  
- Files grouped by status (staged, unstaged, etc.)
- **No manual sort keybinding**
- Uses **panels** for grouping (files, branches, commits, stash)

### Branches View
- Sorted by recency by default
- Can toggle between local/remote with tabs

### Navigation
- `j`/`k` - Move down/up
- `h`/`l` - Move between panels
- `[`/`]` - Previous/next tab
- Numbers `1`-`5` - Jump to panel

### Pattern
- **No explicit sort keybindings**
- **Panel-based grouping** (spatial organization)
- **Context-driven** - each view has its own logic
- Focus on **panel navigation** over manual sorting

---

## 7. htop/btop (process monitors)

### htop Sorting
**Function keys** for column sorting:

- `F6` or `>` - Sort by column (opens menu)
- `F5` - Tree view toggle
- `<`/`>` - Change sort column
- `I` - Invert sort order

**Column selection**:
- Interactive menu via `F6`
- Direct column letters when menu open
- Default: sort by CPU%

### btop Sorting
Similar to htop:

- `s` - Open sort menu
- `e` - Toggle **e**xpanded view
- Arrow keys to select sort column
- `r` - **R**everse sort

### Pattern
- **Function keys** (F5, F6) for menus
- **Single letter toggles** for common operations
- **Interactive menus** for sort column selection
- **`>` and `<`** for column navigation
- **`r` or `I`** for reverse

---

## Key Patterns Summary

| Tool | Primary Key | Type | Reverse | Notes |
|------|-------------|------|---------|-------|
| **vim** | `:sort` | Command | `!` flag | No keybinds |
| **yazi** | `o` + letter | Cycle | Uppercase | Two-key mnemonic |
| **ranger** | `o` + letter | Cycle | Uppercase | Two-key mnemonic |
| **lf** | Custom | Command | `set reverse` | User-configured |
| **fzf** | None | Score-based | `--tac` | Query-driven |
| **lazygit** | Context | Panel-based | N/A | No manual sort |
| **htop** | `F6`, `>`, `<` | Menu/Direct | `I` | Column-based |
| **btop** | `s` | Menu | `r` | Column-based |

---

## Recommendations for `alf`

Based on the research, here are keybinding recommendations:

### Primary Sorting
**Recommended**: `s` key family (most intuitive)

- `sn` - Sort **n**aturally (or by **n**ame)
- `st` - Sort by **t**ype
- `sd` - Sort by **d**escription  
- `sr` - **R**everse sort order (toggle)
- `ss` - Cycle through **s**ort modes

**Alternative**: `o` key family (ranger/yazi pattern)

- `on` - Sort naturally
- `ot` - Sort by type
- `od` - Sort by description
- `or` - Reverse order

### Grouping/View Modes
**Recommended**: Single key toggles

- `g` or `v` - Cycle through view/grouping modes
- `G` (Shift-g) - Reverse cycle through modes
- `z` prefix - Toggle display options (ranger/yazi pattern)
  - `zg` - Toggle grouping
  - `zd` - Toggle description visibility

### Filtering
- `/` - Enter filter mode (universal pattern)
- `f` - Quick filter (lf/ranger pattern)
- `Esc` or `Ctrl-C` - Clear filter

### Benefits of These Choices

1. **`s` for sort**
   - Mnemonic and widely understood
   - Doesn't conflict with common vim bindings
   - Two-key sequences allow multiple sort options
   - btop uses `s`, ranger/yazi use `o` (both valid)

2. **Single-key group toggle**
   - Fast and efficient
   - Shift variant allows reverse cycling
   - Clear visual feedback needed

3. **`/` for filter**
   - Universal pattern (vim, less, ranger, etc.)
   - Users expect this behavior
   - Could use `f` as quicker alternative

4. **Modifiers**
   - Uppercase reverses (ranger/yazi pattern)
   - `r` suffix for reverse (btop/general pattern)
   - Both are valid - pick one for consistency

### Example Keybinding Scheme

```
# Sorting (two-key sequences)
sn - Sort naturally (default)
st - Sort by type (alias/function)
sd - Sort by description
sr - Reverse sort (toggle)

# Grouping (single keys)
g - Cycle grouping mode (none -> type -> tag -> custom)
G - Reverse cycle grouping

# Display toggles
zh - Toggle hidden aliases
zd - Toggle description preview
zi - Toggle icons (if implemented)

# Filtering
/ - Enter filter mode
f - Quick filter prompt
Esc - Clear filter

# Navigation (vim-like)
j/k - Down/up
h/l - Left/right (if multi-column)
gg - Top
G - Bottom
```

---

## Accessibility Notes

- **Consistency**: Stick to one pattern (either `o` or `s`, not both)
- **Discoverability**: Show hints in status line or help screen
- **Confirmation**: Visual feedback when sort/group changes
- **Help**: `?` key should show all keybindings (universal pattern)
- **Mouse**: Consider mouse support for sort headers (btop pattern)

---

## Sources

1. vim - `man vim`, vim documentation
2. yazi - https://yazi-rs.github.io/docs/configuration/keymap
3. ranger - https://man.archlinux.org/man/ranger.1
4. lf - https://github.com/gokcehan/lf/blob/master/doc.md
5. fzf - https://github.com/junegunn/fzf
6. lazygit - https://github.com/jesseduffield/lazygit
7. htop - htop man page, in-app help
8. btop - btop documentation

---

**Date**: 2026-02-15  
**Researcher**: Claude (Anthropic)  
**Purpose**: Keybinding research for `alf` CLI TUI
