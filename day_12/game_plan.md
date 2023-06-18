## Menu screen

The menu screen should have options for 'new game' and a 'quit'. New game should render a modal with a large text input to allow people to paste custom maps like out sample map:
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

This will be error-checked:
- it must have either zero or exactly one each of S and E (these represent start and end)
- all other letters must be lowercase a-z.

If it passes, the game scene will load.

## Game scene

It will then switch to a 3d scene that uses those characters to build a heightmap. The tiles should be green on top and tan all around. 
If there is an S in the input map, it should render a highlight (yellow for S) over that location. The height rendered is lowest (it is an 'a').
If there is an E in the input map, it should render a highlight (blue for E) over that location. The height rendered is highest (it is a 'z').

### gameplay

The game should have a "go" button at the bottom middle only when the map has both S and E present.

Clicking any non-highlighted tile does nothing. When the map has no S or E rendered, it should render a toggle button in the center of the screen (or side by side in the center of the screen if both are not present). This button makes the game tiles that are not highlighted clickable. Clicking highlights the tile and places an S or E at that location, also hiding the button.

Clicking a highlighted location removes the special value (S or E) and the highlight that would accompany it.

### scoring
When the "go" button is clicked the game seeks a path from starting S to ending E. If one is visible, it draws a grey circle at the top of each location on the map that corresponds to a step in the path. It should render the endges by rendering a grey line between the grey circles. The path length should be rendered in a toast.

If no path is available, it should render a message indicating such in a toast.

The map should no longer change states or render any buttons. clicking anywhere or pressing any key at this point will render the end-game modal

## end game modal
buttons for "play again", "new map", and "quit"