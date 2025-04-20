# weather-forecast-rs
Simple CLI weather forecast written in the Rust programming language. Based on api provided by [Open Meteo](https://open-meteo.com).

## Usage:
Compile repo from the source code or download an executable from **Releases** page. 
```bash
cargo build --release
```

Then navigate to the executable file in your shell, and run it:

```bash
./weather-forecast Washington us
```
or 
```bash
./weather-forecast Washington
```

## Example Output:
```
City: Washington, United States
Temperature: 18.8°C
Humidity: 65%
Apparent Temp: 18.2°C
```

Additional info can be found in the repo itself.
