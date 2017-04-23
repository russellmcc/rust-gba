use ::register::*;
use ::errors::*;
use std::rc::Rc;
use quote::Tokens;
use syn::Ident;

fn generate_docs(doc: &str) -> Result<Tokens> {
    let docs: Vec<Tokens> = doc.lines().map(|line| {
        quote! {
            #[doc=#line]
        }
    }).collect();
    Ok(quote!{ #(#docs)* })
}

fn generate_enum_variant_branch(variant: &Variant) -> Result<Tokens> {
    let title = &variant.title;
    let docs = generate_docs(&variant.doc)?;
    match variant.value {
        None => Ok(quote! {
            #docs #title
        }),
        Some(v) => {
            let v_hacked = Ident::new(v.to_string());
            Ok(quote! {
                #docs #title = #v_hacked
            })
        }
    }
}

fn generate_enum(title: &Ident,
                 repr: &Ident,
                 variants: &[Variant]) ->
    Result<Tokens> {
        if variants.is_empty() {
            bail!("Must have at least one variant in each enum")
        }
        let variants =
            variants.iter().map(generate_enum_variant_branch)
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            #[repr(#repr)]
            pub enum #title {
                #(#variants),*
            }
        })
}

pub fn generate_register_struct(register: &Register) -> Result<Tokens> {
    let title = &register.title;
    let repr = &register.repr;
    let docs = generate_docs(&register.doc)?;

    Ok(quote! {
        #docs
        pub struct #title {
            value: VolatileCell<#repr>,
        }
    })
}

#[derive(Default)]
struct RegisterMode {
    // The mode struct definition and struct impl
    tokens: Tokens,

    // The function in the main struct impl that
    // produces/consumes the struct.
    main_impl_fn: Tokens,
}

struct FieldInfo<'a> {
    field: &'a Field,
    mask: usize,
    type_ident: Ident,
    as_bits: Rc<Fn(Tokens) -> Tokens>,
    from_bits: Rc<Fn(Tokens) -> Tokens>,
}

