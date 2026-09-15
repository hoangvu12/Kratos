//! Settings for this engine's listener and paired clients.
use crate::{settings::widgets, state::AppState, theme::Theme};
use gpui::{ClipboardItem, Context, Entity, Render, Task, Window, div, prelude::*, px};
use roboco_rpc::methods;
use serde_json::{Value, json};

pub struct RemoteAccessPage {
    state: Entity<AppState>,
    snapshot: Option<Value>,
    url: Option<String>,
    error: Option<String>,
    busy: bool,
    task: Option<Task<()>>,
}

impl RemoteAccessPage {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        let mut page = Self {
            state,
            snapshot: None,
            url: None,
            error: None,
            busy: false,
            task: None,
        };
        page.request(methods::GET_REMOTE_ACCESS, json!({}), cx);
        page
    }

    fn request(&mut self, method: &'static str, params: Value, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        let Some(engine) = self.state.read(cx).engine().cloned() else {
            return;
        };
        self.busy = true;
        self.error = None;
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = engine
                .client()
                .call(method, params)
                .await
                .map_err(|error| error.to_string());
            let snapshot = if result.is_ok() && method != methods::GET_REMOTE_ACCESS {
                engine
                    .client()
                    .call(methods::GET_REMOTE_ACCESS, json!({}))
                    .await
                    .map_err(|error| error.to_string())
            } else {
                result.clone()
            };
            this.update(cx, |page, cx| {
                page.busy = false;
                match result {
                    Ok(value) if method == methods::CREATE_PAIRING_LINK => {
                        page.url = value["url"].as_str().map(str::to_owned)
                    }
                    Err(error) => page.error = Some(error.to_string()),
                    _ => {}
                }
                match snapshot {
                    Ok(value) => page.snapshot = Some(value),
                    Err(error) => page.error = Some(error.to_string()),
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }
}

impl Render for RemoteAccessPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();
        let status = self.snapshot.as_ref().map(|value| &value["status"]);
        let enabled = status
            .and_then(|value| value["enabled"].as_bool())
            .unwrap_or(false);
        let error = self.error.clone().or_else(|| {
            status
                .and_then(|value| value["error"].as_str())
                .map(str::to_owned)
        });
        let mut page = widgets::page_column()
            .child(widgets::page_header(&theme, "Remote access", None))
            .child(widgets::page_subtitle(&theme, "Pair your other devices with this engine. Use a trusted network or your own tunnel."))
            .child(widgets::section_card(&theme).child(widgets::card_row(&theme, true)
                .child(div().flex_1().child(widgets::row_title(&theme, "Allow remote connections"))
                    .child(widgets::page_subtitle(&theme, if enabled { "Remote clients can connect with a paired session." } else { "Only local clients can connect." })))
                .child(widgets::toggle_switch(&theme, enabled).id("remote-access-toggle").cursor_pointer()
                    .on_click(cx.listener(move |page, _, _, cx| page.request(methods::SET_REMOTE_ACCESS, json!({"enabled":!enabled}), cx))))));
        if let Some(error) = error {
            page = page.child(widgets::error_strip(&theme, error));
        }
        page = page.child(
            div()
                .flex()
                .gap(px(12.0))
                .child(
                    widgets::ghost_action(&theme)
                        .id("remote-refresh")
                        .child("Refresh")
                        .on_click(cx.listener(|page, _, _, cx| {
                            page.request(methods::GET_REMOTE_ACCESS, json!({}), cx)
                        })),
                )
                .when(enabled, |row| {
                    row.child(
                        widgets::ghost_action(&theme)
                            .id("create-pairing-link")
                            .child("Create pairing link")
                            .on_click(cx.listener(|page, _, _, cx| {
                                page.request(methods::CREATE_PAIRING_LINK, json!({}), cx)
                            })),
                    )
                }),
        );
        if let Some(url) = self.url.clone() {
            let copy = url.clone();
            page = page.child(
                widgets::section_card(&theme).child(
                    widgets::card_row(&theme, true)
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(widgets::row_title(&theme, "Pairing link"))
                                .child(widgets::page_subtitle(
                                    &theme,
                                    "Use once within five minutes.",
                                ))
                                .child(div().text_size(px(12.0)).child(url)),
                        )
                        .child(
                            widgets::ghost_action(&theme)
                                .id("copy-pairing-link")
                                .child("Copy")
                                .on_click(cx.listener(move |_, _, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(copy.clone()))
                                })),
                        ),
                ),
            );
        }
        let rows = self
            .snapshot
            .as_ref()
            .and_then(|value| value["sessions"].as_array())
            .cloned()
            .unwrap_or_default();
        page = page.child(widgets::field_label(&theme, "Paired sessions"));
        if rows.is_empty() {
            page = page.child(widgets::page_subtitle(&theme, "No devices paired yet."));
        }
        for (index, row) in rows.into_iter().enumerate() {
            let revoked = !row["revokedAt"].is_null();
            let id = row["id"].as_str().unwrap_or_default().to_owned();
            let label = row["label"]
                .as_str()
                .filter(|s| !s.is_empty())
                .unwrap_or("Paired device")
                .to_owned();
            let seen = row["lastSeen"]
                .as_i64()
                .and_then(chrono::DateTime::from_timestamp_millis);
            let last_seen = super::devices::format_last_seen(seen, chrono::Utc::now());
            page = page.child(
                widgets::section_card(&theme).child(
                    widgets::card_row(&theme, true)
                        .child(
                            div()
                                .flex_1()
                                .child(widgets::row_title(&theme, label))
                                .child(widgets::page_subtitle(
                                    &theme,
                                    if revoked {
                                        "Revoked".to_owned()
                                    } else {
                                        format!("Last seen {last_seen}")
                                    },
                                )),
                        )
                        .when(!revoked, |row| {
                            row.child(
                                widgets::ghost_action(&theme)
                                    .id(("revoke-session", index))
                                    .child("Revoke")
                                    .on_click(cx.listener(move |page, _, _, cx| {
                                        page.request(
                                            methods::REVOKE_PAIRING_SESSION,
                                            json!({"sessionId":id}),
                                            cx,
                                        )
                                    })),
                            )
                        }),
                ),
            );
        }
        page
    }
}
