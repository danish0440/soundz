use super::{SoundApp, bold, mono, vec2};

/// About window tabs for version information, contributors and models.
pub(super) fn about(app: &mut SoundApp, ctx: &egui::Context) {
    let mut open = app.ui.show_about;
    if !open {
        return;
    }
    let tab_id = egui::Id::unique("about_tab");
    egui::Window::new(crate::localization::text("About SoundZ"))
        .id(egui::Id::unique("About SoundZ"))
        .open(&mut open)
        .default_size(vec2(820.0, 420.0))
        .collapsible(false)
        .show(ctx, |ui| {
            let mut tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id)).unwrap_or(0);
            ui.horizontal(|ui| {
                for (i, l) in (0u8..).zip(["About", "Contributors", "Models"]) {
                    if ui.selectable_label(tab == i, crate::localization::text(l)).clicked() {
                        tab = i;
                    }
                }
            });
            ui.data_mut(|d| d.insert_temp(tab_id, tab));
            ui.separator();
            match tab {
                1 => crate::credits::contributors_ui(ui),
                2 => crate::credits::models_ui(ui),
                _ => {
                    let channel = crate::os_window::name_for(crate::os_window::identity());
                    ui.label(egui::RichText::new(channel).font(bold(22.0)));
                    ui.label(crate::localized_format!("Version {}", crate::build_version()));
                    if crate::os_window::identity() == crate::os_window::LaunchIdentity::NightlyV1 {
                        ui.label(crate::localization::text("Updated by AppZ."));
                        ui.hyperlink_to(crate::localization::text("Report a problem"), "https://github.com/danish0440/soundz/issues");
                    }
                    ui.label(crate::localization::text("An open-source digital audio workstation."));
                    ui.label(crate::localization::text("Dual-licensed MIT OR Apache-2.0. Made with Rust and egui."));
                    ui.hyperlink_to("github.com/danish0440/soundz", "https://github.com/danish0440/soundz");
                    ui.hyperlink_to(crate::localization::text("Join us on GitHub"), "https://github.com/danish0440/soundz/issues");
                }
            }
        });
    app.ui.show_about = open;
}

pub(super) fn session_info(app: &mut SoundApp, ctx: &egui::Context) {
    let mut open = app.ui.show_session_info;
    if !open {
        return;
    }
    egui::Window::new(crate::localization::text("Session Info"))
        .id(egui::Id::unique("Session Info"))
        .open(&mut open)
        .default_size(vec2(480.0, 360.0))
        .show(ctx, |ui| {
            let text = soundz_engine::inspect::session_text(&app.engine);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(egui::RichText::new(text).font(mono(11.0)));
            });
        });
    app.ui.show_session_info = open;
}
