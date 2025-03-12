use crate::models::TrainAnnouncement;
use axum::response::Html;

pub fn error_page(message: &str) -> Html<String> {
    Html(format!(
        r#"<html>
            <head><title>Error</title></head>
            <body><h1>Error: {}</h1></body>
           </html>"#,
        message
    ))
}

pub fn render_train_table(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    // Sort announcements by time
    let mut sorted_announcements = announcements;
    sorted_announcements.sort_by_key(|a| a.advertised_time_at_location);

    // Build the HTML table
    let mut html = String::from(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Train Announcements</title>
            <style>
                body {
                    font-family: 'Segoe UI', Arial, sans-serif;
                    background-color: #121212;
                    color: #e0e0e0;
                    margin: 20px;
                    line-height: 1.6;
                }
                h1 {
                    color: #bb86fc;
                    margin-bottom: 20px;
                    border-bottom: 1px solid #333;
                    padding-bottom: 10px;
                }
                table {
                    border-collapse: collapse;
                    width: 100%;
                    margin-top: 25px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
                }
                th, td {
                    padding: 15px;
                    text-align: left;
                    border-bottom: 1px solid #333;
                }
                th {
                    background-color: #1f1f1f;
                    color: #bb86fc;
                    font-weight: 600;
                }
                tr {
                    background-color: #1e1e1e;
                }
                tr:hover {
                    background-color: #2c2c2c;
                }
                .time-column {
                    color: #03dac6; /* Teal accent for time */
                }
                .destination-column {
                    color: #cf6679; /* Pink-red for destination */
                }
                .train-column {
                    font-weight: 500;
                }
                @media (max-width: 600px) {
                    table, th, td {
                        font-size: 14px;
                        padding: 8px;
                    }
                    h1 {
                        font-size: 24px;
                    }
                }
            </style>
        </head>
        <body>
            <h1>Skövde Train Announcements</h1>
            <table>
                <tr>
                    <th>Train ID</th>
                    <th>Time</th>
                    <th>Destination</th>
                </tr>
        "#,
    );

    // Add rows for each announcement
    for announcement in sorted_announcements {
        // Format the datetime nicely
        let formatted_time = announcement
            .advertised_time_at_location
            .format("%H:%M, %d %b %Y");

        // Get destination (if any)
        let destination = if !announcement.to_location.is_empty() {
            // Find the location with highest priority (lowest number)
            announcement
                .to_location
                .iter()
                .min_by_key(|loc| loc.priority)
                .map_or("Unknown".to_string(), |loc| loc.location_name.clone())
        } else {
            "N/A".to_string()
        };

        html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>"#,
            announcement.advertised_train_ident, formatted_time, destination
        ));
    }

    // Close the HTML
    html.push_str(
        r#"
            </table>
        </body>
        </html>
        "#,
    );

    Html(html)
}
