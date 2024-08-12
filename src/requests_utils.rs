use crate::types::{api_response::ApiResponse, mapped_coords::MappedCoords, weather_data::WeatherData};

pub fn build_api_url(date: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  let latitude_param = format!("{}{}", "latitude=", ListCircuitCoordinates::latitudes());
  let longitude_param = format!("{}{}", "&longitude=", ListCircuitCoordinates::longitudes());
  let start_date_param = format!("{}{}", "&start_date=", date);
  let end_date_param = format!("{}{}", "&end_date=", date);

  let full_url = format!(
    "{}{}{}{}{}{}",
    String::from(API_BASE_URL),
    latitude_param,
    longitude_param,
    start_date_param,
    end_date_param,
    String::from(ListCircuitCoordinates::DESIRED_DATA_PARAMS)
  );

  return Ok(full_url);
}

const API_BASE_URL: &str = "https://archive-api.open-meteo.com/v1/archive?";

struct ListCircuitCoordinates {}

impl ListCircuitCoordinates {
  const CIRCUITS_COORDS: [MappedCoords<'static>; 58] = [
    MappedCoords { name: "Adelaide", latitude: -34.90334, longitude: 138.5406},
    MappedCoords { name: "Ascurra", latitude: -26.95958, longitude: -49.2453},
    MappedCoords { name: "Barcelona", latitude: 41.581722, longitude: 2.2077923},
    MappedCoords { name: "Bathurst", latitude: -33.427067, longitude: 149.51793},
    MappedCoords { name: "Brands Hatch", latitude: 51.35325, longitude: 0.32490975},
    MappedCoords { name: "Brasilia", latitude: -15.782073, longitude: -47.88678},
    MappedCoords { name: "Buenos Aires", latitude: -34.692444, longitude: -58.48294},
    MappedCoords { name: "Cadwell Park", latitude: 53.321613, longitude: 0.0},
    MappedCoords { name: "Campo Grande", latitude: -20.492092, longitude: -54.471283},
    MappedCoords { name: "Cascavel", latitude: -24.991213, longitude: -53.379974},
    MappedCoords { name: "Cleveland", latitude: 41.51142, longitude: -81.70029},
    MappedCoords { name: "Córdoba", latitude: -31.599297, longitude: -64.34732},
    MappedCoords { name: "Curitiba", latitude: -25.623901, longitude: -49.206512},
    MappedCoords { name: "Curvelo", latitude: -18.804922, longitude: -44.424774},
    MappedCoords { name: "Daytona", latitude: 29.209137, longitude: -81.0932},
    MappedCoords { name: "Donnington Park", latitude: 52.829525, longitude: -1.3508606},
    MappedCoords { name: "Estoril (Cascais)", latitude: 38.76977, longitude: -9.331512},
    MappedCoords { name: "Fontana", latitude: 34.059753, longitude: -117.33751},
    MappedCoords { name: "Foz", latitude: -25.623901, longitude: -54.489136},
    MappedCoords { name: "Galeao", latitude: -22.81195, longitude: -43.3125},
    MappedCoords { name: "Gateway", latitude: 38.629173, longitude: -90.12244},
    MappedCoords { name: "Goiânia", latitude: -16.69596, longitude: -49.169037},
    MappedCoords { name: "Granja Viana", latitude: -23.585238, longitude: -46.849335},
    MappedCoords { name: "Guaporé", latitude: -28.857645, longitude: -51.899292},
    MappedCoords { name: "Hockenheimring", latitude: 49.314587, longitude: 8.490566},
    MappedCoords { name: "Ibarra", latitude: 0.38664323, longitude: -78.037506},
    MappedCoords { name: "Imola", latitude: 44.32337, longitude: 11.697248},
    MappedCoords { name: "Indianapolis", latitude: 39.824253, longitude: -86.23953},
    MappedCoords { name: "Interlagos", latitude: -23.022848, longitude: -43.542328},
    MappedCoords { name: "Jacarepaguá", latitude: -22.952549, longitude: -43.30899},
    MappedCoords { name: "Jerez", latitude: 36.731106, longitude: -6.0236206},
    MappedCoords { name: "Kyalami", latitude: -25.975395, longitude: 28.032787},
    MappedCoords { name: "Laguna Seca", latitude: 36.590508, longitude: -121.806274},
    MappedCoords { name: "Le Mans", latitude: 47.97891, longitude: 0.14950167},
    MappedCoords { name: "Londrina", latitude: -23.304043, longitude: -51.18573},
    MappedCoords { name: "Long Beach", latitude: 33.778557, longitude: -118.208954},
    MappedCoords { name: "Monaco (Azure Circuit)", latitude: 43.760983, longitude: 7.4773417},
    MappedCoords { name: "Montréal", latitude: 45.51845, longitude: -73.46939},
    MappedCoords { name: "Monza", latitude: 45.58875, longitude: 9.3396225},
    MappedCoords { name: "Nürburgring", latitude: 50.36907, longitude: 6.9718313},
    MappedCoords { name: "Ortona", latitude: 42.28471, longitude: 14.363104},
    MappedCoords { name: "Oulton Park", latitude: 53.18102, longitude: -2.5568237},
    MappedCoords { name: "Road America", latitude: 43.83128, longitude: -87.95764},
    MappedCoords { name: "Salvador", latitude: -12.970122, longitude: -38.45456},
    MappedCoords { name: "Santa Cruz do Sul", latitude: -29.77153, longitude: -52.473846},
    MappedCoords { name: "Silverstone", latitude: 52.056236, longitude: -0.99264526},
    MappedCoords { name: "Snetterton", latitude: 52.47803, longitude: 1.0037174},
    MappedCoords { name: "Spa-Francorchamps", latitude: 50.439365, longitude: 6.0317464},
    MappedCoords { name: "Speedland", latitude: -23.514938, longitude: -46.610504},
    MappedCoords { name: "Spielberg", latitude: 47.205624, longitude: 14.828711},
    MappedCoords { name: "Suzuka (Kansai)", latitude: 34.90334, longitude: 136.59898},
    MappedCoords { name: "Tarumã", latitude: -30.052725, longitude: -51.038513},
    MappedCoords { name: "Termas de Río Hondo", latitude: -27.521969, longitude: -64.804016},
    MappedCoords { name: "Tykki", latitude: 60.913883, longitude: 26.698565},
    MappedCoords { name: "Velo Città", latitude: -22.31986, longitude: -46.81488},
    MappedCoords { name: "Velopark", latitude: -29.841827, longitude: -51.279083},
    MappedCoords { name: "Virginia", latitude: 36.590508, longitude: -79.16229},
    MappedCoords { name: "Watkins Glen", latitude: 42.355007, longitude: -76.935486},
  ];
  const DESIRED_DATA_PARAMS: &'static str = "&hourly=temperature_2m,rain,cloud_cover";

  fn latitudes() -> String {
    return Self::CIRCUITS_COORDS
        .map(|c| c.latitude.to_string())
        .join(",");
  }

  fn longitudes() -> String {
    return Self::CIRCUITS_COORDS
        .map(|c| c.longitude.to_string())
        .join(",");
  }
}


pub fn map_coords(coords: Vec<ApiResponse>) -> Vec<WeatherData> {
  return coords
    .iter()
    .filter_map(|coord| {
      ListCircuitCoordinates::CIRCUITS_COORDS
        .iter()
        .find(|&c| {
          num_between(coord.latitude, c.latitude-1.0, c.latitude+1.0) 
            && num_between(coord.longitude, c.longitude-1.0, c.longitude+1.0)

        })
        .map(|c| {
          WeatherData {
            name: c.name.to_string(),
            data: coord.clone()
          }
        })
    })
    .collect()
}

fn num_between(num: f64, min: f64, max: f64) -> bool {
  num >= min && num <= max
}