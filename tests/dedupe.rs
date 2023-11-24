// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │  ██████╗ ██████╗  ██████╗   Copyright (C) 2022, The Prospective Company   │
// │  ██╔══██╗██╔══██╗██╔═══██╗                                                │
// │  ██████╔╝██████╔╝██║   ██║  This file is part of the Procss library,      │
// │  ██╔═══╝ ██╔══██╗██║   ██║  distributed under the terms of the            │
// │  ██║     ██║  ██║╚██████╔╝  Apache License 2.0.  The full license can     │
// │  ╚═╝     ╚═╝  ╚═╝ ╚═════╝   be found in the LICENSE file.                 │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

#![feature(assert_matches)]

#[cfg(test)]
use std::assert_matches::assert_matches;

use procss::transformers::{self};
use procss::{parse, RenderCss};

#[rustfmt::skip]
#[test]
fn test_advanced_mixin() {
    let test_data =
        "
        div[theme=\"custom\"] {
            @include div-theme;
        }
        
        perspective-copy-menu[theme=\"custom\"],
        .perspective-modal-theme {
            @include perspective-modal-theme;
        }
        
        @mixin div-theme {
            @include div-theme--dimensions;
            @include div-theme--colors;
            @include div-theme--fonts;
            @include div-theme--intl;
            @include div-theme--chart;
            @include div-theme--datagrid;
            @include div-theme--openlayers;
        }
        
        @mixin perspective-modal-theme {
            @include div-theme--fonts;
            @include div-theme--colors;
            background-color: white;
            --column-style-pos-color--content: \"add\";
        }
        @mixin div-theme--dimensions {
            --button--font-size: 16px;
            --config-button--padding: 15px 8px 6px 8px;
            // Comment comment comment
        }
        
        @mixin div-theme--colors {
            color: #161616;
            background-color: #f2f4f6;
        }
        
        @mixin div-theme--fonts {
            font-family: \"Open Sans\";
            --interface-monospace--font-family: \"Roboto Mono\";
            --button--font-family: \"theme Icons\";
        }
        
        @mixin div-theme--intl {
            // Query overlay labels
            --group_by--content: \"Group By\";
            --split_by--content: \"Split By\";
        
            // Icons
            --inactive-column-selector--content: \"\\E835\";
            --active-column-selector--content: \"\\E834\";
        }
        
        @mixin div-theme--chart {
            --chart-y1-label--content: \"arrow_upward\";
            --chart-y2-label--content: \"arrow_downward\";
            --chart-full--gradient: linear-gradient(
                #4d342f 0%,
                #e4521b 22.5%,
                #feeb65 42.5%,
                #f0f0f0 50%,
                #dcedc8 57.5%,
                #42b3d5 67.5%,
                #1a237e 100%
            );
            --chart-positive--gradient: linear-gradient(
                #f0f0f0 0%,
                #dcedc8 10%,
                #42b3d5 50%,
                #1a237e 100%
            );
            --chart-negative--gradient: linear-gradient(
                #4d342f 0%,
                #e4521b 50%,
                #feeb65 90%,
        
                #f0f0f0 100%
            );
        }
        
        @mixin div-theme--openlayers {
            --map-tile-url: \"http://{a-c}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png\";
        }
        
        @mixin div-theme--datagrid {
            --column-style-open-button--content: \"menu\";
            --column-style-close-button--content: \"expand_less\";
        
            table::-webkit-scrollbar-thumb {
                background-color: transparent;
            }
        
            table:hover::-webkit-scrollbar-thumb {
                background-color: #e0e4e9;
            }
        }
        ";

    assert_matches!(
        parse(test_data)
            .map(|mut tree| {
                transformers::apply_mixin(&mut tree);
                let mut flat = tree.flatten_tree();
                transformers::merge_siblings(&mut flat);
                transformers::remove_mixin(&mut flat);
                flat.as_css_string()
            })
            .as_deref(),
            Ok("div[theme=\"custom\"]{--button--font-size:16px;--config-button--padding:15px 8px 6px 8px;color:#161616;background-color:#f2f4f6;font-family:\"Open Sans\";--interface-monospace--font-family:\"Roboto Mono\";--button--font-family:\"theme Icons\";--group_by--content:\"Group By\";--split_by--content:\"Split By\";--inactive-column-selector--content:\"\\E835\";--active-column-selector--content:\"\\E834\";--chart-y1-label--content:\"arrow_upward\";--chart-y2-label--content:\"arrow_downward\";--chart-full--gradient:linear-gradient(#4d342f 0%,#e4521b 22.5%,#feeb65 42.5%,#f0f0f0 50%,#dcedc8 57.5%,#42b3d5 67.5%,#1a237e 100%);--chart-positive--gradient:linear-gradient(#f0f0f0 0%,#dcedc8 10%,#42b3d5 50%,#1a237e 100%);--chart-negative--gradient:linear-gradient(#4d342f 0%,#e4521b 50%,#feeb65 90%,#f0f0f0 100%);--column-style-open-button--content:\"menu\";--column-style-close-button--content:\"expand_less\";}div[theme=\"custom\"] table::-webkit-scrollbar-thumb{background-color:transparent;}div[theme=\"custom\"] table:hover::-webkit-scrollbar-thumb{background-color:#e0e4e9;}div[theme=\"custom\"]{--map-tile-url:\"http://{a-c}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png\";}perspective-copy-menu[theme=\"custom\"],.perspective-modal-theme{font-family:\"Open Sans\";--interface-monospace--font-family:\"Roboto Mono\";--button--font-family:\"theme Icons\";color:#161616;background-color:#f2f4f6;background-color:white;--column-style-pos-color--content:\"add\";}")
    )
}
