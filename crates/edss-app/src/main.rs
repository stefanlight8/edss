use {edss_gui::App, edss_rpc::event::Event, eframe, std::thread, tokio::sync::mpsc};

fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (event_tx, event_rx) = mpsc::channel::<Event>(128);
    let (mut runtime, runtime_handle) = edss_runtime::runtime::Runtime::new(event_tx);

    thread::spawn(move || {
        runtime.start();
    });

    eframe::run_native(
        "Elite Dangerous Session Summaries",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(App::new(cc, runtime_handle, event_rx, None)))),
    )
}
