use dioxus::prelude::*;
use yt_dlp::Youtube;
use std::path::PathBuf;
use std::time::Duration;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        UrlInput {}
    }
}

#[component]
pub fn UrlInput() -> Element {
    let mut url = use_signal(|| String::new());
    let mut output_dir = use_signal(|| String::new());
    let mut output_file = use_signal(|| String::new());
    let mut is_valid_url = use_signal(|| false);
    let mut is_successful_downloaded = use_signal(|| false);
    let mut is_loading = use_signal(|| false);

    rsx! {
        div {
            style: "min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; padding: 2rem;",
            
            div {
                style: "background: white; border-radius: 20px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); padding: 3rem; max-width: 600px; width: 100%;",
                
                // Header
                div {
                    style: "text-align: center; margin-bottom: 2.5rem;",
                    
                    h1 {
                        style: "color: #2d3748; font-size: 2.5rem; font-weight: 700; margin: 0 0 0.5rem 0;",
                        "🎬 YouTube Downloader"
                    }
                    
                    p {
                        style: "color: #718096; font-size: 1.1rem; margin: 0;",
                        "Download your favorite YouTube videos with ease"
                    }
                }
                
                // Form Container
                div {
                    style: "display: flex; flex-direction: column; gap: 1.5rem;",
                    
                    // URL Input Group
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.5rem;",
                        
                        label {
                            style: "color: #4a5568; font-weight: 600; font-size: 0.95rem;",
                            "YouTube URL"
                        }
                        
                        input {
                            r#type: "url",
                            placeholder: "https://www.youtube.com/watch?v=...",
                            value: "{url}",
                            oninput: move |event| {
                                let new_url = event.value().to_string();
                                url.set(new_url.clone());
                                // Basic URL validation
                                is_valid_url.set(new_url.contains("youtube.com") || new_url.contains("youtu.be"));
                            },
                            style: "padding: 1rem; border: 2px solid #e2e8f0; border-radius: 12px; font-size: 1rem; outline: none; transition: border-color 0.2s; background: #f7fafc;",
                        }
                    }

                    // Output File Input Group
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.5rem;",
                        
                        label {
                            style: "color: #4a5568; font-weight: 600; font-size: 0.95rem;",
                            "Output Directory"
                        }
                        
                        input {
                            r#type: "text",
                            placeholder: "output",
                            value: "{output_dir}",
                            oninput: move |event| {
                                let new_output_dir = event.value().to_string();
                                output_dir.set(new_output_dir.clone());
                            },
                            style: "padding: 1rem; border: 2px solid #e2e8f0; border-radius: 12px; font-size: 1rem; outline: none; transition: border-color 0.2s; background: #f7fafc;",
                        }
                    }
                    
                    // Output File Input Group
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.5rem;",
                        
                        label {
                            style: "color: #4a5568; font-weight: 600; font-size: 0.95rem;",
                            "Output Filename"
                        }
                        
                        input {
                            r#type: "text",
                            placeholder: "my-video.mp4",
                            value: "{output_file}",
                            oninput: move |event| {
                                let new_output_file = event.value().to_string();
                                output_file.set(new_output_file.clone());
                            },
                            style: "padding: 1rem; border: 2px solid #e2e8f0; border-radius: 12px; font-size: 1rem; outline: none; transition: border-color 0.2s; background: #f7fafc;",
                        }
                    }
                    
                    // Download Button with Loading State
                    button {
                        disabled: "{!is_valid_url() || is_loading()}",
                        onclick: move |_| {
                            is_successful_downloaded.set(false);
                            is_loading.set(true);
                            let current_url = url.read();
                            if !current_url.is_empty() {
                                println!("Processing URL: {}", current_url);
                                let url_clone = current_url.clone();
                                let output_file_clone = output_file.read().clone();
                                let output_dir_clone = output_dir.read().clone();
                                spawn(async move {
                                    if let Err(e) = download_video(&url_clone, &output_dir_clone, &output_file_clone).await {
                                        eprintln!("Download failed: {}", e);
                                    }
                                    else {
                                        println!("Download completed successfully!");
                                        // Show success message in a popup or alert
                                        is_successful_downloaded.set(true);
                                    }
                                    is_loading.set(false);
                                });
                            }
                        },
                        style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 12px; padding: 1rem 2rem; font-size: 1.1rem; font-weight: 600; cursor: pointer; transition: transform 0.2s, box-shadow 0.2s; margin-top: 1rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem;",
                        
                        if is_loading() {
                            div {
                                style: "width: 20px; height: 20px; border: 2px solid transparent; border-top: 2px solid white; border-radius: 50%; transform: rotate(0deg);",
                            }
                        }
                        
                        if is_loading() {
                            "Downloading..."
                        } else {
                            "🚀 Download Video"
                        }
                    }
                }
                
                // Loading Message
                if is_loading() {
                    div {
                        style: "margin-top: 2rem; padding: 1rem; background: linear-gradient(135deg, #4299e1 0%, #3182ce 100%); color: white; border-radius: 12px; text-align: center;",
                        
                        div {
                            style: "display: flex; align-items: center; justify-content: center; gap: 0.5rem;",
                            
                            div {
                                style: "width: 16px; height: 16px; border: 2px solid transparent; border-top: 2px solid white; border-radius: 50%;",
                            }
                            span { "Downloading video... Please wait" }
                        }
                    }
                }
                
                // Success Message
                if is_successful_downloaded() {
                    div {
                        style: "margin-top: 2rem; padding: 1rem; background: linear-gradient(135deg, #48bb78 0%, #38a169 100%); color: white; border-radius: 12px; text-align: center;",
                        
                        div {
                            style: "display: flex; align-items: center; justify-content: center; gap: 0.5rem;",
                            
                            span { "✅" }
                            span { "Download completed successfully!" }
                        }
                    }
                }
                
                // Footer
                div {
                    style: "margin-top: 2rem; text-align: center; padding-top: 2rem; border-top: 1px solid #e2e8f0;",
                    
                    p {
                        style: "color: #a0aec0; font-size: 0.9rem; margin: 0;",
                        "Built with ❤️ using Rust & Dioxus"
                    }
                }
            }
        }
    }
}

pub async fn download_video(url: &str, output_dir: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(output_dir);
    
    let mut fetcher = Youtube::with_new_binaries(libraries_dir, output_dir).await?;
    fetcher.with_timeout(Duration::from_secs(6000));

    let video = fetcher.fetch_video_infos(url.to_string()).await?;

    let audio_format = video.best_audio_format().unwrap();
    fetcher.download_format(&audio_format, "temp_audio.mp3").await?;
    println!("Downloaded audio format: {} finished.", audio_format.format);

    let video_format = video.worst_video_format().unwrap();
    fetcher.download_format(&video_format, "temp_video.mp4").await?;
    println!("Downloaded video format: {} finished.", video_format.format);

    println!("Combining audio and video...");
    let output_path = fetcher.combine_audio_and_video("temp_audio.mp3", "temp_video.mp4", output_file).await?;

    // Clean up temporary files
    let _ = std::fs::remove_file("temp_audio.mp3");
    let _ = std::fs::remove_file("temp_video.mp4");
    println!("Download completed successfully! Output file: {}", output_path.display());
    Ok(())
}