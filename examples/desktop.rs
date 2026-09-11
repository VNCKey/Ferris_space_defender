use ferris_space_defender::SpaceDefenderGame;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([420.0, 720.0])
            .with_min_inner_size([360.0, 600.0])
            .with_title("Ferris Space Defender"),
        ..Default::default()
    };

    eframe::run_native(
        "Ferris Space Defender",
        native_options,
        Box::new(|_cc| Ok(Box::new(SpaceDefenderGame::default()))),
    )
}
