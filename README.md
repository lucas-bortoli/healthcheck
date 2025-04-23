# Healthcheck App

This application performs health checks on a list of URLs at specified intervals and logs the results. It can also send desktop notifications on errors.

## Requirements

- Rust installed (version 1.60 or later recommended)
- `cargo` package manager

## Installation

Clone the repository:

```bash
git clone <repository_url>
cd <repository_directory>
```

Build the application:

```bash
cargo build --release
```

The executable will be located in the `target/release/` directory.

## Configuration

The application is configured using a JSON5 file named `healthcheck_config.json5`. A sample configuration file will be created if one does not exist when the application first runs.

The configuration file is a JSON5 array of endpoint objects. Each endpoint object has the following properties:

- `url`: (required) The URL to check.
- `interval`: (required) The interval in seconds between checks.
- `expect`: (optional) The expected HTTP status code. If not specified, the application expects a status code of 200.

**Example `healthcheck_config.json5`:**

```json5
[
  {
    url: "https://example.com",
    interval: 60,
  },
  {
    url: "https://example.com/fake_page_should_404",
    interval: 600, // 10 minutes
    expect: 404, // this URL should return 404 Not Found
  },
]
```

## Usage

Place the `healthcheck_config.json5` file in the same directory as the executable. Then, run the executable:

```bash
cargo run -r
```

The application will start checking the URLs defined in the configuration file and log the results to `healthcheck_log.log`. Desktop notifications will be sent for errors.

## Logging

The application logs all health check results to a file named `healthcheck_log.log` in the same directory as the executable.

## Launching on Startup (Windows)

A VBScript launcher is included in the repository to enable running the healthcheck application automatically on Windows startup.

### `hidden_console_launcher.vbs`

The file `hidden_console_launcher.vbs` is located in the root of the repository. It launches the `healthcheck.exe` executable with a hidden console window.

**Instructions:**

1.  **Update the paths:** Open `hidden_console_launcher.vbs` in a text editor. **Critically**, update the lines:

    ```vbs
    WshShell.CurrentDirectory = "C:\Users\Lucas\Binaries"
    WshShell.Run "C:\Users\Lucas\Binaries\healthcheck.exe", 0, False
    ```

    Replace `"C:\Users\Lucas\Binaries"` with the actual directory where you've placed the `healthcheck.exe` executable and the configuration file. Ensure the path to `healthcheck.exe` is correct.

2.  **Copy to Startup Folder:** Copy the modified `hidden_console_launcher.vbs` file to your Windows startup folder. You can access this folder by opening the Run dialog (Win + R) and typing `shell:startup`, then pressing Enter.

3.  **Restart:** Restart your computer. The `healthcheck.exe` application should now launch automatically in the background with a hidden console window.
