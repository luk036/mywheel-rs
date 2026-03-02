# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive README with quick start examples and feature highlights
- Integration examples demonstrating combined usage of multiple data structures
- Extensive benchmark suite comparing mywheel-rs vs std::collections
- Property-based tests using quickcheck for invariant verification
- Integration tests for module interactions and complex scenarios
- Enhanced documentation with comprehensive doc tests for all modules

### Improved
- Documentation coverage across all data structures
- MapAdapter documentation with detailed examples for all methods
- Code examples demonstrating real-world use cases
- Performance comparison documentation with benchmark results

### Documentation
- Added API design rationale and performance comparison guides
- Enhanced rustdoc comments with usage patterns and examples
- Added feature highlights and when-to-use guidelines
- Included memory usage and complexity analysis

## [0.1.2] - 2025-01-XX

### Added
- Logging module with configurable initialization
- QuickCheck support for property-based testing
- Enhanced examples directory

### Changed
- Improved code formatting and style consistency
- Updated dependencies to latest versions

### Fixed
- Documentation build issues
- Clippy warnings

## [0.1.1] - 2024-XX-XX

### Added
- Initial implementation of core data structures
- Dllist: Non-owning doubly linked list
- BPQueue: Bounded priority queue
- RepeatArray: Zero-allocation constant array
- ShiftArray: Offset-based array indexing
- MapAdapter: Vector-like adapter
- Robin: Round-robin scheduler

### Documentation
- Basic API documentation
- Implementation notes and design rationale
