# Orbit game level editor

This is the source code of the level editor for my game, [Orbit Game (name WIP)](https://redpenguin777.itch.io/orbit-game)

Currently, the editor opens into the last file in the levels folder (sorted alphabetically). To open a different level you'll need to rename it so that the editor finds it at the end of the folder

## Keyboard Shortcuts

- `Ctrl+Q` to close the app
- `Ctrl+O` to open a level file
- `Ctrl+S` to save to the current file
- `Ctrl+Shift+S` to save to a new file (future `Ctrl+S` presses will save to this file)
- `Alt+[1-5]` to set the window scale

### Edit Mode

- Hold `RMB` to select any object to drag it around
- `A` or `N` to create a new planet
- `X`, `D` or `Backspace` to delete a planet while holding it
- `Space` to enter Aim Mode

While holding or hovering over a planet or the target, change its size/mass with the scroll wheel or arrow keys `Up` and `Down`. Scaling a planet down far enough will turn it into a negative mass planet, which pushes the player away instead of attracting them.

### Aim Mode

- `Esc` to go back to edit mode
- `Space` or `RMB` to launch
- Aim the player with the mouse. Bringing the mouse closer to the player will have a lower launch strength

The trajectory line shows where the player will fly if you launch. The white part of the line represents how far the player will be able to see in the actual game

### Simulation Mode

This is where the player will actually fly, until they crash

- `Escape` to go back to Edit Mode
- `R` to go back to Aim Mode
- `[1-4]` to set the simulation speed

### Game Over

When the player has either crashed or reached the target

- `Escape` to go back to Edit Mode
- `R` to go back to Aim Mode