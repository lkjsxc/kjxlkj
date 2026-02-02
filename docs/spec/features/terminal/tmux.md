# tmux/screen Integration

Using kjxlkj with terminal multiplexers.

## tmux Integration

### Clipboard Sync


### True Color


### Cursor Shape


## kjxlkj Inside tmux

### Start Session


### Detach/Reattach


## Key Conflicts

### tmux Prefix

Default: `Ctrl+b`

No conflict with kjxlkj defaults.

### Custom Prefix


Avoid remapping kjxlkj keys to prefix.

## Performance

### Escape Time


Important for mode switching.

### Focus Events


## Screen Integration

### Terminal Type


### Clipboard


## Nested Sessions

### Avoiding Confusion


### Visual Distinction


## Copy Mode

### tmux Copy Mode

`Ctrl+b [` enters copy mode.

### kjxlkj Copy

Use kjxlkj's visual mode for editing.

### Clipboard Sharing

OSC52 enables cross-session clipboard.

## Pane Management

### Split Panes


### Navigate Panes


## Session Persistence

### Save Session


### kjxlkj Sessions

Separate from tmux sessions.

## Recommended Setup

### .tmux.conf


## Troubleshooting

### Colors Wrong

Check `$TERM` inside tmux.

### Keys Not Working

Reduce escape-time.

### Clipboard Issues

Enable OSC52 in kjxlkj config.

### Slow Scrolling


## Scripts

### Quick Edit

