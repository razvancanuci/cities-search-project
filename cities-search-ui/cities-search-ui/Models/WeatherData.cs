using System.Text.Json.Serialization;

namespace cities_search_ui.Models;

public class WeatherData
{
    [JsonPropertyName("temperature")] public string Temperature { get; set; } = string.Empty;
    [JsonPropertyName("description")] public string Description { get; set; } = string.Empty;
    [JsonPropertyName("wind")] public string Wind { get; set; } = string.Empty;
    [JsonPropertyName("forecast")] public WeatherForecast[] Forecast { get; set; } = [];
}