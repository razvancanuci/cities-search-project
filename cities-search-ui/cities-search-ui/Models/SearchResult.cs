using System.Text.Json.Serialization;

namespace cities_search_ui.Models;

public class SearchResult
{
    [JsonPropertyName("city")]
    public string? City { get; set; }

    [JsonPropertyName("country")]
    public string? Country { get; set; }

    [JsonPropertyName("population")]
    public int Population { get; set; }

    [JsonPropertyName("latitude")]
    public double Latitude { get; set; }

    [JsonPropertyName("longitude")]
    public double Longitude { get; set; }
    
    [JsonPropertyName("confidence")]
    public double Confidence { get; set; }
}