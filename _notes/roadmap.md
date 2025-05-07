Core Simulation Engine
- Model discrete ticks (e.g. 0.1s)
- Simulate puck movement and possession per tick
- Add basic actions: pass, shoot, carry

Minimal AI
- Implement decide_action() per player:
- If puck carrier → pass, shoot, carry
- If off puck → chase puck or position
- Add simple "System 1" reactive logic
- Stub "System 2" positioning planner (update every N ticks)
- Log player decisions and events per tick

Visualization
- Add 2D rendering with egui, macroquad, or Bevy
- Display rink, players, and puck in real time
- Highlight puck carrier, shots, goals

Game Management
- Line management and fatigue
- Faceoffs, goals, stoppages, period breaks
- Save/export game logs or summaries

Strategic AI and Expansion
- Add real tactics: cycle, forecheck, power play, PK
- Build spatial heuristics / heatmaps
- Support franchise layer: rosters, trades, stats
