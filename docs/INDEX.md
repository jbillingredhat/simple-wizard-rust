# Documentation Index

## User Documentation

- **[Quick Start Guide](QUICKSTART.md)** - Get started with simple-wizard in 5 minutes
- **[README](../README.md)** - Main project overview and features

## Developer Documentation

- **[Implementation Guide](IMPLEMENTATION.md)** - Architecture, components, and technical details
- **[Contributing Guide](../CONTRIBUTING.md)** - How to add new page types and validation presets
- **[Changelog](CHANGELOG.md)** - Project history and milestones

## Development Notes

- **[Notes](NOTES.md)** - Miscellaneous development notes and observations

## Quick Links

### Getting Started
1. Read the [Quick Start Guide](QUICKSTART.md)
2. Try the [example installer](../examples/example_install.sh)
3. Check the [Implementation Guide](IMPLEMENTATION.md) for details

### Contributing
1. Read the [Contributing Guide](../CONTRIBUTING.md)
2. Review [Implementation Guide](IMPLEMENTATION.md) architecture
3. Follow the examples in existing code

### API Reference

The wizard uses a simple JSON protocol over Unix domain sockets. All commands are documented in the [Implementation Guide](IMPLEMENTATION.md#api-compatibility).

## File Organization

```
docs/
├── INDEX.md            # This file
├── QUICKSTART.md       # Quick start guide
├── IMPLEMENTATION.md   # Technical implementation details
├── CHANGELOG.md        # Project history and milestones
└── NOTES.md           # Development notes

Root files:
├── README.md          # Main project documentation
├── CONTRIBUTING.md    # Contribution guidelines
└── examples/          # Example scripts
    ├── README.md
    └── example_install.sh
```

## External Resources

- [iced documentation](https://docs.rs/iced/) - GUI framework
- [tokio documentation](https://docs.rs/tokio/) - Async runtime
- [rfd documentation](https://docs.rs/rfd/) - File dialogs
- [Original Python version](https://github.com/jbilling/simple-wizard) - Reference implementation
