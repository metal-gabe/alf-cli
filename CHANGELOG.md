## [0.4.0](https://github.com/metal-gabe/alf-cli/compare/v0.3.0...v0.4.0) (2026-07-22)


### Features

* **alf-builds:** [no ci] add missing mac & linux release targets; ([2e33fb3](https://github.com/metal-gabe/alf-cli/commit/2e33fb3951e7c7492af40a465fd8fb33833d0ec8))
* **alf-builds:** make updates per Rabbit review feedback; ([c67bde3](https://github.com/metal-gabe/alf-cli/commit/c67bde31790badcd692925f60f0760e55bd60f54))

## [0.3.0](https://github.com/metal-gabe/alf-cli/compare/455e45aeccec6e7df07005e04c56759cf452bfe8...v0.3.0) (2026-07-14)


### Features

* add Enter/Tab selection and shell integration feature ([0eb92b6](https://github.com/metal-gabe/alf-cli/commit/0eb92b62441a13ce1c02b49a3eedd5c7a29b2fe5))
* **enter-tab:** [no ci] fix shell history writing; ([bf7ae4a](https://github.com/metal-gabe/alf-cli/commit/bf7ae4adc37ed7e5dbe3171e67ac355aedd89a23))
* **enter-tab:** [no ci] fix shell integration hooks, add alias expansion config, disable help subcommand; ([54e50ca](https://github.com/metal-gabe/alf-cli/commit/54e50ca9504415e2a4ca8723965ced50af560009))
* **enter-tab:** [no ci] make updates per Rabbit review feedback; ([c0ec6e6](https://github.com/metal-gabe/alf-cli/commit/c0ec6e654b63d5f64d24f469b6760eaa6243843a))
* **enter-tab:** [no ci] remove `shell_keybind` logic & enter search immediately when entering from shell; ([dbb8e63](https://github.com/metal-gabe/alf-cli/commit/dbb8e63e74a3e73e16ba5b450361a1dcb9fd6c7d))
* **enter-tab:** [no ci] write to shell history on execute; ([90f98eb](https://github.com/metal-gabe/alf-cli/commit/90f98eb3bf13e49bbf5abfdbd1d272ef1d403948))
* **layout:** [no ci] add dynamic scroll tracking & improve scrollbar rendering; ([677da1f](https://github.com/metal-gabe/alf-cli/commit/677da1f5eea32d08a97e2c50b188025c59291c9f))
* **layout:** [no ci] add entries header divider; ([b6638c8](https://github.com/metal-gabe/alf-cli/commit/b6638c8e1abb7f8a3f63a915c00fec8a8fb7f053))
* **layout:** [no ci] add entries header; ([8c69b55](https://github.com/metal-gabe/alf-cli/commit/8c69b5552a2e652269448583e212cad43189d20b))
* **layout:** [no ci] add grouping & sorting keybinding logic; ([1c75193](https://github.com/metal-gabe/alf-cli/commit/1c75193ff44abf255b65d6e68cc31b336628bd42))
* **layout:** [no ci] add help modal, improve keybindings and panel navigation; ([a96ffcd](https://github.com/metal-gabe/alf-cli/commit/a96ffcd92a0b4d68d7618d94e662192df5b6789b))
* **layout:** [no ci] add initial grouping & sorting display logic; ([dd4379b](https://github.com/metal-gabe/alf-cli/commit/dd4379b3893672559a1ac152064f9dd539303615))
* **layout:** [no ci] add syntax highlighting for shell script display; ([1f1d019](https://github.com/metal-gabe/alf-cli/commit/1f1d01961c294896eed085d06a6b02b3b476e1d4))
* **layout:** [no ci] implement complete interactive TUI with vim-style navigation; ([ec534e9](https://github.com/metal-gabe/alf-cli/commit/ec534e9d3bd0d40889c41bc377ed0b350e1a189e))
* **layout:** [no ci] update "Help" modal; ([87368c4](https://github.com/metal-gabe/alf-cli/commit/87368c4523e8c074ca3a7d1bf1e38fc8b1e84dc5))
* **layout:** [no ci] update `gg` and `shift-g` to work on the currently active panel; ([c61deb3](https://github.com/metal-gabe/alf-cli/commit/c61deb3b85ba868156f6ac6211517801232c7f46))
* **layout:** [no ci] update help modal content & add dynamic scroll bar; ([197bfc4](https://github.com/metal-gabe/alf-cli/commit/197bfc44b4044cc6636ee7a23e43a226178ae8d5))
* **layout:** [no ci] update help modal content; ([099ed71](https://github.com/metal-gabe/alf-cli/commit/099ed711e941a33e1cacceb8c55092a4b80c3aa9))
* **layout:** [no ci] update imports; ([455e45a](https://github.com/metal-gabe/alf-cli/commit/455e45aeccec6e7df07005e04c56759cf452bfe8))
* **layout:** [no ci] update section name; ([21b9ca9](https://github.com/metal-gabe/alf-cli/commit/21b9ca981d3600dd57cf297f48351f5e3ec777b7))
* **layout:** [no ci] update styling for header & footer labels; ([8526390](https://github.com/metal-gabe/alf-cli/commit/8526390bc8d9baa7e52e3e00d76c710006c06fec))
* **refactor-config:** [no ci] add 'i' keybind for search and show visible entry count in panel; ([954e766](https://github.com/metal-gabe/alf-cli/commit/954e766afc5e68d04b72f5a4aa11042bf5b3f408))
* **refactor-config:** [no ci] add ctrl-j/ctrl-k scroll keybinds in search mode; ([e432641](https://github.com/metal-gabe/alf-cli/commit/e432641f81d4037d41f896dacaff752fbe0b6918))
* **refactor-config:** [no ci] expand ~ and $HOME in shell file paths; ([9577106](https://github.com/metal-gabe/alf-cli/commit/9577106fd8461e191233bdcce308d6eee9f23b1c))
* **refactor-config:** [no ci] implement config system, init wizard, and startup query; ([0c52f00](https://github.com/metal-gabe/alf-cli/commit/0c52f00d5c605215a63f6ba45eb5a6c0802ad76e))
* **refactor-themes:** [no ci] add runtime theme cycling with tj/tk keybindings; ([2975fb1](https://github.com/metal-gabe/alf-cli/commit/2975fb1d2a851a02ede5b8a13412539793297259))
* **release-prep-1:** add CI/CD GitHub Actions workflows; ([1843654](https://github.com/metal-gabe/alf-cli/commit/184365481d1da088773b525eed8b9699cf8504c4))
* **release-prep-1:** add docs link; ([f193627](https://github.com/metal-gabe/alf-cli/commit/f193627a771501375a59c6031ea85cf8c8226eb6))
* **release-prep-1:** add starting CHANGELOG; ([273530f](https://github.com/metal-gabe/alf-cli/commit/273530faa3225a248248729cfb3d59e5e44eff68))
* **release-prep-1:** clean up actions for newer workflows; ([a5712dd](https://github.com/metal-gabe/alf-cli/commit/a5712ddb6dc030e607c3f47090e0690ec98f8cc8))
* **release-prep-1:** update README; ([b71c50d](https://github.com/metal-gabe/alf-cli/commit/b71c50d42459b24141caf4d6fa84b788c6876568))
* **release-prep-1:** update README; ([184ca90](https://github.com/metal-gabe/alf-cli/commit/184ca90646bd15e625b4b699c36885cbcfa64bcb))
* **release-prep-1:** update README; ([6435925](https://github.com/metal-gabe/alf-cli/commit/64359258f939243a025f391099f3c06f30437bd2))
* **release-prep-1:** update README; ([2c8cddd](https://github.com/metal-gabe/alf-cli/commit/2c8cddd9315739573334bbcb89034c4e7ecd90a5))
* **unit-tests-1:** [no ci] replace shell-hook with activate command and alphabetize commands; ([eb51fa9](https://github.com/metal-gabe/alf-cli/commit/eb51fa95472a91e3b34e5d167e1ab373fd1d844b))


### Bug Fixes

* **fix-actions-2:** fix syntax; ([e505fb9](https://github.com/metal-gabe/alf-cli/commit/e505fb9b43833f69c0c0ea9c2a70125d8bd357b1))
* **fix-actions-2:** get main pipeline working?; ([b934275](https://github.com/metal-gabe/alf-cli/commit/b93427555fa661832fbf5d0ce1c7d5dcfacfb9a0))
* **fix-actions-3:** fix potential bug; ([1131db7](https://github.com/metal-gabe/alf-cli/commit/1131db7c68b9ac440ff735162c244ba0415705b5))
* **fix-actions-4:** add release PAT; ([7f457f1](https://github.com/metal-gabe/alf-cli/commit/7f457f1f542cbad303f7458d804d5312e4b82da4))
* **fix-actions-5:** give `main` & `release` proper roles; ([10ff589](https://github.com/metal-gabe/alf-cli/commit/10ff58908669294babaa92349283cdec020b2a55))
* **fix-actions-5:** update git info; ([56c508f](https://github.com/metal-gabe/alf-cli/commit/56c508f4f2837243ebee0380d954b8e80a9c684b))
* **fix-actions-6:** update actions to have proper version bumping logic; ([cf3a6c9](https://github.com/metal-gabe/alf-cli/commit/cf3a6c9b560051e9c9af77e113c061ade8f2b0db))
* **fix-actions:** make updates per Rabbit review feedback; ([b39f72c](https://github.com/metal-gabe/alf-cli/commit/b39f72c2a5bc24ab214d5bf0625da1c959e849ef))
* **fix-actions:** update GH Actions; ([34eb3ab](https://github.com/metal-gabe/alf-cli/commit/34eb3ab2bd85a6980ab656fd30be8396f21b273b))
* **layout:** [no ci] fix UTF-8 handling and improve shell detection; ([45a6d86](https://github.com/metal-gabe/alf-cli/commit/45a6d8611fb0a2280256c5fe59dad9c8a2184730))

