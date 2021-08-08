use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_anim_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub frame: f32 }.into(),
        quote! { pub rate: grim_traits::scene::AnimRate }.into(),
    ];

    let trait_impl = quote! {
        fn get_frame(&self) -> f32 {
            self.frame
        }

        fn set_frame(&mut self, frame: f32) {
            self.frame = frame;
        }

        fn get_rate(&self) -> &grim_traits::scene::AnimRate {
            &self.rate
        }

        fn get_rate_mut(&mut self) -> &mut grim_traits::scene::AnimRate {
            &mut self.rate
        }

        fn set_rate(&mut self, rate: grim_traits::scene::AnimRate) {
            self.rate = rate;
        }
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}