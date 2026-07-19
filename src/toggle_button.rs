// ----------------------------------------------------------------------------

pub fn compact_button_example(ui: &mut egui::Ui, on: &mut bool) {
    ui.label("toggle_button Example : Try Button Click.");

    ui.horizontal(|ui| {
        toggle_button_led(ui, on, 0.8, egui::Color32::BLACK, egui::Color32::LIGHT_RED);
        toggle_button_led(
            ui,
            on,
            0.8,
            egui::Color32::BLACK,
            egui::Color32::from_rgb(255, 190, 170),
        );
        toggle_button_led(ui, on, 0.8, egui::Color32::BLACK, egui::Color32::YELLOW);
        toggle_button_led(ui, on, 0.8, egui::Color32::WHITE, egui::Color32::GREEN);
        toggle_button_led(ui, on, 0.8, egui::Color32::WHITE, egui::Color32::LIGHT_BLUE);
        toggle_button_led(ui, on, 0.8, egui::Color32::WHITE, egui::Color32::BLUE);
        toggle_button_led(
            ui,
            on,
            0.8,
            egui::Color32::WHITE,
            egui::Color32::from_rgb(250, 140, 255),
        );

        ui.add_space(50.0);

        ui.group(|ui| {
            let mut reverse_on = !on.clone();
            toggle_button_led(ui, on, 0.8, egui::Color32::BLACK, egui::Color32::GREEN);
            toggle_button_led(ui, on, 0.8, egui::Color32::BLACK, egui::Color32::YELLOW);
            toggle_button_led(
                ui,
                &mut reverse_on,
                0.8,
                egui::Color32::BLACK,
                egui::Color32::RED,
            );
        });
    });

    ui.horizontal(|ui| {
        toggle_button_compact(ui, on);

        ui.add_space(50.0);

        toggle_button_compact(ui, on);
        let text = match on {
            true => " ON ",
            false => "OFF",
        };
        let _response = ui.selectable_label(*on, text);
    });

    ui.horizontal(|ui| {
        toggle_button_compact_vertical(ui, on);

        ui.add_space(50.0);
        toggle_button_compact_vertical(ui, on);
        let text = match on {
            true => " ON ",
            false => "OFF",
        };
        let _response = ui.selectable_label(*on, text);
    });

    ui.horizontal(|ui| {
        toggle_button(ui, on, 2.0, 1.0);

        ui.add_space(50.0);
        toggle_button(ui, on, 3.0, 1.0);

        ui.add_space(50.0);
        toggle_button(ui, on, 5.5, 1.0);
    });

    ui.horizontal(|ui| {
        toggle_button_margined(ui, on, 2.0, 1.0, 2.0, 2.0);

        ui.add_space(50.0);
        toggle_button_margined(ui, on, 3.0, 1.0, 2.0, 2.0);

        ui.add_space(50.0);
        toggle_button_margined(ui, on, 5.5, 1.0, 2.0, 2.0);
    });

    ui.horizontal(|ui| {
        toggle_button_colored(
            ui,
            on,
            2.0,
            1.0,
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_colored(
            ui,
            on,
            3.0,
            1.0,
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_colored(
            ui,
            on,
            5.5,
            1.0,
            egui::Color32::WHITE,
            egui::Color32::BLACK,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );
    });

    ui.horizontal(|ui| {
        toggle_button_colored(
            ui,
            on,
            2.0,
            0.2,
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_colored(
            ui,
            on,
            3.0,
            0.2,
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_colored(
            ui,
            on,
            5.5,
            0.2,
            egui::Color32::WHITE,
            egui::Color32::BLACK,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );
    });

    ui.horizontal(|ui| {
        toggle_button_circle(
            ui,
            on,
            2.0,
            0.2,
            0.5,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_circle(
            ui,
            on,
            3.0,
            0.2,
            0.5,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );

        ui.add_space(50.0);
        toggle_button_circle(
            ui,
            on,
            5.5,
            0.2,
            0.5,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
        );
    });

    ui.horizontal(|ui| {
        let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;
        let my_stroke: egui::Stroke = (1.0, egui::Color32::GREEN).into();
        toggle_settings_button(
            ui,
            on,
            5.5,
            0.2,
            0.5,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            my_stroke,
            circle_stroke,
        );

        ui.add_space(50.0);
        let bg_stroke = ui.style_mut().visuals.selection.stroke;
        toggle_settings_button(
            ui,
            on,
            5.5,
            0.5,
            0.8,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            bg_stroke,
            (1.0, egui::Color32::GREEN).into(),
        );

        ui.add_space(50.0);
        let bg_stroke = ui.style_mut().visuals.selection.stroke;
        // auto change : toggle_settings_button()
        toggle_settings_button_vertical(
            ui,
            on,
            5.5,
            0.5,
            0.8,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            bg_stroke,
            (1.0, egui::Color32::GREEN).into(),
        );
    });

    ui.horizontal(|ui| {
        let bg_stroke = ui.style_mut().visuals.widgets.inactive.bg_stroke;
        let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;
        let my_stroke: egui::Stroke = (1.0, egui::Color32::GREEN).into();
        toggle_settings_button_vertical(
            ui,
            on,
            0.2,
            5.5,
            0.5,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            my_stroke,
            circle_stroke,
        );

        ui.add_space(50.0);
        toggle_settings_button_vertical(
            ui,
            on,
            0.5,
            3.5,
            0.8,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            bg_stroke,
            (1.0, egui::Color32::GREEN).into(),
        );

        ui.add_space(50.0);
        // auto change : toggle_settings_button_vertical()
        toggle_settings_button(
            ui,
            on,
            0.5,
            5.5,
            0.8,
            0.0,
            0.0,
            egui::Color32::BLUE,
            egui::Color32::RED,
            egui::Color32::BLUE,
            egui::Color32::RED,
            bg_stroke,
            (1.0, egui::Color32::GREEN).into(),
        );
    });
}

// ----------------------------------------------------------------------------
// Toggle Button
// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_led(ui, &mut on, 0.8, egui::Color32::LIGHT_BLUE, egui::Color32::LIGHT_RED);
/// # });
/// ```

pub fn toggle_button_led(
    ui: &mut egui::Ui,
    on: &mut bool,
    scale: f32,
    left_color: egui::Color32,
    right_color: egui::Color32,
) -> egui::Response {
    toggle_button_circle(
        ui,
        on,
        scale,
        scale,
        scale,
        left_color,
        right_color,
        left_color,
        right_color,
    )
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_compact(ui, &mut on);
/// # });
/// ```

pub fn toggle_button_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    toggle_button_margined(ui, on, 2.0, 1.0, 2.0, 2.0)
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_compact_vertical(ui, &mut on);
/// # });
/// ```

pub fn toggle_button_compact_vertical(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    toggle_button_margined(ui, on, 1.0, 2.0, 2.0, 2.0)
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button(ui, &mut on, 5.5, 1.0);
/// # });
/// ```

pub fn toggle_button(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
) -> egui::Response {
    let inactive_bg_fill = ui.style_mut().visuals.widgets.inactive.bg_fill;
    let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;

    toggle_button_colored(
        ui,
        on,
        x_scale,
        y_scale,
        inactive_bg_fill,
        selection_bg_fill,
        inactive_bg_fill,
        selection_bg_fill,
    )
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_margined(ui, &mut on, 5.5, 1.0, 2.0, 2.0);
/// # });
/// ```

pub fn toggle_button_margined(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
    left_margin: f32,
    right_margin: f32,
) -> egui::Response {
    let inactive_bg_fill = ui.style_mut().visuals.widgets.inactive.bg_fill;
    let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;
    let bg_stroke = ui.style_mut().visuals.selection.stroke;
    let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;

    toggle_settings_button(
        ui,
        on,
        x_scale,
        y_scale,
        0.8,
        left_margin,
        right_margin,
        inactive_bg_fill,
        selection_bg_fill,
        inactive_bg_fill,
        selection_bg_fill,
        bg_stroke,
        circle_stroke,
    )
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_colored(ui, &mut on, 5.5, 0.2, egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED);
/// # });
/// ```

pub fn toggle_button_colored(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
    left_bg_fill: egui::Color32,
    right_bg_fill: egui::Color32,
    left_circle_fill: egui::Color32,
    right_circle_fill: egui::Color32,
) -> egui::Response {
    toggle_button_circle(
        ui,
        on,
        x_scale,
        y_scale,
        0.8,
        left_bg_fill,
        right_bg_fill,
        left_circle_fill,
        right_circle_fill,
    )
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// toggle_button_circle(ui, &mut on, 5.5, 0.2, 0.5, egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED);
/// # });
/// ```

pub fn toggle_button_circle(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
    circle_scale: f32,

    left_bg_fill: egui::Color32,
    right_bg_fill: egui::Color32,
    left_circle_fill: egui::Color32,
    right_circle_fill: egui::Color32,
) -> egui::Response {
    let bg_stroke = ui.style_mut().visuals.widgets.inactive.bg_stroke;
    let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;

    toggle_settings_button(
        ui,
        on,
        x_scale,
        y_scale,
        circle_scale,
        0.0,
        0.0,
        left_bg_fill,
        right_bg_fill,
        left_circle_fill,
        right_circle_fill,
        bg_stroke,
        circle_stroke,
    )
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;
/// let my_stroke: egui::Stroke = (1.0, egui::Color32::GREEN).into();
/// toggle_settings_button(ui, &mut on, 5.5, 0.2, 0.5, 0.0, 0.0,
/// egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED, my_stroke, circle_stroke);
/// let bg_stroke = ui.style_mut().visuals.selection.stroke;
/// toggle_settings_button(ui, &mut on, 5.5, 0.5, 0.8, 0.0, 0.0,
/// egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED, bg_stroke, (1.0, egui::Color32::GREEN).into());
/// # });
/// ```

pub fn toggle_settings_button(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
    circle_scale: f32,
    left_margin: f32,
    right_margin: f32,
    left_bg_fill: egui::Color32,
    right_bg_fill: egui::Color32,
    left_circle_fill: egui::Color32,
    right_circle_fill: egui::Color32,
    bg_stroke: egui::Stroke,
    circle_stroke: egui::Stroke,
) -> egui::Response {
    if x_scale < y_scale {
        let response = toggle_settings_button_vertical(
            ui,
            on,
            x_scale,
            y_scale,
            circle_scale,
            left_margin,
            right_margin,
            left_bg_fill,
            right_bg_fill,
            left_circle_fill,
            right_circle_fill,
            bg_stroke,
            circle_stroke,
        );
        return response;
    }

    let desired_size = ui.spacing().interact_size.y * egui::vec2(x_scale, 1.0);
    let y_space_half =
        (ui.spacing().interact_size.y - (ui.spacing().interact_size.y * y_scale)) / 2.0;
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, false, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let mut rect = rect.expand(visuals.expansion);
        rect.min.y += y_space_half;
        rect.max.y -= y_space_half;
        let radius = 0.5 * ui.spacing().interact_size.y;
        let rect_fill = match *on {
            true => right_bg_fill,
            false => left_bg_fill,
        };
        ui.painter()
            .rect(rect, radius, rect_fill, bg_stroke, egui::StrokeKind::Middle);
        let circle_x = egui::lerp(
            (rect.left() - 2.0 + left_margin + radius)
                ..=(rect.right() + 2.0 - right_margin - radius),
            how_on,
        );
        let center = egui::pos2(circle_x, rect.center().y);
        let circle_fill = match *on {
            true => right_circle_fill,
            false => left_circle_fill,
        };
        ui.painter()
            .circle(center, radius * circle_scale, circle_fill, circle_stroke);
    }

    response
}

// ----------------------------------------------------------------------------

/// ### Example
/// ```ignore
/// # egui::__run_test_ui(|ui| {
/// let mut on = true;
/// let bg_stroke = ui.style_mut().visuals.widgets.inactive.bg_stroke;
/// let circle_stroke = ui.style_mut().visuals.widgets.inactive.fg_stroke;
/// let my_stroke: egui::Stroke = (1.0, egui::Color32::GREEN).into();
/// toggle_settings_button_vertical(ui, &mut on, 0.2, 5.5, 0.5, 0.0, 0.0,
/// egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED, my_stroke, circle_stroke);
/// toggle_settings_button_vertical(ui, &mut on, 0.5, 5.5, 0.8, 0.0, 0.0,
/// egui::Color32::BLUE, egui::Color32::RED, egui::Color32::BLUE, egui::Color32::RED, bg_stroke, (1.0, egui::Color32::GREEN).into());
/// # });
/// ```

pub fn toggle_settings_button_vertical(
    ui: &mut egui::Ui,
    on: &mut bool,
    x_scale: f32,
    y_scale: f32,
    circle_scale: f32,
    top_margin: f32,
    bottom_margin: f32,
    top_bg_fill: egui::Color32,
    bottom_bg_fill: egui::Color32,
    top_circle_fill: egui::Color32,
    bottom_circle_fill: egui::Color32,
    bg_stroke: egui::Stroke,
    circle_stroke: egui::Stroke,
) -> egui::Response {
    if x_scale > y_scale {
        let response = toggle_settings_button(
            ui,
            on,
            x_scale,
            y_scale,
            circle_scale,
            top_margin,
            bottom_margin,
            top_bg_fill,
            bottom_bg_fill,
            top_circle_fill,
            bottom_circle_fill,
            bg_stroke,
            circle_stroke,
        );
        return response;
    }

    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, y_scale);
    let x_space_half =
        (ui.spacing().interact_size.y - (ui.spacing().interact_size.y * x_scale)) / 2.0;
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, false, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let mut rect = rect.expand(visuals.expansion);
        rect.min.x += x_space_half;
        rect.max.x -= x_space_half;
        let radius = 0.5 * ui.spacing().interact_size.y;
        let rect_fill = match *on {
            true => bottom_bg_fill,
            false => top_bg_fill,
        };
        ui.painter()
            .rect(rect, radius, rect_fill, bg_stroke, egui::StrokeKind::Middle);
        let circle_y = egui::lerp(
            (rect.top() - 2.0 + top_margin + radius)
                ..=(rect.bottom() + 2.0 - bottom_margin - radius),
            how_on,
        );
        let center = egui::pos2(rect.center().x, circle_y); // circle_x, rect.center().y);
        let circle_fill = match *on {
            true => bottom_circle_fill,
            false => top_circle_fill,
        };
        ui.painter()
            .circle(center, radius * circle_scale, circle_fill, circle_stroke);
    }

    response
}

// ----------------------------------------------------------------------------
