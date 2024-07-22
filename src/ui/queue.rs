use std::path::MAIN_SEPARATOR;

use gpui::*;
use prelude::FluentBuilder;
use tracing::debug;

use crate::data::types::UIQueueItem;

use super::models::{Models, PlaybackInfo};

pub struct QueueItem {
    item: UIQueueItem,
    current_track: Model<Option<String>>,
}

impl QueueItem {
    pub fn new(cx: &mut WindowContext, item: UIQueueItem) -> View<Self> {
        cx.new_view(move |cx| {
            let current_track = cx.global::<PlaybackInfo>().current_track.clone();

            cx.observe(&current_track, move |_, _, cx| {
                cx.notify();
            })
            .detach();

            Self {
                item,
                current_track,
            }
        })
    }
}

impl Render for QueueItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_current = self
            .current_track
            .read(cx)
            .as_ref()
            .map(|v| v == &self.item.file_path)
            .unwrap_or(false);

        let album_art = self
            .item
            .album_art
            .as_ref()
            .map(|v| ImageSource::Data(v.clone()));

        div()
            .w_full()
            .id("info-section")
            .flex()
            .overflow_x_hidden()
            .flex_shrink_0()
            .gap(px(10.0))
            .p(px(10.0))
            .border_t(px(1.0))
            .border_color(rgb(0x1e293b))
            .when(is_current, |div| div.bg(rgb(0x1f2937)))
            .child(
                div()
                    .id("album-art")
                    .rounded(px(4.0))
                    .bg(rgb(0x4b5563))
                    .shadow_sm()
                    .w(px(36.0))
                    .h(px(36.0))
                    .flex_shrink_0()
                    .when(album_art.is_some(), |div| {
                        div.child(
                            img(album_art.unwrap())
                                .w(px(36.0))
                                .h(px(36.0))
                                .rounded(px(4.0)),
                        )
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .line_height(rems(1.0))
                    .text_size(px(15.0))
                    .gap_1()
                    .child(
                        div().font_weight(FontWeight::EXTRA_BOLD).child(
                            self.item
                                .metadata
                                .artist
                                .clone()
                                .unwrap_or("Unknown Artist".into()),
                        ),
                    )
                    .child(
                        // TODO: this goes off the screen if the name is too long
                        // no idea how to fix this with current GPUI limitations
                        div().w_full().overflow_x_hidden().child(
                            self.item.metadata.name.clone().unwrap_or(
                                self.item
                                    .file_path
                                    .split(MAIN_SEPARATOR)
                                    .last()
                                    .unwrap()
                                    .into(),
                            ),
                        ),
                    ),
            )
    }
}

pub struct Queue {
    state: ListState,
}

impl Queue {
    pub fn new<V: 'static>(cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|cx| {
            let items = cx.global::<Models>().queue.clone();

            cx.observe(&items, |this: &mut Queue, m, cx| {
                let items = m.read(cx).clone();
                debug!("Queue updated, new length is {}", items.len());
                this.state =
                    ListState::new(items.len(), ListAlignment::Top, px(32.0), move |idx, cx| {
                        let item = items.get(idx).unwrap().clone();
                        div().child(QueueItem::new(cx, item)).into_any_element()
                    });
                cx.notify();
            })
            .detach();

            Self {
                state: ListState::new(0, ListAlignment::Top, px(32.0), move |_, _| {
                    div().into_any_element()
                }),
            }
        })
    }
}

impl Render for Queue {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .top_0()
            .right_0()
            .h_full()
            .w(px(275.0))
            .bg(rgb(0x111827))
            .border_l(px(1.0))
            .border_color(rgb(0x1e293b))
            .pt(px(9.0))
            .pb(px(0.0))
            .child(
                div()
                    .mb(px(10.0))
                    .mx(px(10.0))
                    .line_height(px(24.0))
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(24.0))
                    .child("Queue"),
            )
            .flex()
            .flex_col()
            .child(list(self.state.clone()).w_full().h_full().flex().flex_col())
    }
}
