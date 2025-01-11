using System.Net.Http.Json;
using cities_search_ui.Models;

namespace cities_search_ui.Services;

public class WeatherApiService(HttpClient httpClient)
{
    public async Task<SearchResult[]> GetSearchResultAsync(string input)
    {
        try
        {
            var response = await httpClient.GetFromJsonAsync<SearchResult[]>($"/api/search?input={input}");
            return response!;
        }
        catch (Exception e)
        {
            return [];
        }
    }

    public async Task<WeatherData?> GetWeatherDataAsync(string city)
    {
        try
        {
            var response = await httpClient.GetFromJsonAsync<WeatherData>($"/api/weather?city={city}");
            return response!;
        }
        catch (Exception e)
        {
            return null;
        }
       
    }
}