fn get_field_info(field: &Field, repr: Ident) -> FieldInfo {
    let mask = (field.start..(field.start + field.size))
        .fold(0usize, |m, i| m | 1 << i);
    let type_ident = match field.format {
        Format::Type{ref type_ident, ..} => type_ident.clone(),
        Format::Bool{..} => Ident::from("bool"),
        Format::Enum{ref title, ..} => title.clone(),
    };
    let from_bits = match field.format {
        Format::Type{ref from_bits, ..} =>
            from_bits.as_ref().map_or_else(|| -> Rc<Fn(Tokens) -> Tokens> {
                let type_ident_for_convert = type_ident.clone();
                Rc::new(move |ts| {
                    quote!{
                        #ts as #type_ident_for_convert
                    }
                })}, |c| c.clone()),
        Format::Bool{..} => Rc::new(|ts| {
            quote!{
                (#ts) != 0
            }
        }),
        Format::Enum{..} => Rc::new(|ts| {
            quote!{
                unsafe { mem::transmute(#ts) }
            }
        }),
    };
    let as_bits = match field.format {
        Format::Type{ref as_bits, ..} =>
            as_bits.as_ref().map_or_else(|| -> Rc<Fn(Tokens) -> Tokens> {
                let repr_clone = repr.clone();
                Rc::new(move |ts| {
                    quote!{
                        #ts as #repr_clone
                    }
                })}, |c| c.clone()),
        Format::Bool{..} => Rc::new(move |ts| {
            quote!{
                if #ts {1} else {0}
            }
        }),
        Format::Enum{..} => {
            let repr_clone = repr.clone();
            Rc::new(move |ts| {
                quote!{
                    #ts as #repr_clone
                }
            })
        },
    };
    FieldInfo { field, mask, type_ident, from_bits, as_bits }
}

fn generate_register_write(register: &Register) -> Result<RegisterMode> {
    if register
        .fields
        .iter()
        .all(|f| f.access == Access::ReadOnly) {
            return Ok(RegisterMode::default());
        }

    let repr = &register.repr;

    let title = Ident::from(format!("{}Write", register.title.to_string()));
    let writable_fields = || {
        register.fields.iter()
        .filter(|f| f.access != Access::ReadOnly)
        .map(|f| get_field_info(f, register.repr.clone()))
    };

    let field_setters = writable_fields()
        .map(|FieldInfo {
            field: &Field { ref name, ref doc, ref start, .. },
            ref mask,
            ref type_ident,
            ref as_bits,
            ..
        }| -> Result<Tokens> {
            let doc_tokens = generate_docs(doc)?;
            let mask_hacked = Ident::new(mask.to_string());
            let shift_hacked = Ident::new(start.to_string());
            let bits = as_bits(quote!{tt});
            let setter_name = Ident::new(format!("set_{}", name));
            Ok(quote!{
                #doc_tokens
                #[inline]
                pub fn #setter_name (&mut self, tt: #type_ident) -> &mut #title {
                    self.value &= !#mask_hacked;
                    self.value |= ((#bits) << #shift_hacked) & #mask_hacked;
                    self
                }
            })
        }).collect::<Result<Vec<_>>>()?;

    let fast_enum_setters = writable_fields()
        .filter_map(|FieldInfo {
            field: &Field { ref start, ref format, .. },
            ref mask,
            ..
        }| {
            match *format {
                Format::Enum {
                    title: ref etitle,
                    ref variants
                } => {
                    let mask_hacked = Ident::new(mask.to_string());
                    let shift_hacked = Ident::new(start.to_string());
                    let title_ref = &title;
                    Some(variants.iter()
                         .filter_map(|v| {
                             v.quick_set.as_ref().map(|qs| (qs, &v.title))
                         })
                         .map(move |(quick_set, vtitle)| {
                             Ok(quote!(
                                 #[inline]
                                 pub fn #quick_set (&mut self) -> &mut #title_ref {
                                     self.value &= !#mask_hacked;
                                     self.value |= ((#etitle :: #vtitle as #repr) << #shift_hacked) & #mask_hacked;
                                     self
                                 }
                             ))
                         }))
                },
                _ => None
            }
        })
        .flat_map(|ts| ts)
        .collect::<Result<Vec<_>>>()?;

    let writable_bool_fields = || writable_fields()
        .filter_map(|fi| {
            match fi.field.format {
                Format::Bool {
                    quick_set_true: ref qst,
                    quick_set_false: ref qsf,
                } => Some(((qst, qsf), fi)),
                _ => None,
            }
        });

    let fast_bool_set_trues = writable_bool_fields()
        .filter_map(|((qst, _), fi)| {
            qst.as_ref().map(|qst| {
                let mask_hacked = Ident::new(fi.mask.to_string());
                let title_ref = &title;
                quote! {
                    #[inline]
                    pub fn #qst (&mut self) -> &mut #title_ref {
                        self.value |= #mask_hacked;
                        self
                    }
                }
            })
        });


    let fast_bool_set_falses = writable_bool_fields()
        .filter_map(|((_, qsf), fi)| {
            qsf.as_ref().map(|qsf| {
                let mask_hacked = Ident::new(fi.mask.to_string());
                let title_ref = &title;
                quote! {
                    #[inline]
                    pub fn #qsf (&mut self) -> &mut #title_ref {
                        self.value &= !#mask_hacked;
                        self
                    }
                }
            })
        });

    let rtitle = &register.title;
    let rreset = Ident::new(register.reset_value.to_string());

    let read_title = Ident::from(format!("{}Read", register.title.to_string()));
    let modify_fn_impl = if !register
        .fields
        .iter()
        .all(|f| f.access == Access::WriteOnly) {
            quote! {
            #[doc = "Combines a read and an update."]
            #[inline]
                pub fn modify
                    <F: FnOnce(#read_title, #title) -> #title>(
                        &mut self,
                        f: F) -> &mut #rtitle {
                        let t = self.value.get();
                        self.value.set(f(#read_title {value: t}, #title {value: t}).value);
                        self
                    }
            }
        } else {
            quote!{}
        };

    Ok(RegisterMode {
        tokens: quote!{
            pub struct #title {
                value: #repr
            }

            impl #title {
                #(#field_setters)*

                #(#fast_enum_setters)*

                #(#fast_bool_set_trues)*

                #(#fast_bool_set_falses)*
            }

            impl Default for #title {
                fn default() -> #title {
                    #title { value: #rreset }
                }
            }
        },
        main_impl_fn: quote!{
            #[doc = "Writes a new value to the register."]
            #[inline]
            pub fn write(&mut self, write: &#title) -> &mut #rtitle {
                self.value.set(write.value);
                self
            }

            #[doc = "Updates only certain fields of the register,"]
            #[doc = "keeping other fields unchanged."]
            #[inline]
            pub fn update<F: FnOnce(#title) -> #title>(&mut self, f: F) -> &mut #rtitle {
                let t = self.value.get();
                self.value.set(f(#title {value: t}).value);
                self
            }

            #modify_fn_impl
        }
    })
}

fn generate_register_read(register: &Register) -> Result<RegisterMode> {
    if register
        .fields
        .iter()
        .all(|f| f.access == Access::WriteOnly) {
            return Ok(RegisterMode::default());
        }

    let readable_fields = || register.fields.iter()
        .filter(|f| f.access != Access::WriteOnly)
        .map(|f| get_field_info(f, register.repr.clone()));

    let field_accessors = readable_fields()
        .map(|FieldInfo {
            field: &Field { ref name, ref doc, ref start, .. },
            ref mask,
            ref type_ident,
            ref from_bits,
            ..
        }| -> Result<Tokens> {
            let doc_tokens = generate_docs(doc)?;
            let mask_hacked = Ident::new(mask.to_string());
            let shift_hacked = Ident::new(start.to_string());
            let body = from_bits(quote!{((self.value & #mask_hacked) >> #shift_hacked)});
            Ok(quote!{
                #doc_tokens
                #[inline]
                pub fn #name (&self) -> #type_ident {
                    #body
                }
            })
        }).collect::<Result<Vec<_>>>()?;
    let repr = &register.repr;
    let fast_enum_accessors = readable_fields()
        .filter_map(|FieldInfo {
            field: &Field { ref start, ref format, .. },
            ref mask,
            ..
        }| {
            match *format {
                Format::Enum {
                    title: ref etitle,
                    ref variants
                } => {
                    let mask_hacked = Ident::new(mask.to_string());
                    let shift_hacked = Ident::new(start.to_string());
                    Some(variants.iter()
                         .filter_map(|v| {
                             v.quick_get.as_ref().map(|qg| (qg, &v.title))
                         })
                         .map(move |(quick_get, title)| {
                             Ok(quote!(
                                 #[inline]
                                 pub fn #quick_get (&self) -> bool {
                                     return (#etitle :: #title as #repr) == ((self.value & #mask_hacked) >> #shift_hacked)
                                 }
                             ))
                         }))
                },
                _ => None
            }
        })
        .flat_map(|ts| ts)
        .collect::<Result<Vec<_>>>()?;

    let title = Ident::from(format!("{}Read", register.title.to_string()));

    Ok(RegisterMode {
        tokens: quote!{
            pub struct #title {
                value: #repr
            }

            impl #title {
                #(#field_accessors)*
                #(#fast_enum_accessors)*
            }
        },
        main_impl_fn: quote!{
            #[inline]
            pub fn read(&self) -> #title {
                #title {value: self.value.get()}
            }
        }
    })
}

pub fn generate_register(register: &Register) -> Result<Tokens> {
    if register.fields.is_empty() {
        bail!("Must have at least one field")
    }

    // first, generate all enums.
    let enum_definitions : Vec<Tokens> =
        register
        .fields
        .iter()
        .filter_map(|x| match x.format {
            Format::Enum {
                ref title,
                ref variants
            } => Some(generate_enum(title,
                                    &register.repr,
                                    variants)),
            _ => None
        }).collect::<Result<Vec<_>>>()?;

    let register_struct = generate_register_struct(register)?;
    let title = &register.title;
    let RegisterMode {
        tokens: read_tokens,
        main_impl_fn: read_impl_fn,
    } = generate_register_read(register)?;
    let RegisterMode {
        tokens: write_tokens,
        main_impl_fn: write_impl_fn,
    } = generate_register_write(register)?;

    Ok(quote! {
        use core::mem;
        use vcell::VolatileCell;

        #(#enum_definitions)*

        #register_struct

        #read_tokens

        #write_tokens

        impl #title {
            #read_impl_fn
            #write_impl_fn
        }
    })
}