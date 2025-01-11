using System.ComponentModel.DataAnnotations;

namespace cities_search_ui.Models;

public class SearchQuery
{
    [Required, StringLength(15, MinimumLength = 3, ErrorMessage = "Search query must be between 3 and 15 characters.")]
    public string? Input { get; set; }
}