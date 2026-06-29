# plugin_example (hayashi-spatial)

Um plugin nativo de exemplo para demonstrar a criação e empacotamento de plugins nativos da linguagem **Hayashi** usando o `hayashi-plugin-sdk`.

Este plugin calcula distâncias geográficas de Haversine para análises econométricas regionais/espaciais.

## Funções Disponíveis

### `distance_to_point(lats: List, lons: List, ref_lat: Float, ref_lon: Float) -> List`
Calcula a distância (em km) de uma lista de coordenadas a um ponto fixo (como uma capital ou porto).

### `haversine_distance(lats1: List, lons1: List, lats2: List, lons2: List) -> List`
Calcula a distância par a par entre duas listas de coordenadas.

## Como Usar no Hayashi

```stata
// Baixa, verifica a attestation e carrega o plugin automaticamente
import("sheep-farm/plugin_example")

load "municipios.csv" as df

// Porto Alegre: -30.03, -51.22
generate df dist_capital = distance_to_point(df["latitude"], df["longitude"], -30.03, -51.22)

reg(pib ~ escolaridade + dist_capital, df)
```

## Compilação Local (para desenvolvimento)

```bash
cargo build --release
# O binário gerado estará em target/release/libplugin_example.so (ou .dll / .dylib)
```

## Como Publicar um Novo Release

O `hay install` baixa apenas binários gerados de forma transparente pela CI. Para publicar:
1. Faça commit das alterações.
2. Crie uma tag Git: `git tag v0.1.0`
3. Dê push na tag: `git push origin v0.1.0`
4. A Action configurada no repositório irá compilar, criar o Artifact Attestation de proveniência criptográfica do GitHub e publicar os binários nos Releases do GitHub.

## Licença

MIT
