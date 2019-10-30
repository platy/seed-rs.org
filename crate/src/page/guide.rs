use crate::{generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB, Model, Guide, Route, MAIL_TO_KAVIK, previous_guide, next_guide};
use seed::{prelude::*, *};
use crate::Visibility::Hidden;

pub fn view(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.container,
            C.w_full,
            C.flex,
            C.flex_wrap,
            C.mx_auto,
            C.px_2,
            C.pt_8,
            C.mt_16,
            // lg__
            C.lg__pt_16,
        ],
        view_guide_list(guide, model).els(),
        view_content(guide, model).els(),
    ]
}

fn view_guide_list(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_xl,
            C.text_gray_800,
            C.leading_normal,
            // lg__
            C.lg__w_1of5,
            C.lg__px_6,
        ],
        view_guide_list_toggle(guide).els(),
        view_guide_list_items(guide, model).els(),
    ]
}

fn view_guide_list_toggle(selected_guide: &Guide) -> impl View<Msg> {
    div![
        class![
            C.sticky,
            C.inset_0,
            // lg__
            C.lg__hidden,
        ],
        button![
            id!("guide_list_toggle"),
            class![
                C.flex,
                C.w_full,
                C.justify_between,
                C.px_4,
                C.py_3,
                C.bg_white,
                C.border,
                C.rounded,
                C.border_gray_600,
                C.hover__border_purple_500,
                C.appearance_none,
                C.focus__outline_none,
                C.text_sm,
                C.font_bold,
                // lg__
                C.lg__bg_transparent,
            ],
            simple_ev(Ev::Click, Msg::ToggleGuideList),
            selected_guide.menu_title,
            svg![
                class![
                    C.fill_current,
                    C.h_5,
                    C.float_right,
                ],
                attrs!{
                    At::ViewBox => "0 0 20 20",
                },
                path![
                    attrs!{
                        At::D => "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z",
                    }
                ]
            ],
        ]
    ]
}

fn view_guide_list_items(selected_guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        id!("menu_items"),
        class![
            C.w_full,
            C.inset_0,
            C.hidden => model.guide_list_visibility == Hidden,
            C.overflow_x_hidden,
            C.overflow_y_auto,
            C.mt_0,
            C.border,
            C.border_gray_400,
            C.bg_white,
            C.shadow,
            C.z_20,
            // lg__
            C.lg__sticky,
            C.lg__overflow_y_hidden,
            C.lg__border_transparent,
            C.lg__shadow_none,
            C.lg__bg_transparent,
            C.lg__block,
        ],
        style! {
            St::Top => em(7),
        },
        view_search(model).els(),
        ul![
            model.guides.iter().map(|guide| {
                let guide_is_selected = guide == selected_guide;
                let guide_is_matched = model.matched_guides.contains(guide);
                view_guide_list_item(guide, guide_is_selected, guide_is_matched).els()
            })
        ]
    ]
}

fn view_search(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.flex_1,
            C.w_full,
            C.mx_auto,
            C.max_w_sm,
            C.content_center,
            C.py_4,
            // lg__
            C.lg__py_0,
        ],
        div![
            class![
                C.relative,
                C.pl_4,
                C.pr_4,
                // md__
                C.md__pr_0,
            ],
            input![
                class![
                    C.w_full,
                    C.bg_gray_100,
                    C.text_sm,
                    C.text_gray_800,
                    C.placeholder_gray_800,
                    C.border,
                    C.focus__outline_none,
                    C.focus__border_purple_500,
                    C.rounded,
                    C.py_1,
                    C.px_2,
                    C.pl_10,
                    C.appearance_none,
                    C.leading_normal,
                ],
                attrs!{
                    At::Type => "search",
                    At::Placeholder => "Search",
                    At::Value => model.search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ],
            div![
                class![
                    C.absolute,
                ],
                style!{
                    St::Top => rem(0.375),
                    St::Left => rem(1.75),
                },
                svg![
                    class![
                        C.fill_current,
                        C.pointer_events_none,
                        C.text_gray_800,
                        C.w_4,
                        C.h_4,
                    ],
                    attrs!{
                        At::ViewBox => "0 0 20 20",
                    },
                    path![
                        attrs!{
                            At::D => "M12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12zM12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z",
                        }
                    ]
                ],
            ]
        ]
    ]
}

