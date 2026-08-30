use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogResult {
    Primary,
    Secondary,
    Cancel,
} // this is better than the Svae, cancle, close
  //becouse this is more generaic and we can specify it as
  //we wan
  //this just make it more usable componenets
  //Debug allows to debug-print to use println!({:?},result)
  //Clone==>This type can explicitly
  //create another value containing the same logical data by calling .clone()
  //Copy comes along with Clone
  //PartilaEq allows for result == DialogResult::Primary
  //Eq says equaliy is fully well-defined for the type.

pub(crate) fn show_confirmation_dialog(
    ctx: &egui::Context,
    id: &str,
    title: &str,
    message: &str,
    primary_text: &str,
    secondary_text: &str,
    cancle_text: &str,
) -> Option<DialogResult> {
    let response = egui::Modal::new(egui::Id::new(id)).show(ctx, |ui| {
        ui.heading(title);

        ui.add_space(8.0);
        ui.label(message);
        ui.add_space(12.0);

        let mut result = None;

        ui.horizontal(|ui| {
            if ui.button(primary_text).clicked() {
                result = Some(DialogResult::Primary)
            };
            if ui.button(secondary_text).clicked() {
                result = Some(DialogResult::Secondary)
            };
            if ui.button(cancle_text).clicked() {
                result = Some(DialogResult::Cancel)
            };
        });
        result
    });
    if response.should_close() {
        Some(DialogResult::Cancel)
    } else {
        response.inner
    }
}

//
