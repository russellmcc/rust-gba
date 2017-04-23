use quote::Tokens;
use syn::Ident;
use std::rc::Rc;
use gba_register_generator::*;

pub fn tokens() -> Tokens {
    let registers = [
        RegisterBuilder::default()
            .title(Ident::new("DisplayControl"))
            .doc("Controls property of the display.")
            .repr(Ident::new("u16"))
            .fields(vec![
                FieldBuilder::default()
                    .name(Ident::new("video_mode"))
                    .doc("Current mode of the video controller.")
                    .start(0usize)
                    .size(3usize)
                    .format(Format::Type{
                        type_ident: Ident::new("u8"),
                        as_bits: None,
                        from_bits: None,
                    })
                    .access(Access::ReadWrite)
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("display_buffer"))
                    .doc("In double-buffered modes 4 and 5, controls
which buffer is currently being displayed.  In other modes, has no effect.")
                    .start(4usize)
                    .size(1usize)
                    .access(Access::ReadWrite)
                    .format(Format::Type {
                        type_ident: Ident::new("u8"),
                        as_bits: None,
                        from_bits: None,
                    })
                    .access(Access::ReadWrite)
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("hblank_fast_oam_access"))
                    .doc("Allows access to OAM memory during hblank.
Enabling this will reduce th number of visible sprites.")
                    .start(5usize)
                    .size(1usize)
                    .format(Format::Bool {
                        quick_set_true: None,
                        quick_set_false: None,
                    })
                    .access(Access::ReadWrite)
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("obj_vram_mapping_mode"))
                    .doc("Chooses one or two dimensionally mapping mode for obj vram")
                    .start(6usize)
                    .size(1usize)
                    .format(Format::Enum{
                        title: Ident::new("ObjVramMappingMode"),
                        variants: vec![
                            VariantBuilder::default()
                                .title("TwoDimensional")
                                .doc("2D Mapping")
                                .build().unwrap(),
                            VariantBuilder::default()
                                .title("OneDimensional")
                                .doc("1D Mapping")
                                .build().unwrap()
                        ]
                    })
                    .access(Access::ReadWrite)
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("forced_blank"))
                    .doc("Allow fast CPU access to all VRAM, making video controller only display white.")
                    .start(7usize)
                    .size(1usize)
                    .format(Format::Bool {
                        quick_set_true: Some(Ident::new("force_blank")),
                        quick_set_false: Some(Ident::new("clear_force_blank")),
                    })
                    .access(Access::ReadWrite)
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("display_layers"))
                    .doc("Layers that should be displayed")
                    .start(8usize)
                    .size(5usize)
                    .access(Access::ReadWrite)
                    .format(Format::Type {
                        type_ident: Ident::new("::video::LayerSet"),
                        as_bits: Some(Rc::new(|ts| {
                            quote! {
                                #ts.bits()
                            }
                        })),
                        from_bits: Some(Rc::new(|ts| {
                            quote! {
                                ::video::LayerSet::from_bits(#ts).unwrap()
                            }
                        })),
                    })
                    .build().unwrap(),
                FieldBuilder::default()
                    .name(Ident::new("windows"))
                    .doc("Windows that should be displayed")
                    .start(13usize)
                    .size(3usize)
                    .access(Access::ReadWrite)
                    .format(Format::Type {
                        type_ident: Ident::new("::video::WindowSet"),
                        as_bits: Some(Rc::new(|ts| {
                            quote! {
                                #ts.bits()
                            }
                        })),
                        from_bits: Some(Rc::new(|ts| {
                            quote! {
                                ::video::WindowSet::from_bits(#ts).unwrap()
                            }
                        })),
                    })
                    .build().unwrap(),
            ])
            .build().unwrap()
    ];
    let register_tokens = registers.iter().map(|t| generate_register(t));
    quote! {
        #(#register_tokens)*
    }
}