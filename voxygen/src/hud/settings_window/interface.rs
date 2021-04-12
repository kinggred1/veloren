use super::{Event, ScaleChange, RESET_BUTTONS_HEIGHT, RESET_BUTTONS_WIDTH};

use crate::{
    hud::{
        img_ids::Imgs, BarNumbers, BuffPosition, CrosshairType, ShortcutNumbers, Show, TEXT_COLOR,
    },
    i18n::Localization,
    ui::{fonts::Fonts, ImageSlider, ScaleMode, ToggleButton},
    GlobalState,
};
use conrod_core::{
    color,
    position::{Align, Relative},
    widget::{self, Button, Image, Rectangle, Scrollbar, Text},
    widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget, WidgetCommon,
};

widget_ids! {
    struct Ids{
        window,
        window_r,
        window_scrollbar,
        reset_interface_button,
        button_help,
        show_help_label,
        ui_scale_label,
        ui_scale_slider,
        ui_scale_value,
        relative_to_win_button,
        relative_to_win_text,
        absolute_scale_button,
        absolute_scale_text,
        general_txt,
        load_tips_button,
        load_tips_button_label,
        debug_button,
        debug_button_label,
        ch_title,
        ch_transp_slider,
        ch_transp_value,
        ch_transp_text,
        ch_1_bg,
        ch_2_bg,
        ch_3_bg,
        crosshair_outer_1,
        crosshair_inner_1,
        crosshair_outer_2,
        crosshair_inner_2,
        crosshair_outer_3,
        crosshair_inner_3,
        //
        hotbar_title,
        bar_numbers_title,
        show_bar_numbers_none_button,
        show_bar_numbers_none_text,
        show_bar_numbers_values_button,
        show_bar_numbers_values_text,
        show_bar_numbers_percentage_button,
        show_bar_numbers_percentage_text,
        //
        show_shortcuts_button,
        show_shortcuts_text,
        buff_pos_bar_button,
        buff_pos_bar_text,
        buff_pos_map_button,
        buff_pos_map_text,
        //
        chat_transp_title,
        chat_transp_text,
        chat_transp_slider,
        chat_char_name_text,
        chat_char_name_button,
        //
        sct_title,
        sct_show_text,
        sct_show_radio,
        sct_single_dmg_text,
        sct_single_dmg_radio,
        sct_show_batch_text,
        sct_show_batch_radio,
        sct_batched_dmg_radio,
        sct_inc_dmg_text,
        sct_inc_dmg_radio,
        sct_batch_inc_text,
        sct_batch_inc_radio,
        //
        speech_bubble_text,
        speech_bubble_dark_mode_text,
        speech_bubble_dark_mode_button,
        speech_bubble_icon_text,
        speech_bubble_icon_button,
    }
}

#[derive(WidgetCommon)]
pub struct Interface<'a> {
    global_state: &'a GlobalState,
    show: &'a Show,
    imgs: &'a Imgs,
    fonts: &'a Fonts,
    localized_strings: &'a Localization,
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
}
impl<'a> Interface<'a> {
    pub fn new(
        global_state: &'a GlobalState,
        show: &'a Show,
        imgs: &'a Imgs,
        fonts: &'a Fonts,
        localized_strings: &'a Localization,
    ) -> Self {
        Self {
            global_state,
            show,
            imgs,
            fonts,
            localized_strings,
            common: widget::CommonBuilder::default(),
        }
    }
}

pub struct State {
    ids: Ids,
}

