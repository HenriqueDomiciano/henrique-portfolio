use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::app::components::{Project, ProjectCard, applications};
pub mod components;
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/henrique-portfolio.css"/>

        // sets the document title
        <Title text="Henrique Domiciano"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let projects = vec![
        Project {
            name: "RP2040 USB UART",
            description: "USB CDC multiple UART bridge written in Rust and Embassy.",
            url: "https://github.com/HenriqueDomiciano/rp2040-usb-uart-cdc",
        },
        Project {
            name: "ESP32 BLE UART Bridge",
            description: "An ESP32 based BLE to Uart Bridge for wireless communication.",
            url: "https://github.com/HenriqueDomiciano/Esp32-BLE-to-UART-bridge",
        },
        Project {
            name: "Firmware Testing",
            description: "Embedded firmware validation and automation",
            url: "",
        },
        Project {
            name: "ESP32 BLE to MODBUS relay Adapter",
            description: "Project to transform any relay Board in to an BLE modbus RTU relay board",
            url: "https://github.com/HenriqueDomiciano/ESP32-Modbus-Relay-BLE-server",
        },
        Project {
            name: "Rust MODBUS RTU Relay Board controller cli",
            description: "Project to add an interface of communication to chinese MDBUS RTU Relay Boards",
            url: "https://github.com/HenriqueDomiciano/relay_board_rs_485",
        },
    ];

    view! {
        <section class="about-hero">
            <div class="container">
                 <div class="about-hero-content">

                    <div class="eyebrow">
                        <span class="status-dot"></span>
                        "ABOUT_ME"
                    </div>

                    <h1>
                        "Henrique "
                        <span>"Osinski"</span>
                    </h1>

                    <h2>
                        "Firmware Engineer"
                    </h2>

                    <p class="about-hero-description">
                        "I build embedded systems, firmware and software
                        that connect hardware and the real world."
                    </p>

                    <p class="about-hero-stack">
                        "C · Rust · Python · Linux · RTOS"
                    </p>

                    <div class="hero-actions">
                        <a
                            class="btn btn-secondary"
                            href="#applications"
                        >
                            "MISCELLANEOUS"
                        </a>
                        <a
                            class="btn btn-primary"
                            href="#projects"
                        >
                            "VIEW PROJECTS"
                        </a>
                        <a
                            class="btn btn-secondary"
                            href="https://github.com/HenriqueDomiciano"
                            target="_blank"
                        >
                            "GITHUB"
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <section id="projects">

            <div class="container">

                    <div class="section-number">
                        "01 // PROJECTS"
                    </div>

                    <h2>
                        "Things I've built"
                    </h2>

                    <p>
                        "Embedded systems, firmware and software projects."
                    </p>
                <div class="projects-grid">

                    {
                        projects
                            .into_iter().enumerate()
                            .map(|(i,project)| {
                                view! {
                                    <ProjectCard project=project number= i as u32 />
                                }
                            })
                            .collect_view()
                    }

                </div>
            </div>
        </section>
        <section id="applications"> 
            <div class="container">
                    <div class="section-number">
                        "02 // Some Nice Aplications"
                    </div>
                    <div class="terminal-body">
                        <applications::UartAsciiConverter />    
                    </div>
                    <div class = "terminal-body">
                        <applications::FileHasher />
                    </div>
            </div>
        </section>
    }
}
