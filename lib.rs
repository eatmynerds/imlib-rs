#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use self::{
    imlib_context_new as ImlibContextNew, imlib_context_pop as ImlibContextPop,
    imlib_context_push as ImlibContextPush, imlib_context_set_blend as ImlibContextSetBlend,
    imlib_context_set_color_range as ImlibContextSetColorRange,
    imlib_context_set_colormap as ImlibContextSetColormap,
    imlib_context_set_display as ImlibContextSetDisplay,
    imlib_context_set_dither as ImlibContextSetDither,
    imlib_context_set_drawable as ImlibContextSetDrawable,
    imlib_context_set_image as ImlibContextSetImage,
    imlib_context_set_visual as ImlibContextSetVisual,
    imlib_create_color_range as ImlibCreateColorRange,
    imlib_create_cropped_scaled_image as ImlibCreateCroppedScaledImage,
    imlib_free_image_and_decache as ImlibFreeImageAndDecache,
    imlib_image_get_height as ImlibImageGetHeight, imlib_image_get_width as ImlibImageGetWidth,
    imlib_load_image as ImlibLoadImage,
    imlib_render_image_on_drawable as ImlibRenderImageOnDrawable,
    imlib_save_image as ImlibSaveImage, Imlib_Context as ImlibContext, Imlib_Image as ImlibImage,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
