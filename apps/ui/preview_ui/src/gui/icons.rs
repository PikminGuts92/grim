use egui_extras::image::RetainedImage;
use font_awesome_as_a_crate as fa;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FA_GRID: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_grid",
        fa::svg(fa::Type::Solid, "table-cells").unwrap()
    ).unwrap();

    pub static ref FA_CUBES: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_cubes",
        fa::svg(fa::Type::Solid, "cubes").unwrap()
    ).unwrap();

    pub static ref FA_REFRESH: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_refresh",
        fa::svg(fa::Type::Solid, "arrows-rotate").unwrap()
    ).unwrap();

    pub static ref FA_ARROWS_MULTI: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_arrows_multi",
        fa::svg(fa::Type::Solid, "arrows-up-down-left-right").unwrap()
    ).unwrap();

    pub static ref FA_EYE: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_eye",
        fa::svg(fa::Type::Solid, "eye").unwrap()
    ).unwrap();

    pub static ref FA_TRASH: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_trash",
        fa::svg(fa::Type::Solid, "trash-can").unwrap()
    ).unwrap();

    pub static ref FA_EDIT: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_pen_to_square",
        fa::svg(fa::Type::Solid, "pen-to-square").unwrap()
    ).unwrap();

    pub static ref FA_SEARCH: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_magnifying_glass",
        fa::svg(fa::Type::Solid, "magnifying-glass").unwrap()
    ).unwrap();

    // Pro icon :(
    /* pub static ref FA_GLOBE: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_globe",
        fa::svg(fa::Type::Regular, "globe").unwrap()
    ).unwrap(); */

    pub static ref FA_CIRCLE: RetainedImage = egui_extras::RetainedImage::from_svg_str(
        "fa_circle",
        fa::svg(fa::Type::Regular, "circle").unwrap()
    ).unwrap();
}