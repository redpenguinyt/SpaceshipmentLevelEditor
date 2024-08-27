# Spaceshipment level editor

This is the source code of the level editor for [Spaceshipment](https://redpenguin777.itch.io/spaceshipment)

Currently, the editor opens into the last file in the levels folder (sorted alphabetically). To open a different level, use `Ctrl+O`

## Keyboard Shortcuts

- `Ctrl+Q` to close the app
- `Ctrl+O` to open a level file
- `Ctrl+S` to save to the current file
- `Ctrl+Shift+S` to save as
- `Ctrl+Alt+S` to save incrementally (if you were working on a file `mylevel.obl`, saving incrementally will save the file as `mylevel001.obl` and `Ctrl S` will now save to this file. Pressing again will save as `mylevel002.obl`, then `mylevel003.obl` and so on)
- `Alt+[1-5]` to set the window scale
- `F1` to show hints!
- `F2` to take a 400x240 screenshot
- `B` to toggle the background image, which is found by searching for an image file with the same name as the level (but with a png file extension)

### Edit Mode

- Hold `RMB` to select any object to drag it around
- `A` or `N` to spawn a new planet
- `W` or `L` to spawn a new wall
- `X`, `D` or `Backspace` to delete a planet while holding it
- `Space` to enter Aim Mode
- `H` to toggle grab indicators (e.g. the circles around the ends of walls)
- `I` to invert a planet's mass, toggling it between regular and antiplanet
- `Ctrl+D` to duplicate the currently selected planet

While holding or hovering over a planet or the target, change its size/mass with the scroll wheel. Scaling a planet down far enough will turn it into a negative mass planet, which pushes the player away instead of attracting them.

You can also use the arrow keys to adjust the position of a body using the arrow keys (while having it selected)

### Aim Mode

- `Esc` to go back to edit mode
- `Space` or `RMB` to launch
- Aim the player with the mouse. Bringing the mouse closer to the player will have a lower launch strength

The trajectory line shows where the player will fly if you launch. The white part of the line represents how far the player will be able to see in the actual game

### Simulation Mode

This is where the player will actually fly, until they crash

- `Escape` to go back to Edit Mode
- `R` to go back to Aim Mode
- `Space` to pause the simulation
- `[1-4]` to set the simulation speed

### Game Over

When the player has either crashed or reached the target

- `Escape` to go back to Edit Mode
- `R` to go back to Aim Mode
