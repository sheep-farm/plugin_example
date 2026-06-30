# plugin_example (hayashi-spatial)

An example native plugin demonstrating the creation and packaging of native plugins for the **Hayashi** language using the `hayashi-plugin-sdk`.

This plugin calculates Haversine geographic distances for regional/spatial econometric analyses.

## Available Functions

### `distance_to_point(lats: List, lons: List, ref_lat: Float, ref_lon: Float) -> List`
Calculates the distance (in km) from a list of coordinates to a fixed point (such as a capital or port).

### `haversine_distance(lats1: List, lons1: List, lats2: List, lons2: List) -> List`
Calculates the pairwise distance between two lists of coordinates.

## How to Install

Install the package directly from GitHub using the Hayashi CLI:

```bash
hay install sheep-farm/plugin_example
```

This will download the native dynamic library pre-compiled by CI/CD and verify its GitHub Artifact Attestation for cryptographic build provenance.

## How to Use in Hayashi

After installation, import the package in your `.hay` script:

```text
import("sheep-farm/plugin_example")

load "municipalities.csv" as df

// Porto Alegre coordinates: -30.03, -51.22
let distances = plugin_example::distance_to_point(df["latitude"], df["longitude"], -30.03, -51.22)
generate df dist_capital = distances

reg(pib ~ schooling + dist_capital, df)
```

## License

MIT
