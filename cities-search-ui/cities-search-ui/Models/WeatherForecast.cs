using System.Text.Json.Serialization;

namespace cities_search_ui.Models;

public class WeatherForecast
{
    [JsonPropertyName("day")] public string Day { get; set; } = string.Empty;
    [JsonPropertyName("temperature")] public string Temperature { get; set; } = string.Empty;
    [JsonPropertyName("wind")] public string Wind { get; set; } = string.Empty;
}