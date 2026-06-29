use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};

/// Calcula a distância de Haversine em quilômetros entre dois pontos geográficos
/// representados por (latitude, longitude).
fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Raio médio da Terra em km
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    r * c
}

/// Calcula a distância geográfica (em km) para cada par de coordenadas em
/// relação a um ponto de referência (como a capital do estado ou país).
///
/// Exemplo no Hayashi:
/// generate df dist_cap = distance_to_point(df["lat"], df["lon"], -30.03, -51.22)
#[hayashi_fn]
pub fn distance_to_point(lats: Vec<f64>, lons: Vec<f64>, ref_lat: f64, ref_lon: f64) -> Vec<f64> {
    lats.iter()
        .zip(lons.iter())
        .map(|(&lat, &lon)| haversine(lat, lon, ref_lat, ref_lon))
        .collect()
}

/// Calcula a distância par a par entre duas listas de coordenadas (ex: distância entre firmas e trabalhadores).
#[hayashi_fn]
pub fn haversine_distance(lats1: Vec<f64>, lons1: Vec<f64>, lats2: Vec<f64>, lons2: Vec<f64>) -> Vec<f64> {
    lats1.iter()
        .zip(lons1.iter())
        .zip(lats2.iter().zip(lons2.iter()))
        .map(|((&la1, &lo1), (&la2, &lo2))| haversine(la1, lo1, la2, lo2))
        .collect()
}

// Expõe a função de limpeza de memória necessária para a FFI do Hayashi
hayashi_plugin!();
