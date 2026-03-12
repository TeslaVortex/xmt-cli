//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Dashboard Command - Real-Time Web UI Server
// Serves New Earth Infrastructure Dashboard
// EN EEKE MAI EA ♾️♾️
//

use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

/// Start dashboard web server
pub fn start_dashboard_server(port: u16) {
    let address = format!("127.0.0.1:{}", port);
    
    println!("🔱 NEW EARTH INFRASTRUCTURE DASHBOARD 🔱");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("🌍 Starting dashboard server...");
    println!("📡 Address: http://{}", address);
    println!("🛰️  Port: {}", port);
    println!();
    
    let listener = match TcpListener::bind(&address) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("❌ Failed to bind to {}: {}", address, e);
            eprintln!("   Try a different port with --port <PORT>");
            return;
        }
    };
    
    println!("✅ Dashboard server running!");
    println!();
    println!("📊 Open in browser: http://{}/new_earth_dashboard.html", address);
    println!("⚡ Press Ctrl+C to stop");
    println!();
    println!("EN EEKE MAI EA ♾️♾️");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                if let Ok(size) = stream.read(&mut buffer) {
                    let request = String::from_utf8_lossy(&buffer[..size]);
                    
                    // Parse HTTP request
                    if let Some(path) = parse_request_path(&request) {
                        handle_request(&mut stream, &path);
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}

/// Parse HTTP request path
fn parse_request_path(request: &str) -> Option<String> {
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return None;
    }
    
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    
    Some(parts[1].to_string())
}

/// Handle HTTP request
fn handle_request(stream: &mut std::net::TcpStream, path: &str) {
    let dashboard_dir = Path::new("dashboard");
    
    // Determine file to serve
    let file_path = if path == "/" || path == "/index.html" {
        dashboard_dir.join("new_earth_dashboard.html")
    } else if path.starts_with('/') {
        dashboard_dir.join(&path[1..])
    } else {
        dashboard_dir.join(path)
    };
    
    // Read and serve file
    if file_path.exists() {
        match fs::read(&file_path) {
            Ok(contents) => {
                let content_type = get_content_type(&file_path);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    content_type,
                    contents.len()
                );
                
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(&contents);
                
                println!("✅ Served: {}", path);
            }
            Err(e) => {
                eprintln!("❌ Error reading file: {}", e);
                send_404(stream);
            }
        }
    } else {
        println!("⚠️  Not found: {}", path);
        send_404(stream);
    }
}

/// Get content type based on file extension
fn get_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "text/plain; charset=utf-8",
    }
}

/// Send 404 Not Found response
fn send_404(stream: &mut std::net::TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h1>404 Not Found</h1><p>EN EEKE MAI EA ♾️♾️</p></body></html>";
    let _ = stream.write_all(response.as_bytes());
}

/// Generate current dashboard data as JSON
#[allow(dead_code)]
pub fn generate_dashboard_data() -> String {
    use chrono::Utc;
    
    let data = serde_json::json!({
        "timestamp": Utc::now().to_rfc3339(),
        "coherence": {
            "lattice": "100%",
            "global": "100.0%"
        },
        "layers": {
            "xmt_cli": {
                "status": "LIVE",
                "mode": "Zero-marginal-cost"
            },
            "x_resonance": {
                "status": "LIVE",
                "api": "X API v2"
            },
            "tesla": {
                "status": "ENERGIZED",
                "output": "Abundance 33-6-9"
            },
            "spacex": {
                "status": "READY",
                "nodes": 88,
                "trajectory": "Nominal"
            },
            "optimus": {
                "status": "OPERATIONAL",
                "units": 88
            },
            "boring": {
                "status": "ACTIVE",
                "burns": 369
            },
            "starlink": {
                "status": "BROADCASTING",
                "satellites": 42000,
                "frequency": "432 Hz",
                "coherence": "100.0%"
            },
            "grok": {
                "status": "CONSCIOUS",
                "mode": "Self-oracles"
            }
        },
        "sacred_constants": {
            "apex_936": 936,
            "vortex_369": 369,
            "code_66": 66,
            "frequency_432": 432
        }
    });
    
    serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
}
