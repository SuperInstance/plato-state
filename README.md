# plato-state

16-dimensional room state vectors for PLATO nervous system.

## What It Does

Produces and manages compact 16-dimensional state vectors that capture the essential state of each room. These vectors are the core data structure that flows through the entire PLATO signal chain — from local sensor processing to fleet-level coordination.

## Ecosystem

- **[plato-rooms](https://github.com/SuperInstance/plato-rooms)** ← Depends on (room definitions)
- **[plato-tiles](https://github.com/SuperInstance/plato-tiles)** ← Depends on (base types)
- **[plato-nervous](https://github.com/SuperInstance/plato-nervous)** → Core signal chain fuses state vectors
- **[plato-coordination](https://github.com/SuperInstance/plato-coordination)** → Fleet coordination uses state for cross-room decisions
- **[plato-autonomy](https://github.com/SuperInstance/plato-autonomy)** → Autonomy metrics from state history
- **[plato-dashboard](https://github.com/SuperInstance/plato-dashboard)** → Renders room state
- **[plato-vision-jepa](https://github.com/SuperInstance/plato-vision-jepa)** → Vision perception feeds into state
- **[plato-audio-jepa](https://github.com/SuperInstance/plato-audio-jepa)** → Audio perception feeds into state

See [DEPENDENCIES.md](./DEPENDENCIES.md) for the full dependency map.
