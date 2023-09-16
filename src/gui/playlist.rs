use iced::{
    widget::{button, column, container, scrollable},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, Msg, PlayerMsg},
    theme::{Button, Text, Theme},
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_playlist(&self) -> Element {
        column![
            button("Shuffle")
                .style(Button::Primary)
                .on_press(Msg::Plr(PlayerMsg::Shuffle)),
            container(self.playlist_songs()).width(Length::Fill),
        ]
        .width(Length::Fill)
        .into()
    }

    fn playlist_songs(&self) -> Element {
        let cur = self.player.get_current();

        scrollable(
            column(
                self.player
                    .get_playlist()
                    .iter()
                    .map(|p| {
                        let c = p.to_owned();
                        let song = self.library.get_song(c);
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        self.list_item(&song, style, c, true)
                    })
                    .collect(),
            )
            .padding([0, 15, 0, 5]),
        )
        .into()
    }
}
