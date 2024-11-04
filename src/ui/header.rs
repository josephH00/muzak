use gpui::*;
use prelude::FluentBuilder;

use crate::library::scan::ScanEvent;

use super::{
    constants::{APP_ROUNDING, FONT_AWESOME},
    global_actions::Quit,
    models::Models,
    theme::Theme,
};

pub struct Header {
    scan_status: View<ScanStatus>,
}

impl Header {
    pub fn new<V: 'static>(cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|cx| Self {
            scan_status: ScanStatus::new(cx),
        })
    }
}

impl Render for Header {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let decorations = cx.window_decorations();
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .w_full()
            .text_sm()
            .min_h(px(33.0))
            .max_h(px(33.0))
            .bg(theme.background_secondary)
            .text_sm()
            .border_b_1()
            .id("titlebar")
            .border_color(theme.border_color)
            .when(cfg!(target_os = "windows"), |this| {
                this.on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            })
            .when(cfg!(not(target_os = "windows")), |this| {
                this.on_mouse_down(MouseButton::Left, move |ev, cx| {
                    if ev.click_count != 2 {
                        cx.start_window_move();
                    }
                })
                .on_click(|ev, cx| {
                    if ev.down.click_count == 2 {
                        cx.zoom_window();
                    }
                })
            })
            .map(|div| match decorations {
                Decorations::Server => div,
                Decorations::Client { tiling } => div
                    .when(!(tiling.top || tiling.left), |div| {
                        div.rounded_tl(APP_ROUNDING)
                    })
                    .when(!(tiling.top || tiling.right), |div| {
                        div.rounded_tr(APP_ROUNDING)
                    }),
            })
            .child(
                div()
                    .pl(px(12.0))
                    .pb(px(6.0))
                    .pt(px(4.0))
                    .flex()
                    .child("Muzak")
                    .child(self.scan_status.clone()),
            )
            .when(cfg!(not(target_os = "macos")), |this| {
                this.child(
                    div()
                        .flex()
                        .ml_auto()
                        .child(WindowButton::Minimize)
                        .child(WindowButton::Maximize)
                        .child(WindowButton::Close),
                )
            })
    }
}

pub struct ScanStatus {
    scan_model: Model<ScanEvent>,
}

impl ScanStatus {
    pub fn new<V: 'static>(cx: &mut ViewContext<V>) -> View<Self> {
        let scan_model = cx.global::<Models>().scan_state.clone();

        cx.new_view(|cx| {
            cx.observe(&scan_model, |_, _, cx| {
                cx.notify();
            })
            .detach();

            Self { scan_model }
        })
    }
}

impl Render for ScanStatus {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let status = self.scan_model.read(cx);

        div()
            .flex()
            .text_sm()
            .ml(px(16.0))
            .child(
                div()
                    .mr(px(8.0))
                    .pt(px(5.0))
                    .text_size(px(9.0))
                    .h_full()
                    .font_family(FONT_AWESOME)
                    .child(match status {
                        ScanEvent::ScanCompleteIdle | ScanEvent::ScanCompleteWatching => "",
                        _ => "",
                    }),
            )
            .text_color(theme.text_secondary)
            .child(match status {
                ScanEvent::ScanCompleteIdle => "".to_string(),
                ScanEvent::ScanProgress { current, total } => {
                    format!(
                        "Scanning ({}%)",
                        (*current as f64 / *total as f64 * 100.0).round()
                    )
                }
                ScanEvent::DiscoverProgress(progress) => {
                    format!("Discovering files ({})", progress)
                }
                ScanEvent::Cleaning => "".to_string(),
                ScanEvent::ScanCompleteWatching => "Watching for updates".to_string(),
            })
    }
}

#[derive(PartialEq, Clone, Copy, IntoElement)]
pub enum WindowButton {
    Close,
    Minimize,
    Maximize,
}

impl RenderOnce for WindowButton {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let (bg, hover, active) = if self == WindowButton::Close {
            (
                theme.close_button,
                theme.close_button_hover,
                theme.close_button_active,
            )
        } else {
            (
                theme.window_button,
                theme.window_button_hover,
                theme.window_button_active,
            )
        };

        div()
            .flex()
            .w(px(32.0))
            .h(px(33.0))
            .pb(px(1.0))
            .items_center()
            .justify_center()
            .id(match self {
                WindowButton::Close => "close",
                WindowButton::Minimize => "minimize",
                WindowButton::Maximize => "maximize",
            })
            .bg(bg)
            .hover(|this| this.bg(hover))
            .active(|this| this.bg(active))
            .font_family(FONT_AWESOME)
            .text_size(px(11.0))
            .on_mouse_down(MouseButton::Left, |_, cx| {
                cx.stop_propagation();
                cx.prevent_default();
            })
            .child(match self {
                WindowButton::Close => "",
                WindowButton::Minimize => "",
                WindowButton::Maximize => "",
            })
            .when(self == WindowButton::Close, |this| this.rounded_tr(px(4.0)))
            .on_click(move |_, cx| match self {
                WindowButton::Close => cx.dispatch_action(Box::new(Quit)),
                WindowButton::Minimize => cx.minimize_window(),
                WindowButton::Maximize => cx.zoom_window(),
            })
    }
}
