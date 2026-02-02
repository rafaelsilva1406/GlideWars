# Glide Wars Benchmarks

Performance benchmarks for critical game systems.

## Running Benchmarks

```bash
cargo bench
```

## Benchmark Targets

- **collision_detection**: Measures collision detection performance
- **terrain_generation**: Measures obstacle spawning and cleanup performance
- **enemy_ai**: Measures enemy AI update performance with multiple enemies

## Expected Performance Targets

Based on 60 FPS target (16.67ms frame budget):

- Collision detection: < 1ms per frame (50 entities)
- Terrain generation: < 10ms per spawn
- Enemy AI updates: < 5ms for 50 enemies

## Notes

- Benchmarks are disabled by default in this MINGW environment due to dlltool dependencies
- To enable benchmarks, ensure you have the full MinGW toolchain with binutils installed
- Benchmarks use criterion for statistical analysis
