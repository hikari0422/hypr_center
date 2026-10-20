use std::future::Future;

pub fn spawn_ui<UI, Fut, T, F>(
    ui: &UI,
    async_action: Fut,
    on_complete: F,
) where
    UI: slint::ComponentHandle + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
    F: FnOnce(UI, T) + Send + 'static,
{
    let ui_weak = ui.as_weak();

    tokio::spawn(async move {
        let result = async_action.await;
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                on_complete(ui, result);
            }
        });
    });
}
