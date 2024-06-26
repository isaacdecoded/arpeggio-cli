# ARPEGGIO CLI

The Arpeggio CLI tool provides an interface for managing projects based on the [Arpeggio template](https://github.com/isaacdecoded/arpeggio), enforcing the practice of __Domain-driven Design (DDD)__, __Clean Architecture__ and __Command and Query Responsibility Segregation (CQRS)__ approaches.

### Requirements

- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- [Rust](https://www.rust-lang.org/tools/install)

### Installation

```sh
cargo install arpeggio
```

### Usage

Most relevant commands included in the Arpeggio CLI are:

- Create a new project
```sh
arpeggio new <PROJECT_NAME>
```

- Add a bounded context to current project directory
```sh
arpeggio add bounded-context <BOUNDED_CONTEXT_NAME>
```

- Add an aggregate to specified bounded context
```sh
arpeggio add aggregate <AGGREGATE_NAME> [BOUNDED_CONTEXT_NAME]
```

- And finally, add components to specified aggregate and bounded context, for example:
```sh
arpeggio add entity <ENTITY_NAME> [AGGREGATE_NAME] [BOUNDED_CONTEXT_NAME]
```
```sh
arpeggio add command <COMMAND_NAME> [AGGREGATE_NAME] [BOUNDED_CONTEXT_NAME]
```
```sh
arpeggio add controller <CONTROLLER_NAME> [AGGREGATE_NAME] [BOUNDED_CONTEXT_NAME]
```
```sh
arpeggio add repository <REPOSITORY_NAME> [AGGREGATE_NAME] [BOUNDED_CONTEXT_NAME] [--domain] [--infrastructure]
```

For more information, this is possible to explore the commands and its arguments by using the _--help_ flag:
```sh
arpeggio --help
```
