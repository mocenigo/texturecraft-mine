//! Save As remembers the file the engine wrote, with the extension it added.

use egui_kittest::Harness;
use texturecraft_ui_egui::TextureCraftApp;

#[test]
fn save_as_remembers_the_written_path() {
    let session = texturecraft_engine::sample_session(256, 64).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1600.0, 960.0)).build_eframe(|cc| TextureCraftApp::new(cc, session));
    h.run_steps(2);
    let dir = std::env::temp_dir().join(format!("tc-save-as-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let chosen = dir.join("named");
    let app = h.state_mut();
    app.ui.windows.save_for = Some(app.session.mesh_id);
    texturecraft_ui_egui::windows::picked(app, "file.saveAs", vec![chosen.clone()]);
    let written = format!("{}.{}", chosen.display(), texturecraft_engine::core::PROJECT_EXT);
    assert_eq!(app.ui.recent.first(), Some(&written));
    assert!(std::path::Path::new(&written).exists());
    std::fs::remove_dir_all(&dir).unwrap();
}