impl<'a> Widget for Interface<'a> {
    type Event = Vec<Event>;
    type State = State;
    type Style = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    #[allow(clippy::unused_unit)] // TODO: Pending review in #587
    fn style(&self) -> Self::Style { () }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, ui, .. } = args;

        let mut events = Vec::new();

        Rectangle::fill_with(args.rect.dim(), color::TRANSPARENT)
            .xy(args.rect.xy())
            .graphics_for(args.id)
            .scroll_kids()
            .scroll_kids_vertically()
            .set(state.ids.window, ui);
        Rectangle::fill_with([args.rect.w() / 2.0, args.rect.h()], color::TRANSPARENT)
            .top_right()
            .parent(state.ids.window)
            .set(state.ids.window_r, ui);
        Scrollbar::y_axis(state.ids.window)
            .thickness(5.0)
            .rgba(0.33, 0.33, 0.33, 1.0)
            .set(state.ids.window_scrollbar, ui);

        let bar_values = self.global_state.settings.interface.bar_numbers;
        let crosshair_transp = self.global_state.settings.interface.crosshair_transp;
        let crosshair_type = self.global_state.settings.interface.crosshair_type;
        let ui_scale = self.global_state.settings.interface.ui_scale;
        let chat_transp = self.global_state.settings.interface.chat_transp;

        Text::new(&self.localized_strings.get("hud.settings.general"))
            .top_left_with_margins_on(state.ids.window, 5.0, 5.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.general_txt, ui);

        // Help
        let show_help = ToggleButton::new(
            self.show.help,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .w_h(18.0, 18.0)
        .down_from(state.ids.general_txt, 20.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.button_help, ui);

        if self.show.help != show_help {
            events.push(Event::ToggleHelp);
        }

        Text::new(&self.localized_strings.get("hud.settings.help_window"))
            .right_from(state.ids.button_help, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.button_help)
            .color(TEXT_COLOR)
            .set(state.ids.show_help_label, ui);

        // Loading Screen Tips
        let show_tips = ToggleButton::new(
            self.global_state.settings.interface.loading_tips,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .w_h(18.0, 18.0)
        .down_from(state.ids.button_help, 8.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.load_tips_button, ui);

        if self.global_state.settings.interface.loading_tips != show_tips {
            events.push(Event::ToggleTips(
                !self.global_state.settings.interface.loading_tips,
            ));
        }

        Text::new(&self.localized_strings.get("hud.settings.loading_tips"))
            .right_from(state.ids.load_tips_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.load_tips_button)
            .color(TEXT_COLOR)
            .set(state.ids.load_tips_button_label, ui);
        // Debug
        let show_debug = ToggleButton::new(
            self.show.debug,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .w_h(18.0, 18.0)
        .down_from(state.ids.load_tips_button, 8.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.debug_button, ui);

        if self.show.debug != show_debug {
            events.push(Event::ToggleDebug);
        }

        Text::new(&self.localized_strings.get("hud.settings.debug_info"))
            .right_from(state.ids.debug_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.debug_button)
            .color(TEXT_COLOR)
            .set(state.ids.debug_button_label, ui);

        // Ui Scale
        Text::new(&self.localized_strings.get("hud.settings.ui_scale"))
            .down_from(state.ids.debug_button, 20.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.ui_scale_label, ui);

        // Relative Scaling Button
        let (check_img, check_mo_img, check_press_img, relative_selected) = match ui_scale {
            ScaleMode::RelativeToWindow(_) => (
                self.imgs.check_checked,
                self.imgs.check_checked_mo,
                self.imgs.check_checked,
                true,
            ),
            ScaleMode::Absolute(_) | ScaleMode::DpiFactor => (
                self.imgs.check,
                self.imgs.check_mo,
                self.imgs.check_press,
                false,
            ),
        };
        if Button::image(check_img)
            .w_h(12.0, 12.0)
            .down_from(state.ids.ui_scale_label, 20.0)
            .hover_image(check_mo_img)
            .press_image(check_press_img)
            .set(state.ids.relative_to_win_button, ui)
            .was_clicked()
            && !relative_selected
        {
            events.push(Event::UiScale(ScaleChange::ToRelative));
        }

        Text::new(self.localized_strings.get("hud.settings.relative_scaling"))
            .right_from(state.ids.relative_to_win_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.relative_to_win_button)
            .color(TEXT_COLOR)
            .set(state.ids.relative_to_win_text, ui);

        // Absolute Scaling Button
        let (check_img, check_mo_img, check_press_img, absolute_selected) = match ui_scale {
            ScaleMode::Absolute(_) => (
                self.imgs.check_checked,
                self.imgs.check_checked_mo,
                self.imgs.check_checked,
                true,
            ),
            ScaleMode::RelativeToWindow(_) | ScaleMode::DpiFactor => (
                self.imgs.check,
                self.imgs.check_mo,
                self.imgs.check_press,
                false,
            ),
        };
        if Button::image(check_img)
            .w_h(12.0, 12.0)
            .down_from(state.ids.relative_to_win_button, 8.0)
            .hover_image(check_mo_img)
            .press_image(check_press_img)
            .set(state.ids.absolute_scale_button, ui)
            .was_clicked()
            && !absolute_selected
        {
            events.push(Event::UiScale(ScaleChange::ToAbsolute));
        }

        Text::new(self.localized_strings.get("hud.settings.custom_scaling"))
            .right_from(state.ids.absolute_scale_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.absolute_scale_button)
            .color(TEXT_COLOR)
            .set(state.ids.absolute_scale_text, ui);

        // Slider -> Inactive when "Relative to window" is selected
        if let ScaleMode::Absolute(scale) = ui_scale {
            if let Some(new_val) = ImageSlider::continuous(
                scale.log(2.0),
                0.5f64.log(2.0),
                2.0f64.log(2.0),
                self.imgs.slider_indicator,
                self.imgs.slider,
            )
            .w_h(208.0, 22.0)
            .right_from(state.ids.absolute_scale_text, 12.0)
            .track_breadth(30.0)
            .slider_length(10.0)
            .pad_track((5.0, 5.0))
            .set(state.ids.ui_scale_slider, ui)
            {
                events.push(Event::UiScale(ScaleChange::Adjust(2.0f64.powf(new_val))));
            }
            // Custom Scaling Text
            Text::new(&format!("{:.2}", scale))
                .right_from(state.ids.ui_scale_slider, 10.0)
                .font_size(self.fonts.cyri.scale(14))
                .font_id(self.fonts.cyri.conrod_id)
                .color(TEXT_COLOR)
                .set(state.ids.ui_scale_value, ui);
        } else {
            // Grey and unfunctional slider when Relative is selected
            ImageSlider::continuous(0.0, 0.0, 1.0, self.imgs.nothing, self.imgs.slider)
                .w_h(208.0, 22.0)
                .right_from(state.ids.absolute_scale_text, 10.0)
                .track_breadth(12.0)
                .slider_length(10.0)
                .track_color(Color::Rgba(1.0, 1.0, 1.0, 0.2))
                .slider_color(Color::Rgba(1.0, 1.0, 1.0, 0.2))
                .pad_track((5.0, 5.0))
                .set(state.ids.ui_scale_slider, ui);
        }

        // Crosshair Options
        // Crosshair Types
        // Round
        if Button::image(if let CrosshairType::Round = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg
        })
        .w_h(15.0 * 4.0, 15.0 * 4.0)
        .hover_image(if let CrosshairType::Round = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_hover
        })
        .press_image(if let CrosshairType::Round = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_press
        })
        .down_from(state.ids.ch_title, 20.0)
        .set(state.ids.ch_1_bg, ui)
        .was_clicked()
        {
            events.push(Event::CrosshairType(CrosshairType::Round));
        }

        // Crosshair
        Image::new(self.imgs.crosshair_outer_round)
            .w_h(20.0 * 1.5, 20.0 * 1.5)
            .middle_of(state.ids.ch_1_bg)
            .color(Some(Color::Rgba(
                1.0,
                1.0,
                1.0,
                self.global_state.settings.interface.crosshair_transp,
            )))
            .graphics_for(state.ids.ch_1_bg)
            .set(state.ids.crosshair_outer_1, ui);
        Image::new(self.imgs.crosshair_inner)
            .w_h(21.0 * 2.0, 21.0 * 2.0)
            .middle_of(state.ids.crosshair_outer_1)
            .color(Some(Color::Rgba(1.0, 1.0, 1.0, 0.6)))
            .graphics_for(state.ids.ch_1_bg)
            .set(state.ids.crosshair_inner_1, ui);

        // Rounded Edges
        if Button::image(if let CrosshairType::RoundEdges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg
        })
        .w_h(15.0 * 4.0, 15.0 * 4.0)
        .hover_image(if let CrosshairType::RoundEdges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_hover
        })
        .press_image(if let CrosshairType::RoundEdges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_press
        })
        .right_from(state.ids.ch_1_bg, 20.0)
        .set(state.ids.ch_2_bg, ui)
        .was_clicked()
        {
            events.push(Event::CrosshairType(CrosshairType::RoundEdges));
        }

        // Crosshair
        Image::new(self.imgs.crosshair_outer_round_edges)
            .w_h(21.0 * 1.5, 21.0 * 1.5)
            .middle_of(state.ids.ch_2_bg)
            .color(Some(Color::Rgba(
                1.0,
                1.0,
                1.0,
                self.global_state.settings.interface.crosshair_transp,
            )))
            .graphics_for(state.ids.ch_2_bg)
            .set(state.ids.crosshair_outer_2, ui);
        Image::new(self.imgs.crosshair_inner)
            .w_h(21.0 * 2.0, 21.0 * 2.0)
            .middle_of(state.ids.crosshair_outer_2)
            .color(Some(Color::Rgba(1.0, 1.0, 1.0, 0.6)))
            .graphics_for(state.ids.ch_2_bg)
            .set(state.ids.crosshair_inner_2, ui);

        // Edges
        if Button::image(if let CrosshairType::Edges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg
        })
        .w_h(15.0 * 4.0, 15.0 * 4.0)
        .hover_image(if let CrosshairType::Edges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_hover
        })
        .press_image(if let CrosshairType::Edges = crosshair_type {
            self.imgs.crosshair_bg_pressed
        } else {
            self.imgs.crosshair_bg_press
        })
        .right_from(state.ids.ch_2_bg, 20.0)
        .set(state.ids.ch_3_bg, ui)
        .was_clicked()
        {
            events.push(Event::CrosshairType(CrosshairType::Edges));
        }

        // Crosshair
        Image::new(self.imgs.crosshair_outer_edges)
            .w_h(21.0 * 1.5, 21.0 * 1.5)
            .middle_of(state.ids.ch_3_bg)
            .color(Some(Color::Rgba(
                1.0,
                1.0,
                1.0,
                self.global_state.settings.interface.crosshair_transp,
            )))
            .graphics_for(state.ids.ch_3_bg)
            .set(state.ids.crosshair_outer_3, ui);
        Image::new(self.imgs.crosshair_inner)
            .w_h(21.0 * 2.0, 21.0 * 2.0)
            .middle_of(state.ids.crosshair_outer_3)
            .color(Some(Color::Rgba(1.0, 1.0, 1.0, 0.6)))
            .graphics_for(state.ids.ch_3_bg)
            .set(state.ids.crosshair_inner_3, ui);
        // Crosshair Transparency Text and Slider
        Text::new(&self.localized_strings.get("hud.settings.crosshair"))
            .down_from(state.ids.absolute_scale_button, 20.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.ch_title, ui);
        Text::new(&self.localized_strings.get("hud.settings.transparency"))
            .right_from(state.ids.ch_3_bg, 20.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.ch_transp_text, ui);

        if let Some(new_val) = ImageSlider::continuous(
            crosshair_transp,
            0.0,
            1.0,
            self.imgs.slider_indicator,
            self.imgs.slider,
        )
        .w_h(104.0, 22.0)
        .down_from(state.ids.ch_transp_text, 8.0)
        .track_breadth(12.0)
        .slider_length(10.0)
        .pad_track((5.0, 5.0))
        .set(state.ids.ch_transp_slider, ui)
        {
            events.push(Event::CrosshairTransp(new_val));
        }

        Text::new(&format!("{:.2}", crosshair_transp,))
            .right_from(state.ids.ch_transp_slider, 8.0)
            .font_size(self.fonts.cyri.scale(14))
            .graphics_for(state.ids.ch_transp_slider)
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.ch_transp_value, ui);

        // Hotbar text
        Text::new(&self.localized_strings.get("hud.settings.hotbar"))
            .down_from(state.ids.ch_1_bg, 20.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.hotbar_title, ui);
        // Show Shortcut Numbers
        if Button::image(
            match self.global_state.settings.interface.shortcut_numbers {
                ShortcutNumbers::On => self.imgs.checkbox_checked,
                ShortcutNumbers::Off => self.imgs.checkbox,
            },
        )
        .w_h(18.0, 18.0)
        .hover_image(
            match self.global_state.settings.interface.shortcut_numbers {
                ShortcutNumbers::On => self.imgs.checkbox_checked_mo,
                ShortcutNumbers::Off => self.imgs.checkbox_mo,
            },
        )
        .press_image(
            match self.global_state.settings.interface.shortcut_numbers {
                ShortcutNumbers::On => self.imgs.checkbox_checked,
                ShortcutNumbers::Off => self.imgs.checkbox_press,
            },
        )
        .down_from(state.ids.hotbar_title, 8.0)
        .set(state.ids.show_shortcuts_button, ui)
        .was_clicked()
        {
            match self.global_state.settings.interface.shortcut_numbers {
                ShortcutNumbers::On => {
                    events.push(Event::ToggleShortcutNumbers(ShortcutNumbers::Off))
                },
                ShortcutNumbers::Off => {
                    events.push(Event::ToggleShortcutNumbers(ShortcutNumbers::On))
                },
            }
        }
        Text::new(&self.localized_strings.get("hud.settings.toggle_shortcuts"))
            .right_from(state.ids.show_shortcuts_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_shortcuts_button)
            .color(TEXT_COLOR)
            .set(state.ids.show_shortcuts_text, ui);
        // Buff Position
        // Buffs above skills
        if Button::image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Bar => self.imgs.check_checked,
            BuffPosition::Map => self.imgs.check,
        })
        .w_h(12.0, 12.0)
        .hover_image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Bar => self.imgs.check_checked_mo,
            BuffPosition::Map => self.imgs.check_mo,
        })
        .press_image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Bar => self.imgs.check_checked,
            BuffPosition::Map => self.imgs.check_press,
        })
        .down_from(state.ids.show_shortcuts_button, 8.0)
        .set(state.ids.buff_pos_bar_button, ui)
        .was_clicked()
        {
            events.push(Event::BuffPosition(BuffPosition::Bar))
        }
        Text::new(&self.localized_strings.get("hud.settings.buffs_skillbar"))
            .right_from(state.ids.buff_pos_bar_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_shortcuts_button)
            .color(TEXT_COLOR)
            .set(state.ids.buff_pos_bar_text, ui);
        // Buffs left from minimap
        if Button::image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Map => self.imgs.check_checked,
            BuffPosition::Bar => self.imgs.check,
        })
        .w_h(12.0, 12.0)
        .hover_image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Map => self.imgs.check_checked_mo,
            BuffPosition::Bar => self.imgs.check_mo,
        })
        .press_image(match self.global_state.settings.interface.buff_position {
            BuffPosition::Map => self.imgs.check_checked,
            BuffPosition::Bar => self.imgs.check_press,
        })
        .down_from(state.ids.buff_pos_bar_button, 8.0)
        .set(state.ids.buff_pos_map_button, ui)
        .was_clicked()
        {
            events.push(Event::BuffPosition(BuffPosition::Map))
        }
        Text::new(&self.localized_strings.get("hud.settings.buffs_mmap"))
            .right_from(state.ids.buff_pos_map_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_shortcuts_button)
            .color(TEXT_COLOR)
            .set(state.ids.buff_pos_map_text, ui);

        // Content Right Side

        /*Scrolling Combat text

        O Show Damage Numbers
            O Show single Damage Numbers
            O Show batched dealt Damage
            O Show incoming Damage
                O Batch incoming Numbers

        Number Display Duration: 1s ----I----5s
            */
        // SCT/ Scrolling Combat Text
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.scrolling_combat_text"),
        )
        .top_left_with_margins_on(state.ids.window_r, 5.0, 5.0)
        .font_size(self.fonts.cyri.scale(18))
        .font_id(self.fonts.cyri.conrod_id)
        .color(TEXT_COLOR)
        .set(state.ids.sct_title, ui);
        // Generally toggle the SCT
        let show_sct = ToggleButton::new(
            self.global_state.settings.interface.sct,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .w_h(18.0, 18.0)
        .down_from(state.ids.sct_title, 20.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.sct_show_radio, ui);

        if self.global_state.settings.interface.sct != show_sct {
            events.push(Event::Sct(!self.global_state.settings.interface.sct))
        }
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.scrolling_combat_text"),
        )
        .right_from(state.ids.sct_show_radio, 10.0)
        .font_size(self.fonts.cyri.scale(14))
        .font_id(self.fonts.cyri.conrod_id)
        .graphics_for(state.ids.sct_show_radio)
        .color(TEXT_COLOR)
        .set(state.ids.sct_show_text, ui);
        if self.global_state.settings.interface.sct {
            // Toggle single damage numbers
            let show_sct_damage_batch = !ToggleButton::new(
                !self.global_state.settings.interface.sct_damage_batch,
                self.imgs.checkbox,
                self.imgs.checkbox_checked,
            )
            .w_h(18.0, 18.0)
            .down_from(state.ids.sct_show_text, 8.0)
            .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
            .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
            .set(state.ids.sct_single_dmg_radio, ui);

            Text::new(
                &self
                    .localized_strings
                    .get("hud.settings.single_damage_number"),
            )
            .right_from(state.ids.sct_single_dmg_radio, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.sct_single_dmg_radio)
            .color(TEXT_COLOR)
            .set(state.ids.sct_single_dmg_text, ui);
            // Toggle Batched Damage
            let show_sct_damage_batch = ToggleButton::new(
                show_sct_damage_batch,
                self.imgs.checkbox,
                self.imgs.checkbox_checked,
            )
            .w_h(18.0, 18.0)
            .down_from(state.ids.sct_single_dmg_radio, 8.0)
            .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
            .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
            .set(state.ids.sct_show_batch_radio, ui);

            if self.global_state.settings.interface.sct_damage_batch != show_sct_damage_batch {
                events.push(Event::SctDamageBatch(
                    !self.global_state.settings.interface.sct_damage_batch,
                ))
            }
            Text::new(&self.localized_strings.get("hud.settings.cumulated_damage"))
                .right_from(state.ids.sct_show_batch_radio, 10.0)
                .font_size(self.fonts.cyri.scale(14))
                .font_id(self.fonts.cyri.conrod_id)
                .graphics_for(state.ids.sct_batched_dmg_radio)
                .color(TEXT_COLOR)
                .set(state.ids.sct_show_batch_text, ui);
            // Toggle Incoming Damage
            let show_sct_player_batch = !ToggleButton::new(
                !self.global_state.settings.interface.sct_player_batch,
                self.imgs.checkbox,
                self.imgs.checkbox_checked,
            )
            .w_h(18.0, 18.0)
            .down_from(state.ids.sct_show_batch_radio, 8.0)
            .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
            .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
            .set(state.ids.sct_inc_dmg_radio, ui);

            Text::new(&self.localized_strings.get("hud.settings.incoming_damage"))
                .right_from(state.ids.sct_inc_dmg_radio, 10.0)
                .font_size(self.fonts.cyri.scale(14))
                .font_id(self.fonts.cyri.conrod_id)
                .graphics_for(state.ids.sct_inc_dmg_radio)
                .color(TEXT_COLOR)
                .set(state.ids.sct_inc_dmg_text, ui);
            // Toggle Batched Incoming Damage
            let show_sct_player_batch = ToggleButton::new(
                show_sct_player_batch,
                self.imgs.checkbox,
                self.imgs.checkbox_checked,
            )
            .w_h(18.0, 18.0)
            .down_from(state.ids.sct_inc_dmg_radio, 8.0)
            .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
            .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
            .set(state.ids.sct_batch_inc_radio, ui);

            if self.global_state.settings.interface.sct_player_batch != show_sct_player_batch {
                events.push(Event::SctPlayerBatch(
                    !self.global_state.settings.interface.sct_player_batch,
                ))
            }
            Text::new(
                &self
                    .localized_strings
                    .get("hud.settings.cumulated_incoming_damage"),
            )
            .right_from(state.ids.sct_batch_inc_radio, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.sct_batch_inc_radio)
            .color(TEXT_COLOR)
            .set(state.ids.sct_batch_inc_text, ui);
        }

        // Speech bubble dark mode
        Text::new(&self.localized_strings.get("hud.settings.speech_bubble"))
            .down_from(
                if self.global_state.settings.interface.sct {
                    state.ids.sct_batch_inc_radio
                } else {
                    state.ids.sct_show_radio
                },
                20.0,
            )
            .x_align(Align::Start)
            .x_relative_to(state.ids.sct_show_text, -40.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.speech_bubble_text, ui);
        let speech_bubble_dark_mode = ToggleButton::new(
            self.global_state.settings.interface.speech_bubble_dark_mode,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .down_from(state.ids.speech_bubble_text, 10.0)
        .w_h(18.0, 18.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.speech_bubble_dark_mode_button, ui);
        if self.global_state.settings.interface.speech_bubble_dark_mode != speech_bubble_dark_mode {
            events.push(Event::SpeechBubbleDarkMode(speech_bubble_dark_mode));
        }
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.speech_bubble_dark_mode"),
        )
        .right_from(state.ids.speech_bubble_dark_mode_button, 10.0)
        .font_size(self.fonts.cyri.scale(15))
        .font_id(self.fonts.cyri.conrod_id)
        .color(TEXT_COLOR)
        .set(state.ids.speech_bubble_dark_mode_text, ui);
        // Speech bubble icon
        let speech_bubble_icon = ToggleButton::new(
            self.global_state.settings.interface.speech_bubble_icon,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .down_from(state.ids.speech_bubble_dark_mode_button, 10.0)
        .w_h(18.0, 18.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.speech_bubble_icon_button, ui);
        if self.global_state.settings.interface.speech_bubble_icon != speech_bubble_icon {
            events.push(Event::SpeechBubbleIcon(speech_bubble_icon));
        }
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.speech_bubble_icon"),
        )
        .right_from(state.ids.speech_bubble_icon_button, 10.0)
        .font_size(self.fonts.cyri.scale(15))
        .font_id(self.fonts.cyri.conrod_id)
        .color(TEXT_COLOR)
        .set(state.ids.speech_bubble_icon_text, ui);

        // Energybars Numbers
        // Hotbar text
        Text::new(&self.localized_strings.get("hud.settings.energybar_numbers"))
            .down_from(state.ids.speech_bubble_icon_button, 20.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.bar_numbers_title, ui);

        // None
        if Button::image(if let BarNumbers::Off = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check
        })
        .w_h(12.0, 12.0)
        .hover_image(if let BarNumbers::Off = bar_values {
            self.imgs.check_checked_mo
        } else {
            self.imgs.check_mo
        })
        .press_image(if let BarNumbers::Off = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check_press
        })
        .down_from(state.ids.bar_numbers_title, 8.0)
        .set(state.ids.show_bar_numbers_none_button, ui)
        .was_clicked()
        {
            events.push(Event::ToggleBarNumbers(BarNumbers::Off))
        }
        Text::new(&self.localized_strings.get("hud.settings.none"))
            .right_from(state.ids.show_bar_numbers_none_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_bar_numbers_none_button)
            .color(TEXT_COLOR)
            .set(state.ids.show_bar_numbers_none_text, ui);

        // Values
        if Button::image(if let BarNumbers::Values = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check
        })
        .w_h(12.0, 12.0)
        .hover_image(if let BarNumbers::Values = bar_values {
            self.imgs.check_checked_mo
        } else {
            self.imgs.check_mo
        })
        .press_image(if let BarNumbers::Values = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check_press
        })
        .down_from(state.ids.show_bar_numbers_none_button, 8.0)
        .set(state.ids.show_bar_numbers_values_button, ui)
        .was_clicked()
        {
            events.push(Event::ToggleBarNumbers(BarNumbers::Values))
        }
        Text::new(&self.localized_strings.get("hud.settings.values"))
            .right_from(state.ids.show_bar_numbers_values_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_bar_numbers_values_button)
            .color(TEXT_COLOR)
            .set(state.ids.show_bar_numbers_values_text, ui);

        // Percentages
        if Button::image(if let BarNumbers::Percent = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check
        })
        .w_h(12.0, 12.0)
        .hover_image(if let BarNumbers::Percent = bar_values {
            self.imgs.check_checked_mo
        } else {
            self.imgs.check_mo
        })
        .press_image(if let BarNumbers::Percent = bar_values {
            self.imgs.check_checked
        } else {
            self.imgs.check_press
        })
        .down_from(state.ids.show_bar_numbers_values_button, 8.0)
        .set(state.ids.show_bar_numbers_percentage_button, ui)
        .was_clicked()
        {
            events.push(Event::ToggleBarNumbers(BarNumbers::Percent))
        }
        Text::new(&self.localized_strings.get("hud.settings.percentages"))
            .right_from(state.ids.show_bar_numbers_percentage_button, 10.0)
            .font_size(self.fonts.cyri.scale(14))
            .font_id(self.fonts.cyri.conrod_id)
            .graphics_for(state.ids.show_bar_numbers_percentage_button)
            .color(TEXT_COLOR)
            .set(state.ids.show_bar_numbers_percentage_text, ui);

        // Chat Transp
        Text::new(&self.localized_strings.get("hud.settings.chat"))
            .down_from(state.ids.show_bar_numbers_percentage_button, 20.0)
            .font_size(self.fonts.cyri.scale(18))
            .font_id(self.fonts.cyri.conrod_id)
            .color(TEXT_COLOR)
            .set(state.ids.chat_transp_title, ui);
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.background_transparency"),
        )
        .right_from(state.ids.chat_transp_slider, 20.0)
        .font_size(self.fonts.cyri.scale(14))
        .font_id(self.fonts.cyri.conrod_id)
        .color(TEXT_COLOR)
        .set(state.ids.chat_transp_text, ui);

        if let Some(new_val) = ImageSlider::continuous(
            chat_transp,
            0.0,
            0.9,
            self.imgs.slider_indicator,
            self.imgs.slider,
        )
        .w_h(104.0, 22.0)
        .down_from(state.ids.chat_transp_title, 8.0)
        .track_breadth(12.0)
        .slider_length(10.0)
        .pad_track((5.0, 5.0))
        .set(state.ids.chat_transp_slider, ui)
        {
            events.push(Event::ChatTransp(new_val));
        }

        // "Show character names in chat" toggle button
        let chat_char_name = ToggleButton::new(
            self.global_state.settings.interface.chat_character_name,
            self.imgs.checkbox,
            self.imgs.checkbox_checked,
        )
        .w_h(18.0, 18.0)
        .down_from(state.ids.chat_transp_slider, 20.0)
        .hover_images(self.imgs.checkbox_mo, self.imgs.checkbox_checked_mo)
        .press_images(self.imgs.checkbox_press, self.imgs.checkbox_checked)
        .set(state.ids.chat_char_name_button, ui);
        if self.global_state.settings.interface.chat_character_name != chat_char_name {
            events.push(Event::ChatCharName(
                !self.global_state.settings.interface.chat_character_name,
            ));
        }
        Text::new(
            &self
                .localized_strings
                .get("hud.settings.chat_character_name"),
        )
        .right_from(state.ids.chat_char_name_button, 20.0)
        .font_size(self.fonts.cyri.scale(14))
        .font_id(self.fonts.cyri.conrod_id)
        .color(TEXT_COLOR)
        .set(state.ids.chat_char_name_text, ui);

        // TODO Show account name in chat

        // Reset the interface settings to the default settings
        if Button::image(self.imgs.button)
            .w_h(RESET_BUTTONS_WIDTH, RESET_BUTTONS_HEIGHT)
            .hover_image(self.imgs.button_hover)
            .press_image(self.imgs.button_press)
            .down_from(state.ids.buff_pos_map_button, 12.0)
            .label(&self.localized_strings.get("hud.settings.reset_interface"))
            .label_font_size(self.fonts.cyri.scale(14))
            .label_color(TEXT_COLOR)
            .label_font_id(self.fonts.cyri.conrod_id)
            .label_y(Relative::Scalar(2.0))
            .set(state.ids.reset_interface_button, ui)
            .was_clicked()
        {
            events.push(Event::ResetInterfaceSettings);
        }

        events
    }
}