fn view_guide_list_item(guide: &Guide, active: bool, matched: bool) -> impl View<Msg> {
    li![
        class![
            C.hover__bg_purple_100 => !matched,
            C.bg_purple_200 => matched,
            // md__
            C.md__my_0,
            // lg__
            C.lg__hover__bg_transparent => !matched,
        ],
        a![
            class![
                C.block,
                C.py_2,
                C.pl_4,
                C.align_middle,
                C.text_gray_700,
                C.hover__text_purple_500,
                C.border_l_4,
                C.border_transparent,
                C.focus__outline_none,
                // lg__
                C.lg__border_purple_500 => active,
                if active { C.lg__hover__border_purple_500 } else { C.lg__hover__border_purple_400 },
            ],
            attrs! {
                At::Href => Route::Guide(guide.slug.to_owned()).to_string(),
            },
            simple_ev(Ev::Click, Msg::HideGuideList),
            span![
                class![
                    C.block,
                    C.pb_1,
                    C.text_sm,
                    C.text_gray_900 => active,
                    C.font_bold => active,
                    // md__
                    C.md__pb_0,
                ],
                guide.menu_title,
            ]
        ]
    ]
}

fn view_content(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.p_8,
            C.mt_6,
            C.text_gray_900,
            C.leading_normal,
            C.bg_white,
            C.border,
            C.border_gray_400,
            C.rounded,
            // lg__
            C.lg__w_4of5,
            C.lg__mt_0,
        ],
        view_browsing_links(guide, &model.guides).els(),
        view_content_markdown(guide.html).els(),
        view_browsing_links(guide, &model.guides).els(),
    ]
}

fn view_content_top_back_link(selected_guide: &Guide, guides: &[Guide]) -> impl View<Msg> {
    if let Some(previous_guide) = previous_guide(selected_guide, guides) {
        div![
            class![
                C.font_sans,
            ],
            span![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                ],
                "< ",
                a![
                    class![
                        C.text_base,
                        C.text_purple_500,
                        C.font_bold,
                        C.hover__underline,
                        // md__
                        C.md__text_sm,
                    ],
                    attrs! {
                        At::Href => Route::Guide(previous_guide.slug.to_owned()).to_string(),
                    },
                    previous_guide.menu_title,
                ]
            ],
        ]
    } else {
        empty![]
    }
}

fn view_content_markdown(content: &str) -> impl View<Msg> {
    div![
        class![
            "markdown"
        ],
        raw!(content)
    ]
}

fn view_browsing_links(selected_guide: &Guide, guides: &[Guide]) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_gray_500,
            C.px_4,
            C.py_6,
            C.flex,
            C.justify_between,
            // md__
            C.md__text_sm,
            // lg__
            C.lg__ml_auto,
            C.text_base,
        ],
        if let Some(previous_guide) = previous_guide(selected_guide, guides) {
            div![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.flex,
                ],
                span!["<"],
                a![
                    class![
                        C.text_base,
                        C.text_purple_500,
                        C.font_bold,
                        C.hover__underline,
                        // md__
                        C.md__text_sm,
                    ],
                    attrs! {
                        At::Href => Route::Guide(previous_guide.slug.to_owned()).to_string(),
                    },
                    previous_guide.menu_title,
                ],
            ]
        } else {
            empty![]
        },
        // spacer
        div![
            class![
                C.w_5,
            ]
        ],
        if let Some(next_guide) = next_guide(selected_guide, guides) {
            div![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.flex,
                ],
                a![
                    class![
                        C.text_base,
                        C.text_purple_500,
                        C.font_bold,
                        C.hover__underline,
                        // md__
                        C.md__text_sm,
                    ],
                    attrs! {
                        At::Href => Route::Guide(next_guide.slug.to_owned()).to_string(),
                    },
                    next_guide.menu_title,
                ],
                span![">"],
            ]
        } else {
            empty![]
        }
    ]
}