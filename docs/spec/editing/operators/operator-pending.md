# Operator-Pending Mode

The state between operator and motion.

## Overview

After typing an operator, the editor
enters operator-pending mode waiting
for a motion or text object.

## Mode Indication

### Visual Cues


### State


## Entering Mode

### Single Key Operators


### With Count


### With Register


## Completing Operation

### With Motion


### With Text Object


### With Search


## Canceling

### Commands


### Effect

Returns to normal mode.
Count and register discarded.

## Timeout

### Configuration


### Behavior

If no motion within timeout,
operation is canceled.

Set to 0 for no timeout.

## Double Operator

### Line Operation


### With Count


## Custom Mappings

### Operator-Pending Maps


### Text Objects


## Motion Types

### Characterwise


### Linewise


### Blockwise


## Exclusive vs Inclusive

### Exclusive

Motion stops before target:

### Inclusive

Motion includes target:

## Count Multiplication

### Before Operator

