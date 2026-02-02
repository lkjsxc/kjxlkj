# Script-Local Mappings

SID and script-local mapping scope.

## Overview

Script-local mappings use `<SID>` to
create namespace-isolated keybindings.

## Purpose

### Namespace Isolation

Prevent naming conflicts between
different configuration modules.

### Internal Helpers

Create helper mappings not exposed
to user's global namespace.

## SID Syntax

### Script ID


### Using SID


## Script Context

### Module Scope


### Isolation

Each module gets unique script ID.
`<SID>helper` in git.toml differs from
`<SID>helper` in lsp.toml.

## Defining Script-Local

### Internal Function


### Chain Helper


## Public vs Private

### Private (SID)


Not accessible outside module.

### Public (Plug)


Exposed for user remapping.

## Module Organization

### Structure


### Module Example


## Advantages

### No Collisions

Multiple modules can use same
internal names without conflict.


Different script IDs, no collision.

### Clean Namespace

Only `<Plug>` mappings visible to users.
Internal `<SID>` mappings hidden.

### Refactoring

Change internal implementation without
affecting public interface.


## Combining Modules

### Import Pattern


### Override Pattern


## Debugging

