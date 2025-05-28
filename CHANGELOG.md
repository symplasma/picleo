# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.2 (2025-05-28)

### New Features

 - <csr-id-6c5ffec02a9b3fa79f32bd0b3af00bfac60e5760/> Improve navigation controls

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

