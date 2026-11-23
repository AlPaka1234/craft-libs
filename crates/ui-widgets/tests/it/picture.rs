//! `Picture` sized in device pixels.

use craft_ui::contract::Show as _;
use craft_ui::display::{Picture, PictureSize};
use craft_ui::runtime::env::Scale;
use craft_ui::testing::{Fixture, trial};
use egui::{Color32, ColorImage, Vec2};
use std::cell::Cell;
use std::rc::Rc;

/// The side, in texels, of the picture's image.
const SIDE: usize = 32;

fn image() -> ColorImage {
    ColorImage::new([SIDE, SIDE], vec![Color32::from_rgb(40, 120, 200); SIDE * SIDE])
}

/// A `DevicePixels` picture keeps one texel to a device pixel at every display scale: its size in points times the
/// scale is its texel size.
#[trial]
fn a_device_pixel_picture_keeps_one_texel_to_a_pixel_at_every_scale() {
    for scale in [1.0_f32, 2.0] {
        let size = Rc::new(Cell::new(Vec2::ZERO));
        let seen = size.clone();
        let mut fixture = Fixture::new(move |ui| {
            seen.set(Picture::new("Gradient", 1, &image).extent(PictureSize::DevicePixels).show(ui).rect.size());
        });
        fixture.scale(Scale::new(scale));
        fixture.settle();
        let device = size.get() * scale;
        assert!(
            (device.x - SIDE as f32).abs() < 0.01 && (device.y - SIDE as f32).abs() < 0.01,
            "at scale {scale} the picture is {device:?} device pixels, not {SIDE} square"
        );
    }
}

/// How many shapes a rendered picture of a transparent image paints, with or without its transparency grid.
fn shapes_of_rendered(checker: bool) -> usize {
    use craft_ui::list::ThumbSize;
    use craft_ui::primitives::textures::{FakeThumbSource, ThumbnailOptions, Thumbnails};
    let mut fixture = Fixture::new(move |ui| {
        let options = ThumbnailOptions::new().jobs(std::sync::Arc::new(craft_host::InlineJobs));
        let thumbnails = Thumbnails::<u64>::in_context(ui.ctx(), "picture_checker_test", options);
        let source = FakeThumbSource::<u64>::new();
        Picture::rendered("Mask", 1, &thumbnails, &source).extent(PictureSize::Thumb(ThumbSize::Large)).checker(checker).show(ui);
    });
    fixture.settle();
    fixture.frame_shapes().len()
}

#[trial]
fn a_rendered_picture_paints_a_checker_unless_it_opts_out() {
    let with = shapes_of_rendered(true);
    let without = shapes_of_rendered(false);
    assert!(with > without, "the default paints {with} shapes, the opt-out {without}: the grid is the difference");
}
