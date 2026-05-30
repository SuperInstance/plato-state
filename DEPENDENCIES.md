# DEPENDENCIES — plato-state

## Signal Chain Layer

**L0 (State Representation)** — 16-dimensional room state vectors.

The central state representation crate. Produces and manages 16-dimensional room state vectors that capture the essential state of each room for the PLATO nervous system.

## Ecosystem Dependencies

| Repo | Relationship | Description |
|------|-------------|-------------|
| [plato-rooms](https://github.com/SuperInstance/plato-rooms) | **Depends on** | Room definitions and sensor configurations |
| [plato-tiles](https://github.com/SuperInstance/plato-tiles) | **Depends on** | Tile types that carry state data |
| [plato-nervous](https://github.com/SuperInstance/plato-nervous) | **Depended on by** | Fuses state vectors with vision/audio for the full signal chain |
| [plato-coordination](https://github.com/SuperInstance/plato-coordination) | **Depended on by** | Fleet coordination uses state vectors for cross-room decisions |
| [plato-autonomy](https://github.com/SuperInstance/plato-autonomy) | **Depended on by** | Autonomy metrics are computed from state vector history |
| [plato-dashboard](https://github.com/SuperInstance/plato-dashboard) | **Depended on by** | Dashboard renders room state for monitoring |
| [plato-vision-jepa](https://github.com/SuperInstance/plato-vision-jepa) | **Depended on by** | Vision perception feeds into room state |
| [plato-audio-jepa](https://github.com/SuperInstance/plato-audio-jepa) | **Depended on by** | Audio perception feeds into room state |

## Data Flow

```
IN:
  - Room definitions (from plato-rooms)
  - Tile data (from plato-tiles)
  - Vision state vectors (from plato-vision-jepa, 16-dim)
  - Audio state vectors (from plato-audio-jepa, 16-dim)

OUT:
  - 16-dimensional RoomStateVector
  - State history and drift metrics
  - Anomaly scores per dimension
```

## Dependency Graph Position

```
plato-tiles
  ↓
plato-rooms
  ↓
plato-state ← (this crate)
  ↓ used by plato-nervous, plato-coordination, plato-autonomy, plato-dashboard, plato-vision-jepa, plato-audio-jepa
```
