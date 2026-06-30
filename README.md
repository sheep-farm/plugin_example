# plugin_example (hayashi-spatial)

An example native plugin demonstrating the creation and packaging of native plugins for the **Hayashi** language using the `hayashi-plugin-sdk`.

This plugin calculates Haversine geographic distances for regional/spatial econometric analyses.

## Available Functions

### `distance_to_point(lats: List, lons: List, ref_lat: Float, ref_lon: Float) -> List`
Calculates the distance (in km) from a list of coordinates to a fixed point (such as a capital or port).

### `haversine_distance(lats1: List, lons1: List, lats2: List, lons2: List) -> List`
Calculates the pairwise distance between two lists of coordinates.

## How to Use in Hayashi

```stata
// Downloads, verifies build attestation, and loads the plugin automatically
import("sheep-farm/plugin_example")

load "municipalities.csv" as df

// Porto Alegre: -30.03, -51.22
let distances = plugin_example::distance_to_point(df["latitude"], df["longitude"], -30.03, -51.22)
generate df dist_capital = distances

reg(pib ~ schooling + dist_capital, df)
```

## Local Compilation (for development)

```bash
cargo build --release
# The generated binary will be at target/release/libplugin_example.so (or .dll / .dylib)
```

## How to Publish a New Release

The `hay install` command downloads binaries compiled transparently by CI/CD. To publish:
1. Commit your changes.
2. Create a Git tag: `git tag v0.1.0`
3. Push the tag: `git push origin v0.1.0`
4. The GitHub Action configured in the repository will compile, create a cryptographic GitHub Artifact Attestation for build provenance, and upload the binaries to the GitHub Release.

## License

MIT
