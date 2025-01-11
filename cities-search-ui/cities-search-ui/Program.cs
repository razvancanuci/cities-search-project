using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using cities_search_ui;
using cities_search_ui.Services;
using Polly;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddHttpClient<WeatherApiService>(c =>
    {
        c.BaseAddress = new Uri(Environment.GetEnvironmentVariable("API_URL")!);
    })
    .AddTransientHttpErrorPolicy(b => b.WaitAndRetryAsync(new[]
    {
        TimeSpan.FromSeconds(1),
        TimeSpan.FromSeconds(5),
        TimeSpan.FromSeconds(10),
    }));

await builder.Build().RunAsync();