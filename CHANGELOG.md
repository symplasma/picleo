# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.8 (2025-07-26)

### Documentation

 - <csr-id-7914b0c8069095e5c1aa08ced2fec0c4ba2beb99/> Add TODO and autocomplete

### New Features

 - <csr-id-843d3a855031648dce27dea1771ad75e1cadf2a8/> add `editable` flag to control edit mode in Picker
 - <csr-id-22c452214e3b16c644e6212aa5113231b2489819/> add `--keep-colors` CLI flag to control ANSI color rendering in preview

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add `editable` flag to control edit mode in Picker ([`843d3a8`](https://github.com/symplasma/picleo/commit/843d3a855031648dce27dea1771ad75e1cadf2a8))
    - Add `--keep-colors` CLI flag to control ANSI color rendering in preview ([`22c4522`](https://github.com/symplasma/picleo/commit/22c452214e3b16c644e6212aa5113231b2489819))
    - Add TODO and autocomplete ([`7914b0c`](https://github.com/symplasma/picleo/commit/7914b0c8069095e5c1aa08ced2fec0c4ba2beb99))
</details>

## v0.1.7 (2025-07-25)

<csr-id-e78c4e14ef26ecbda9737fdfadc5fe86fa309e2c/>
<csr-id-548afc8272921ffdac2e7d52878986f0a8893efa/>
<csr-id-62225ec1c8f1941e396542f4560f7dfaa2d863eb/>
<csr-id-1990e8748c072c9f0739be02d491cda81e1eed0a/>
<csr-id-2a793aa894c159b235987f8f685cf75f8c678c5e/>
<csr-id-764a55e3b3b7a8e06d56d1622683df26430308d3/>
<csr-id-04e5b3b72faf6756383cd82a2b1b9ae79198a397/>
<csr-id-d480ed53dee5989bcf516b268bcdb5745d831492/>
<csr-id-f6a2edb3acedf03609cc8d8c52600d1a3322370d/>

### Chore

 - <csr-id-e78c4e14ef26ecbda9737fdfadc5fe86fa309e2c/> Remove unecessary use

### Documentation

 - <csr-id-0cf9ca1e9ad09a21d66d323f201799ba918425c3/> Update status, features, and link
 - <csr-id-7845f3ca9a4d306e5c3542634fc4dc212ec35e0d/> Add preview command design
 - <csr-id-df095d4f9a46a62921b3118bbc27ad2ad4ace525/> Update readme

### New Features

 - <csr-id-f9ced82a2c271b578fc518c3ba1f80de806cec03/> Sanitize preview command output
 - <csr-id-76abdf6c33f7c371b2f62fece080193e8afc55ba/> Add shell escaping to preview command placeholders
 - <csr-id-2b183d4a2b2a2ac9f531af6c691b66af4201b69e/> add preview command support with placeholders
 - <csr-id-50a2aba713ac9409a71439a23607aab758c4f922/> support file and directory input with dynamic picker type

### Bug Fixes

 - <csr-id-60b087c4c8016a42fcd102879b86562f46e30ad8/> resolve shell command execution and import order
 - <csr-id-7181e10c782e1461bc8b996450a5632b2cb0cf67/> change shell from sh to zsh for command execution

### Refactor

 - <csr-id-548afc8272921ffdac2e7d52878986f0a8893efa/> Better handling of command args
 - <csr-id-62225ec1c8f1941e396542f4560f7dfaa2d863eb/> Finish moving methods and structs
   Moving the code around as the main file was getting way too long and it was getting hard to find things in the mess. Also, AI tends to mirror the structure of the existing code. We don't need it to make more mess.
 - <csr-id-1990e8748c072c9f0739be02d491cda81e1eed0a/> Split Search and Editing modes
 - <csr-id-2a793aa894c159b235987f8f685cf75f8c678c5e/> Event dispatch cleanup
 - <csr-id-764a55e3b3b7a8e06d56d1622683df26430308d3/> Improve redraw and preview logic
 - <csr-id-04e5b3b72faf6756383cd82a2b1b9ae79198a397/> Clean up conditions
 - <csr-id-d480ed53dee5989bcf516b268bcdb5745d831492/> execute preview command directly without shell
 - <csr-id-f6a2edb3acedf03609cc8d8c52600d1a3322370d/> Refactoring main to simplify code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 16 calendar days.
 - 17 days passed between releases.
 - 18 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.7 ([`0ea571d`](https://github.com/symplasma/picleo/commit/0ea571d860da9a60e849cf3055ad52509c83876e))
    - Update status, features, and link ([`0cf9ca1`](https://github.com/symplasma/picleo/commit/0cf9ca1e9ad09a21d66d323f201799ba918425c3))
    - Sanitize preview command output ([`f9ced82`](https://github.com/symplasma/picleo/commit/f9ced82a2c271b578fc518c3ba1f80de806cec03))
    - Better handling of command args ([`548afc8`](https://github.com/symplasma/picleo/commit/548afc8272921ffdac2e7d52878986f0a8893efa))
    - Finish moving methods and structs ([`62225ec`](https://github.com/symplasma/picleo/commit/62225ec1c8f1941e396542f4560f7dfaa2d863eb))
    - Split Search and Editing modes ([`1990e87`](https://github.com/symplasma/picleo/commit/1990e8748c072c9f0739be02d491cda81e1eed0a))
    - Event dispatch cleanup ([`2a793aa`](https://github.com/symplasma/picleo/commit/2a793aa894c159b235987f8f685cf75f8c678c5e))
    - Add todo comments ([`35b44ce`](https://github.com/symplasma/picleo/commit/35b44ce6e0c2358c94542023638ba96e115df6e7))
    - Improve redraw and preview logic ([`764a55e`](https://github.com/symplasma/picleo/commit/764a55e3b3b7a8e06d56d1622683df26430308d3))
    - Clean up conditions ([`04e5b3b`](https://github.com/symplasma/picleo/commit/04e5b3b72faf6756383cd82a2b1b9ae79198a397))
    - Execute preview command directly without shell ([`d480ed5`](https://github.com/symplasma/picleo/commit/d480ed53dee5989bcf516b268bcdb5745d831492))
    - Resolve shell command execution and import order ([`60b087c`](https://github.com/symplasma/picleo/commit/60b087c4c8016a42fcd102879b86562f46e30ad8))
    - Add shell escaping to preview command placeholders ([`76abdf6`](https://github.com/symplasma/picleo/commit/76abdf6c33f7c371b2f62fece080193e8afc55ba))
    - Change shell from sh to zsh for command execution ([`7181e10`](https://github.com/symplasma/picleo/commit/7181e10c782e1461bc8b996450a5632b2cb0cf67))
    - Add preview command support with placeholders ([`2b183d4`](https://github.com/symplasma/picleo/commit/2b183d4a2b2a2ac9f531af6c691b66af4201b69e))
    - Add preview command design ([`7845f3c`](https://github.com/symplasma/picleo/commit/7845f3ca9a4d306e5c3542634fc4dc212ec35e0d))
    - Update readme ([`df095d4`](https://github.com/symplasma/picleo/commit/df095d4f9a46a62921b3118bbc27ad2ad4ace525))
    - Refactoring main to simplify code ([`f6a2edb`](https://github.com/symplasma/picleo/commit/f6a2edb3acedf03609cc8d8c52600d1a3322370d))
    - Remove unecessary use ([`e78c4e1`](https://github.com/symplasma/picleo/commit/e78c4e14ef26ecbda9737fdfadc5fe86fa309e2c))
    - Support file and directory input with dynamic picker type ([`50a2aba`](https://github.com/symplasma/picleo/commit/50a2aba713ac9409a71439a23607aab758c4f922))
</details>

## v0.1.6 (2025-07-07)

<csr-id-b4f914493a3830d121464de08a53fd99b561dfa5/>
<csr-id-7b9b90c3134c401e132c224c4f160b9c8754ae7d/>
<csr-id-a9147753f1fc154543bec97216477eafc114670b/>
<csr-id-0f3366ac433473a8931802b02f6060d54cd3b9f4/>
<csr-id-eb2e9067ff49462afb06b07ee876e18035998f49/>

### Chore

 - <csr-id-b4f914493a3830d121464de08a53fd99b561dfa5/> Make new items selected by default
 - <csr-id-7b9b90c3134c401e132c224c4f160b9c8754ae7d/> Run cargo fmt

### Documentation

 - <csr-id-a6b4765cde5340a53133b2b7f8d9ff336d682990/> Update README

### New Features

 - <csr-id-db68bbf9456837c6df42d96a518c8eb213192d6d/> Add item editing and creation
 - <csr-id-567824681a9de1e538607e15ae20c7ff661576af/> add item editing mode with Ctrl+n, Enter, and Esc support
 - <csr-id-ee35a5af83de37329e424bd8564791833cc51c09/> Switch the duplicate key to Ctrl-d
 - <csr-id-ed2b381926b816a9398c6a7c9b659a98823b82ca/> Add `SelectedItems` struct with methods to extract selected item values
 - <csr-id-234a88ef0b5d200d8e00af503b08da0d583c7e04/> add Ctrl+n functionality to create new requested item from selected
 - <csr-id-667c2a32ff7b771c6ddf34613168ea95631bf016/> add page navigation with Shift/Control modifier on mouse scroll

### Bug Fixes

 - <csr-id-aa51dcb2ebb76de61f97141a203aba6b81e62f4d/> update iteration over SelectedItems to use existing_values() and requested_values()

### Refactor

 - <csr-id-a9147753f1fc154543bec97216477eafc114670b/> Refactor search mode event handling
   This is partly to help the AI assistant. It's last go at adding an editing mode wasn't bad but it was very WET code.
 - <csr-id-0f3366ac433473a8931802b02f6060d54cd3b9f4/> Convert SelectableItem struct to enum with Existing and Requested variants

### Style

 - <csr-id-eb2e9067ff49462afb06b07ee876e18035998f49/> format mouse scroll event handling with improved readability

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.6 ([`8bc912e`](https://github.com/symplasma/picleo/commit/8bc912ecb88d1c90b4b0e700c5a8ab4e048dce64))
    - Update README ([`a6b4765`](https://github.com/symplasma/picleo/commit/a6b4765cde5340a53133b2b7f8d9ff336d682990))
    - Add item editing and creation ([`db68bbf`](https://github.com/symplasma/picleo/commit/db68bbf9456837c6df42d96a518c8eb213192d6d))
    - Make new items selected by default ([`b4f9144`](https://github.com/symplasma/picleo/commit/b4f914493a3830d121464de08a53fd99b561dfa5))
    - Add item editing mode with Ctrl+n, Enter, and Esc support ([`5678246`](https://github.com/symplasma/picleo/commit/567824681a9de1e538607e15ae20c7ff661576af))
    - Refactor search mode event handling ([`a914775`](https://github.com/symplasma/picleo/commit/a9147753f1fc154543bec97216477eafc114670b))
    - Switch the duplicate key to Ctrl-d ([`ee35a5a`](https://github.com/symplasma/picleo/commit/ee35a5af83de37329e424bd8564791833cc51c09))
    - Update iteration over SelectedItems to use existing_values() and requested_values() ([`aa51dcb`](https://github.com/symplasma/picleo/commit/aa51dcb2ebb76de61f97141a203aba6b81e62f4d))
    - Add `SelectedItems` struct with methods to extract selected item values ([`ed2b381`](https://github.com/symplasma/picleo/commit/ed2b381926b816a9398c6a7c9b659a98823b82ca))
    - Run cargo fmt ([`7b9b90c`](https://github.com/symplasma/picleo/commit/7b9b90c3134c401e132c224c4f160b9c8754ae7d))
    - Add Ctrl+n functionality to create new requested item from selected ([`234a88e`](https://github.com/symplasma/picleo/commit/234a88ef0b5d200d8e00af503b08da0d583c7e04))
    - Convert SelectableItem struct to enum with Existing and Requested variants ([`0f3366a`](https://github.com/symplasma/picleo/commit/0f3366ac433473a8931802b02f6060d54cd3b9f4))
    - Format mouse scroll event handling with improved readability ([`eb2e906`](https://github.com/symplasma/picleo/commit/eb2e9067ff49462afb06b07ee876e18035998f49))
    - Add page navigation with Shift/Control modifier on mouse scroll ([`667c2a3`](https://github.com/symplasma/picleo/commit/667c2a32ff7b771c6ddf34613168ea95631bf016))
</details>

## v0.1.5 (2025-07-07)

<csr-id-90103a3ed35a83eaed9c21b5c9e09d69f5742061/>

### Chore

 - <csr-id-90103a3ed35a83eaed9c21b5c9e09d69f5742061/> Clean up some AI changes

### Documentation

 - <csr-id-be931272f14dba9c7e1d23c69e7055d85ee67978/> Add features to list
 - <csr-id-97e1f3ed3a5fc327d7f2ee50a4e901616cf235d8/> Add usage and other info

### New Features

 - <csr-id-778f22fdca84ed1f01390db58df96c68df7c0364/> add mouse click support to toggle item selection
 - <csr-id-8f516a0f42d9f65fb979b67c2f2bb05b5347e7de/> Escape clears query, then quits
 - <csr-id-cd98d76040b734d326762ecd5945536bde527033/> Improve layout
 - <csr-id-c33165347b2570cdee1f1b543aa081bf2eba8763/> Add `invert_scroll` config option to control mouse scroll direction
 - <csr-id-45963e6078604e3c33bbfe584f0c711e7cd60d18/> Add figment-based configuration system with platform-appropriate config file support
 - <csr-id-30ba8800cae9d8d4cee68e36089eb9fa4effcf18/> add middle-click support to toggle item selection
 - <csr-id-3c46cae2cdd2316250875588b321d596b5a14131/> add mouse scroll support for picker navigation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.5 ([`c9e5448`](https://github.com/symplasma/picleo/commit/c9e54480929db941855b730a4aa2d75ad538579f))
    - Add mouse click support to toggle item selection ([`778f22f`](https://github.com/symplasma/picleo/commit/778f22fdca84ed1f01390db58df96c68df7c0364))
    - Escape clears query, then quits ([`8f516a0`](https://github.com/symplasma/picleo/commit/8f516a0f42d9f65fb979b67c2f2bb05b5347e7de))
    - Add features to list ([`be93127`](https://github.com/symplasma/picleo/commit/be931272f14dba9c7e1d23c69e7055d85ee67978))
    - Improve layout ([`cd98d76`](https://github.com/symplasma/picleo/commit/cd98d76040b734d326762ecd5945536bde527033))
    - Add usage and other info ([`97e1f3e`](https://github.com/symplasma/picleo/commit/97e1f3ed3a5fc327d7f2ee50a4e901616cf235d8))
    - Clean up some AI changes ([`90103a3`](https://github.com/symplasma/picleo/commit/90103a3ed35a83eaed9c21b5c9e09d69f5742061))
    - Add `invert_scroll` config option to control mouse scroll direction ([`c331653`](https://github.com/symplasma/picleo/commit/c33165347b2570cdee1f1b543aa081bf2eba8763))
    - Add figment-based configuration system with platform-appropriate config file support ([`45963e6`](https://github.com/symplasma/picleo/commit/45963e6078604e3c33bbfe584f0c711e7cd60d18))
    - Add middle-click support to toggle item selection ([`30ba880`](https://github.com/symplasma/picleo/commit/30ba8800cae9d8d4cee68e36089eb9fa4effcf18))
    - Add mouse scroll support for picker navigation ([`3c46cae`](https://github.com/symplasma/picleo/commit/3c46cae2cdd2316250875588b321d596b5a14131))
</details>

## v0.1.4 (2025-07-06)

<csr-id-30d47743a2f274727b93e36749c9871a8d32d974/>
<csr-id-84a390c9a497199e5d0ebd85de83a975d199a3a8/>
<csr-id-472bbe20a890566b4440cd31e3807d5e1546b7ac/>
<csr-id-5a70a3672df309ccab090790b72d839011c5ded1/>
<csr-id-a0138182ce82cc323a7fa5d94eb9cb0aceca8234/>
<csr-id-54bdce889d04cefa624081b882c64451f5a2f840/>
<csr-id-a5c49136fd0ea877ebbce60c3392cb0cc51eef19/>

### Chore

 - <csr-id-30d47743a2f274727b93e36749c9871a8d32d974/> Add index debugging indicators
 - <csr-id-84a390c9a497199e5d0ebd85de83a975d199a3a8/> Call join_finished_threads in tick
 - <csr-id-472bbe20a890566b4440cd31e3807d5e1546b7ac/> Add necessary move keyword

### New Features

 - <csr-id-a8e53a50a718c654de32f5c43655d6617fcc49d5/> Add "no items found" screen
   This also prevents a crash that seems to be due to the RangeInclusive and how it interacts with Nucleo when there are no matches.
 - <csr-id-ea5b80881346f5229d653494dbfa0cdf62760380/> Add sliding item window
   This is a first try. Still not working totally reliably just yet.
 - <csr-id-345b843f25f804c6b4f39560d0d7bfd05bf671f4/> Improve redraw logic
 - <csr-id-c756ecee3dad3321e760305daac3df4e30ee4d26/> Add polling to improve responsiveness
   We now redraw the screen even when there is no user input. This can probably be made more efficient though.
 - <csr-id-799d1aaa936ca62790fb8504bdd140839d3ebcc5/> Add matched items
 - <csr-id-0df432b122a024e6d613a1ff95c6d8301de64f6e/> Add item count
 - <csr-id-bb0ea943c36c8b142db2d3e308316069f04166e5/> Add running thread count
 - <csr-id-8981ad967a8c4dfddf7eec92d6da35888c3516af/> Add running_threads method
 - <csr-id-f580bd4727608f373472b581a162757f1c5133a1/> add join_finished_threads method to Picker to manage thread handles
 - <csr-id-50ebd68a4f43623c217b68033a7f7e9062ab8521/> add threaded flag to enable threaded item injection
 - <csr-id-fee75db61788861db8731b9356c70a8bad4447aa/> add recursive flag to index files in directories recursively
 - <csr-id-079aa42b5ef1fa68a357a67dd7db9516c4a5fb51/> add threaded item injection with join handles in Picker

### Bug Fixes

 - <csr-id-b6b314a1d8c96c1a7ded1dfad521201281070a64/> Finally got navigation working

### Refactor

 - <csr-id-5a70a3672df309ccab090790b72d839011c5ded1/> Convert a few calls to last_item_index
 - <csr-id-a0138182ce82cc323a7fa5d94eb9cb0aceca8234/> Clean up range and window logic
 - <csr-id-54bdce889d04cefa624081b882c64451f5a2f840/> Update app height before render
 - <csr-id-a5c49136fd0ea877ebbce60c3392cb0cc51eef19/> Factor out dir walking

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 1 calendar day.
 - 32 days passed between releases.
 - 20 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.4 ([`b4b257b`](https://github.com/symplasma/picleo/commit/b4b257bb50805e0666ebbee022c68f428393dacc))
    - Add "no items found" screen ([`a8e53a5`](https://github.com/symplasma/picleo/commit/a8e53a50a718c654de32f5c43655d6617fcc49d5))
    - Finally got navigation working ([`b6b314a`](https://github.com/symplasma/picleo/commit/b6b314a1d8c96c1a7ded1dfad521201281070a64))
    - Convert a few calls to last_item_index ([`5a70a36`](https://github.com/symplasma/picleo/commit/5a70a3672df309ccab090790b72d839011c5ded1))
    - Clean up range and window logic ([`a013818`](https://github.com/symplasma/picleo/commit/a0138182ce82cc323a7fa5d94eb9cb0aceca8234))
    - Update app height before render ([`54bdce8`](https://github.com/symplasma/picleo/commit/54bdce889d04cefa624081b882c64451f5a2f840))
    - Add index debugging indicators ([`30d4774`](https://github.com/symplasma/picleo/commit/30d47743a2f274727b93e36749c9871a8d32d974))
    - Add sliding item window ([`ea5b808`](https://github.com/symplasma/picleo/commit/ea5b80881346f5229d653494dbfa0cdf62760380))
    - Improve redraw logic ([`345b843`](https://github.com/symplasma/picleo/commit/345b843f25f804c6b4f39560d0d7bfd05bf671f4))
    - Add polling to improve responsiveness ([`c756ece`](https://github.com/symplasma/picleo/commit/c756ecee3dad3321e760305daac3df4e30ee4d26))
    - Add matched items ([`799d1aa`](https://github.com/symplasma/picleo/commit/799d1aaa936ca62790fb8504bdd140839d3ebcc5))
    - Add item count ([`0df432b`](https://github.com/symplasma/picleo/commit/0df432b122a024e6d613a1ff95c6d8301de64f6e))
    - Add running thread count ([`bb0ea94`](https://github.com/symplasma/picleo/commit/bb0ea943c36c8b142db2d3e308316069f04166e5))
    - Call join_finished_threads in tick ([`84a390c`](https://github.com/symplasma/picleo/commit/84a390c9a497199e5d0ebd85de83a975d199a3a8))
    - Add running_threads method ([`8981ad9`](https://github.com/symplasma/picleo/commit/8981ad967a8c4dfddf7eec92d6da35888c3516af))
    - Add join_finished_threads method to Picker to manage thread handles ([`f580bd4`](https://github.com/symplasma/picleo/commit/f580bd4727608f373472b581a162757f1c5133a1))
    - Add necessary move keyword ([`472bbe2`](https://github.com/symplasma/picleo/commit/472bbe20a890566b4440cd31e3807d5e1546b7ac))
    - Add threaded flag to enable threaded item injection ([`50ebd68`](https://github.com/symplasma/picleo/commit/50ebd68a4f43623c217b68033a7f7e9062ab8521))
    - Factor out dir walking ([`a5c4913`](https://github.com/symplasma/picleo/commit/a5c49136fd0ea877ebbce60c3392cb0cc51eef19))
    - Add recursive flag to index files in directories recursively ([`fee75db`](https://github.com/symplasma/picleo/commit/fee75db61788861db8731b9356c70a8bad4447aa))
    - Add threaded item injection with join handles in Picker ([`079aa42`](https://github.com/symplasma/picleo/commit/079aa42b5ef1fa68a357a67dd7db9516c4a5fb51))
</details>

## v0.1.3 (2025-06-04)

<csr-id-e28dd409e829470a618f797a3bc39735635cbb07/>
<csr-id-c53acba0facdf7ea7f5d53e00120fd4f8c8d0859/>
<csr-id-27e1d7aa90e9d99578bf984ffd02af4e6a8a947a/>

### Chore

 - <csr-id-e28dd409e829470a618f797a3bc39735635cbb07/> Remove todo comment

### New Features

 - <csr-id-84815245415a2863bbab4194701c6c2d544d6be7/> add Ctrl+K to delete from cursor to end of query
 - <csr-id-de6b6f8cf04dac78b020f6f0a41487ffd740727a/> Add jump to beginning and end
 - <csr-id-abd9a43d780cad00f034968c2f4235ef5c78c565/> Allow Alt as a motion modifier
   Users can use Ctrl or Alt to move or delete forward or backward a word at a time (based on whitespace).
 - <csr-id-ba9d2bca22c21e42e711a2aa08c889a2d750b4cf/> add word deletion with Ctrl+Backspace and Ctrl+Delete
 - <csr-id-d66ca929c88dc4d91acd499c8819d291ba0e49e8/> add word navigation with Ctrl+Left/Right arrow keys
 - <csr-id-7f0f1e37150ba16837b74982e0f3053bb87f3df6/> Add user control of cursor position
 - <csr-id-c870a5d4bab8a9a11ed4ea5a2d56a5339676fcba/> Add cursor movement
 - <csr-id-03a0ca2f1430d555bb58d2bbed4e7eaf97833f1e/> add blue block cursor to search input rendering

### Bug Fixes

 - <csr-id-eb2593acd882410ba3bea3cbcba37d7c8697cb56/> Allow typing upper case chars

### Refactor

 - <csr-id-c53acba0facdf7ea7f5d53e00120fd4f8c8d0859/> modify query editing to respect cursor position
 - <csr-id-27e1d7aa90e9d99578bf984ffd02af4e6a8a947a/> Cleanup names and remove cloning

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.3 ([`2be65f2`](https://github.com/symplasma/picleo/commit/2be65f263db9beb9932844aab2edf5364715d918))
    - Add Ctrl+K to delete from cursor to end of query ([`8481524`](https://github.com/symplasma/picleo/commit/84815245415a2863bbab4194701c6c2d544d6be7))
    - Add jump to beginning and end ([`de6b6f8`](https://github.com/symplasma/picleo/commit/de6b6f8cf04dac78b020f6f0a41487ffd740727a))
    - Remove todo comment ([`e28dd40`](https://github.com/symplasma/picleo/commit/e28dd409e829470a618f797a3bc39735635cbb07))
    - Modify query editing to respect cursor position ([`c53acba`](https://github.com/symplasma/picleo/commit/c53acba0facdf7ea7f5d53e00120fd4f8c8d0859))
    - Allow Alt as a motion modifier ([`abd9a43`](https://github.com/symplasma/picleo/commit/abd9a43d780cad00f034968c2f4235ef5c78c565))
    - Add word deletion with Ctrl+Backspace and Ctrl+Delete ([`ba9d2bc`](https://github.com/symplasma/picleo/commit/ba9d2bca22c21e42e711a2aa08c889a2d750b4cf))
    - Add word navigation with Ctrl+Left/Right arrow keys ([`d66ca92`](https://github.com/symplasma/picleo/commit/d66ca929c88dc4d91acd499c8819d291ba0e49e8))
    - Add user control of cursor position ([`7f0f1e3`](https://github.com/symplasma/picleo/commit/7f0f1e37150ba16837b74982e0f3053bb87f3df6))
    - Add cursor movement ([`c870a5d`](https://github.com/symplasma/picleo/commit/c870a5d4bab8a9a11ed4ea5a2d56a5339676fcba))
    - Add blue block cursor to search input rendering ([`03a0ca2`](https://github.com/symplasma/picleo/commit/03a0ca2f1430d555bb58d2bbed4e7eaf97833f1e))
    - Allow typing upper case chars ([`eb2593a`](https://github.com/symplasma/picleo/commit/eb2593acd882410ba3bea3cbcba37d7c8697cb56))
    - Cleanup names and remove cloning ([`27e1d7a`](https://github.com/symplasma/picleo/commit/27e1d7aa90e9d99578bf984ffd02af4e6a8a947a))
</details>

## v0.1.2 (2025-05-28)

### New Features

 - <csr-id-6c5ffec02a9b3fa79f32bd0b3af00bfac60e5760/> Improve navigation controls

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.2 ([`a06bbf8`](https://github.com/symplasma/picleo/commit/a06bbf8ebc986281eba28725068cad9f2622dde1))
    - Improve navigation controls ([`6c5ffec`](https://github.com/symplasma/picleo/commit/6c5ffec02a9b3fa79f32bd0b3af00bfac60e5760))
</details>

## v0.1.1 (2025-05-20)

<csr-id-0d12ccbe8bcc3e7810813343b36f294b47e9d46a/>
<csr-id-8f211607f28408986f6ceb72f9d111a4e14241c2/>

### New Features

 - <csr-id-d07f07b893a7980776127e27ec061d8ad3f67798/> add DisplayPath wrapper to support PathBuf in Picker

### Bug Fixes

 - <csr-id-f2f43f974020a3e099992945759d80d6c774f0da/> resolve ownership issue by cloning DisplayPath

### Refactor

 - <csr-id-0d12ccbe8bcc3e7810813343b36f294b47e9d46a/> restructure main function to handle file and stdin input separately

### Chore

 - <csr-id-8f211607f28408986f6ceb72f9d111a4e14241c2/> Add smart-release generated changelog

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 2 calendar days.
 - 11 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release picleo v0.1.1 ([`6dfe3b8`](https://github.com/symplasma/picleo/commit/6dfe3b8f351c9b4fa58d870efb39809d01b42120))
    - Add smart-release generated changelog ([`8f21160`](https://github.com/symplasma/picleo/commit/8f211607f28408986f6ceb72f9d111a4e14241c2))
    - Release picleo v0.1.1 ([`feff30a`](https://github.com/symplasma/picleo/commit/feff30a6100d932509f1f2d91cf9d33aab4b9aab))
    - Upgrade incompatible dependencies ([`d075281`](https://github.com/symplasma/picleo/commit/d075281f6a0758bf752f0107f28872a5fba194e7))
    - Upgrade compatible dependencies ([`9bdbde5`](https://github.com/symplasma/picleo/commit/9bdbde598107e5528a4c2860d1152c44c9c46939))
    - Switch to rendering the TUI on stderr ([`782d378`](https://github.com/symplasma/picleo/commit/782d37878c5e43fa5d65cd8c73aee9ccae24e0c9))
    - Renamed the bin ([`942ca93`](https://github.com/symplasma/picleo/commit/942ca931fd7980dfd51260aea85a6eaffe99cdde))
    - Correct clippy lints ([`a7d9303`](https://github.com/symplasma/picleo/commit/a7d9303a62944521cb5236459d77683277732fbc))
    - Resolve ownership issue by cloning DisplayPath ([`f2f43f9`](https://github.com/symplasma/picleo/commit/f2f43f974020a3e099992945759d80d6c774f0da))
    - Add DisplayPath wrapper to support PathBuf in Picker ([`d07f07b`](https://github.com/symplasma/picleo/commit/d07f07b893a7980776127e27ec061d8ad3f67798))
    - Restructure main function to handle file and stdin input separately ([`0d12ccb`](https://github.com/symplasma/picleo/commit/0d12ccbe8bcc3e7810813343b36f294b47e9d46a))
</details>

## v0.1.0 (2025-05-09)

### New Features

 - <csr-id-0ed9db1c60cdd79484cdbdc8b9034fd3f6b7b086/> create Rust fuzzy selector CLI application with TUI and file/stdin input

### Bug Fixes

 - <csr-id-3cfa272bec863d65b8d6f99534a4f8051087cb02/> correct Picker type signature to resolve Selectable type mismatch
 - <csr-id-15de0926a46706c140f4125efbf3b5d07f9ae722/> Remove nucleo fuzzy feature and update matching strategy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 14 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update metadata and readme ([`9a93019`](https://github.com/symplasma/picleo/commit/9a930193bc7352ffa2446ee2216ffa2dacac8423))
    - Reorganize and rename crate ([`676fdae`](https://github.com/symplasma/picleo/commit/676fdae8a8bca1e93f53cd873e0a32aca47e29c4))
    - Correct Picker type signature to resolve Selectable type mismatch ([`3cfa272`](https://github.com/symplasma/picleo/commit/3cfa272bec863d65b8d6f99534a4f8051087cb02))
    - Make Picker generic ([`6576d48`](https://github.com/symplasma/picleo/commit/6576d48160727f76411af693228397f71808ad8a))
    - Add todos ([`18209df`](https://github.com/symplasma/picleo/commit/18209df4b1bed4f813c3ad1cb098cddd9026104e))
    - Clean up clippy lints ([`ad35104`](https://github.com/symplasma/picleo/commit/ad35104ef07dde22c6b139deeb687af750213b3c))
    - Remove the push() method ([`fad9755`](https://github.com/symplasma/picleo/commit/fad97554c5dbea8dcc0e6122d9851edde3447646))
    - Rename App to Picker ([`ddb9fc9`](https://github.com/symplasma/picleo/commit/ddb9fc98a367c17de71e08988d1a54d485cfdcf7))
    - Refactor the run() method ([`a02e7b1`](https://github.com/symplasma/picleo/commit/a02e7b1ca8c6b77549de789b760b75cf4a1571c6))
    - Use inject_items for stdin ([`163f9e7`](https://github.com/symplasma/picleo/commit/163f9e76d663a737a8c3ff5543446b1b98ad8bfa))
    - Add inject_items() method ([`7d90818`](https://github.com/symplasma/picleo/commit/7d90818228504f8f473b1cbd5d81d54c5d265e83))
    - Refactor the run_app method ([`8c47f56`](https://github.com/symplasma/picleo/commit/8c47f5674a3ea39cb6f604f8791156c79387d5e0))
    - Add lorri and shell.nix config ([`1d5bda1`](https://github.com/symplasma/picleo/commit/1d5bda1ae64b0cf6bf4f6732dbcf479c32720e2d))
    - Remove unused items ([`d9e93f4`](https://github.com/symplasma/picleo/commit/d9e93f40345495ee463a1aa237259c13ecaeef75))
    - Add clear query functionality ([`a4ac21a`](https://github.com/symplasma/picleo/commit/a4ac21a331f8bb46fd8abd466f9ff892d3902b74))
    - Remove comments ([`ada72af`](https://github.com/symplasma/picleo/commit/ada72af3c6d745a9ba24d63157372eb0fd881901))
    - Auto-select highlighted item ([`b0c6849`](https://github.com/symplasma/picleo/commit/b0c684935dc3825da28c61b77b2845405520411c))
    - Print items after restoring terminal ([`b47fb9b`](https://github.com/symplasma/picleo/commit/b47fb9bd7ec34bfc708edd6e668fe89034fd3b54))
    - Add critical TODOs ([`3741e2e`](https://github.com/symplasma/picleo/commit/3741e2e658927ce377046352e92d3a0e4019de65))
    - Connect fuzzy matching ([`2a58cf6`](https://github.com/symplasma/picleo/commit/2a58cf63d8046572b5f6bdf01a77004020e84476))
    - Get selection moving ([`5d03e53`](https://github.com/symplasma/picleo/commit/5d03e537d8859e9aac197445f91876762db25b8d))
    - Call tick so data shows ([`5e8d82b`](https://github.com/symplasma/picleo/commit/5e8d82b59fd15573d78e81b4db639b4228b5b4fd))
    - Respond to key modifiers ([`7e555c9`](https://github.com/symplasma/picleo/commit/7e555c975f5f8dd11a70d25d1b883d3727efcaf8))
    - Allow modifying query ([`f189388`](https://github.com/symplasma/picleo/commit/f189388f5a8e7d10b9beda9a65c44a064d2fefab))
    - Initial switch to Nucleo for matching ([`c27a6bd`](https://github.com/symplasma/picleo/commit/c27a6bde592fd91b971191865ba28eff5fcb2cd4))
    - Run cargo fmt ([`a6ff083`](https://github.com/symplasma/picleo/commit/a6ff083c4bf376d781d2ab7cf6c318314e2914e8))
    - Add Cargo.lock ([`6f83254`](https://github.com/symplasma/picleo/commit/6f83254fbcc5dfc26fa532951d3f639c485884f7))
    - Add target dir to ignores ([`95ebd02`](https://github.com/symplasma/picleo/commit/95ebd029c96912330bf896640054b251bf778f6e))
    - Remove nucleo fuzzy feature and update matching strategy ([`15de092`](https://github.com/symplasma/picleo/commit/15de0926a46706c140f4125efbf3b5d07f9ae722))
    - These changes address the compilation errors in your Rust application. The key modifications include: ([`3b1a7e0`](https://github.com/symplasma/picleo/commit/3b1a7e05e669147f7ea779de9718c204cf701384))
    - Create Rust fuzzy selector CLI application with TUI and file/stdin input ([`0ed9db1`](https://github.com/symplasma/picleo/commit/0ed9db1c60cdd79484cdbdc8b9034fd3f6b7b086))
    - Add aider ignores ([`43b2593`](https://github.com/symplasma/picleo/commit/43b2593b8aa81a1027df11ea7cd07fab796d2410))
    - Initial Commit ([`38a1747`](https://github.com/symplasma/picleo/commit/38a17476bcc2b678ff200eb9c6fadaac2d8a0fae))
</details>

