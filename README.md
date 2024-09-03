raf_utils
=========

![Build](https://github.com/RafalSzefler/raf_utils/actions/workflows/rust.yml/badge.svg)
![GitHub Tag](https://img.shields.io/github/v/tag/RafalSzefler/raf_utils)

This is a collection of somewhat independent Rust utils.

* [`raf_array`](https://rafalszefler.github.io/raf_utils/raf_array) that adds a thin wrapper around Rust arrays, and also adds
ref counted arrays and immutable strings.
* [`raf_fnv1a_hasher`](https://rafalszefler.github.io/raf_utils/raf_fnv1a_hasher) which is an implementation of FNV1a hashing algorithm.
* [`raf_multi_valued_logic`](https://rafalszefler.github.io/raf_utils/raf_multi_valued_logic) which implements primitives for multi valued logic.
* [`raf_newick`](https://rafalszefler.github.io/raf_utils/raf_newick) which handles serialization and parsing of Newick format for
directed acyclci graphs.
* [`raf_readonly`](https://rafalszefler.github.io/raf_utils/raf_readonly) which is a proc-macro for generating readonly structs.
* [`raf_shadow_alloc`](https://rafalszefler.github.io/raf_utils/raf_shadow_alloc) which allows a fast buffer allocation on a separate,
thread local stack.
* [`raf_stable_enum`](https://rafalszefler.github.io/raf_utils/raf_stable_enum) which provides proc-macro-attribute for converting Rust enums
into stable ABI enums.
* [`raf_structural_logging`](https://rafalszefler.github.io/raf_utils/raf_structural_logging) which provides abstractions and basic implementation
of rich structural logging.
* [`raf_structural_logging_console`](https://rafalszefler.github.io/raf_utils/raf_structural_logging_console) which provides console handler for `raf_structural_logging`.
* [`raf_tagged_pointer`](https://rafalszefler.github.io/raf_utils/raf_tagged_pointer) which wraps raw pointers into a struct that allows
packing of additional bits, depending on alignment.
