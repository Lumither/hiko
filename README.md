# Hiko
A simple service watchdog written in Rust(backend) and ASP.NET(frontend)

## Introduction

**Warning: The project is in active development currently and will be unstable. The APIs could be changed without prior notice.**

This is a tool designed to monitor the availability of other services running on servers. It operates 24/7 and can be run on a server. It uses the following methods to monitor services:

- Download and Search: It downloads a specified URL resource (supports redirection) and searches for a specific string within the downloaded content. If the string is found, the service is considered to be running normally. If the string is not found, there might be an issue with the service.

- HTTP/S HEAD Check: It sends a HEAD request to the specified URL (supports redirection) and checks the HTTP response code. If the response code is 200, the service is considered to be running normally. Any other response code indicates a possible issue with the service.

## Features (planed, under development)

The tool offers the followimentng features:

- Configurable Monitoring Objects: You can configure the services to monitor by specifying their name, URL, and the string to search for or the expected HTTP response code. You can add, remove, or modify monitoring objects as needed. The monitoring objects will be displayed in alphabetical order based on their names.

- Email Alerts for Resource Issues: The tool keeps track of resource issues and sends email alerts when a certain threshold is reached. You can configure the email account from which alerts are sent and the destination email addresses for receiving the alerts.

- Configuration Persistence: The tool saves the configuration, ensuring that it is not lost even in case of program exceptions or failures.

- Advanced Optional Functionality: It offers optional advanced functionality. It can parse the HTML content received from the URLs and execute any JavaScript within it. It can also follow JavaScript-triggered requests to load additional resources.

## Installation and Configuration

1. Clone this repository to your server.
2. Edit the configuration file to specify the monitoring objects and email alert settings. You can refer to the example configuration file provided and make modifications as needed.
3. Run the startup script for the tool:
    ```shell
    $ ./start.sh
    ```

4. The tool will automatically start monitoring based on the configured settings and send email alerts when necessary.



## License

This project is licensed under the Mozilla Public License, Version 2.0. A copy of the license can be found in the [LICENSE](LICENSE) file.