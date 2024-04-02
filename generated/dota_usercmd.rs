// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `dota_usercmd.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:dota.CDota2UserCmdPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDota2UserCmdPB {
    // message fields
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.base)
    pub base: ::protobuf::MessageField<super::usercmd::CBaseUserCmdPB>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.spectator_query_unit_entindex)
    pub spectator_query_unit_entindex: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.crosshairtrace)
    pub crosshairtrace: ::protobuf::MessageField<super::networkbasetypes::CMsgVector>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.cameraposition_x)
    pub cameraposition_x: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.cameraposition_y)
    pub cameraposition_y: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.clickbehavior)
    pub clickbehavior: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.statspanel)
    pub statspanel: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.shoppanel)
    pub shoppanel: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.stats_dropdown)
    pub stats_dropdown: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:dota.CDota2UserCmdPB.stats_dropdown_sort)
    pub stats_dropdown_sort: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:dota.CDota2UserCmdPB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDota2UserCmdPB {
    fn default() -> &'a CDota2UserCmdPB {
        <CDota2UserCmdPB as ::protobuf::Message>::default_instance()
    }
}

impl CDota2UserCmdPB {
    pub fn new() -> CDota2UserCmdPB {
        ::std::default::Default::default()
    }

    // optional int32 spectator_query_unit_entindex = 2;

    pub fn spectator_query_unit_entindex(&self) -> i32 {
        self.spectator_query_unit_entindex.unwrap_or(0)
    }

    pub fn clear_spectator_query_unit_entindex(&mut self) {
        self.spectator_query_unit_entindex = ::std::option::Option::None;
    }

    pub fn has_spectator_query_unit_entindex(&self) -> bool {
        self.spectator_query_unit_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spectator_query_unit_entindex(&mut self, v: i32) {
        self.spectator_query_unit_entindex = ::std::option::Option::Some(v);
    }

    // optional int32 cameraposition_x = 4;

    pub fn cameraposition_x(&self) -> i32 {
        self.cameraposition_x.unwrap_or(0)
    }

    pub fn clear_cameraposition_x(&mut self) {
        self.cameraposition_x = ::std::option::Option::None;
    }

    pub fn has_cameraposition_x(&self) -> bool {
        self.cameraposition_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cameraposition_x(&mut self, v: i32) {
        self.cameraposition_x = ::std::option::Option::Some(v);
    }

    // optional int32 cameraposition_y = 5;

    pub fn cameraposition_y(&self) -> i32 {
        self.cameraposition_y.unwrap_or(0)
    }

    pub fn clear_cameraposition_y(&mut self) {
        self.cameraposition_y = ::std::option::Option::None;
    }

    pub fn has_cameraposition_y(&self) -> bool {
        self.cameraposition_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cameraposition_y(&mut self, v: i32) {
        self.cameraposition_y = ::std::option::Option::Some(v);
    }

    // optional uint32 clickbehavior = 6;

    pub fn clickbehavior(&self) -> u32 {
        self.clickbehavior.unwrap_or(0)
    }

    pub fn clear_clickbehavior(&mut self) {
        self.clickbehavior = ::std::option::Option::None;
    }

    pub fn has_clickbehavior(&self) -> bool {
        self.clickbehavior.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clickbehavior(&mut self, v: u32) {
        self.clickbehavior = ::std::option::Option::Some(v);
    }

    // optional uint32 statspanel = 7;

    pub fn statspanel(&self) -> u32 {
        self.statspanel.unwrap_or(0)
    }

    pub fn clear_statspanel(&mut self) {
        self.statspanel = ::std::option::Option::None;
    }

    pub fn has_statspanel(&self) -> bool {
        self.statspanel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_statspanel(&mut self, v: u32) {
        self.statspanel = ::std::option::Option::Some(v);
    }

    // optional uint32 shoppanel = 8;

    pub fn shoppanel(&self) -> u32 {
        self.shoppanel.unwrap_or(0)
    }

    pub fn clear_shoppanel(&mut self) {
        self.shoppanel = ::std::option::Option::None;
    }

    pub fn has_shoppanel(&self) -> bool {
        self.shoppanel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shoppanel(&mut self, v: u32) {
        self.shoppanel = ::std::option::Option::Some(v);
    }

    // optional uint32 stats_dropdown = 9;

    pub fn stats_dropdown(&self) -> u32 {
        self.stats_dropdown.unwrap_or(0)
    }

    pub fn clear_stats_dropdown(&mut self) {
        self.stats_dropdown = ::std::option::Option::None;
    }

    pub fn has_stats_dropdown(&self) -> bool {
        self.stats_dropdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats_dropdown(&mut self, v: u32) {
        self.stats_dropdown = ::std::option::Option::Some(v);
    }

    // optional uint32 stats_dropdown_sort = 10;

    pub fn stats_dropdown_sort(&self) -> u32 {
        self.stats_dropdown_sort.unwrap_or(0)
    }

    pub fn clear_stats_dropdown_sort(&mut self) {
        self.stats_dropdown_sort = ::std::option::Option::None;
    }

    pub fn has_stats_dropdown_sort(&self) -> bool {
        self.stats_dropdown_sort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats_dropdown_sort(&mut self, v: u32) {
        self.stats_dropdown_sort = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::usercmd::CBaseUserCmdPB>(
            "base",
            |m: &CDota2UserCmdPB| { &m.base },
            |m: &mut CDota2UserCmdPB| { &mut m.base },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "spectator_query_unit_entindex",
            |m: &CDota2UserCmdPB| { &m.spectator_query_unit_entindex },
            |m: &mut CDota2UserCmdPB| { &mut m.spectator_query_unit_entindex },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::networkbasetypes::CMsgVector>(
            "crosshairtrace",
            |m: &CDota2UserCmdPB| { &m.crosshairtrace },
            |m: &mut CDota2UserCmdPB| { &mut m.crosshairtrace },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "cameraposition_x",
            |m: &CDota2UserCmdPB| { &m.cameraposition_x },
            |m: &mut CDota2UserCmdPB| { &mut m.cameraposition_x },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "cameraposition_y",
            |m: &CDota2UserCmdPB| { &m.cameraposition_y },
            |m: &mut CDota2UserCmdPB| { &mut m.cameraposition_y },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "clickbehavior",
            |m: &CDota2UserCmdPB| { &m.clickbehavior },
            |m: &mut CDota2UserCmdPB| { &mut m.clickbehavior },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "statspanel",
            |m: &CDota2UserCmdPB| { &m.statspanel },
            |m: &mut CDota2UserCmdPB| { &mut m.statspanel },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "shoppanel",
            |m: &CDota2UserCmdPB| { &m.shoppanel },
            |m: &mut CDota2UserCmdPB| { &mut m.shoppanel },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "stats_dropdown",
            |m: &CDota2UserCmdPB| { &m.stats_dropdown },
            |m: &mut CDota2UserCmdPB| { &mut m.stats_dropdown },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "stats_dropdown_sort",
            |m: &CDota2UserCmdPB| { &m.stats_dropdown_sort },
            |m: &mut CDota2UserCmdPB| { &mut m.stats_dropdown_sort },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDota2UserCmdPB>(
            "CDota2UserCmdPB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDota2UserCmdPB {
    const NAME: &'static str = "CDota2UserCmdPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.base)?;
                },
                16 => {
                    self.spectator_query_unit_entindex = ::std::option::Option::Some(is.read_int32()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.crosshairtrace)?;
                },
                32 => {
                    self.cameraposition_x = ::std::option::Option::Some(is.read_int32()?);
                },
                40 => {
                    self.cameraposition_y = ::std::option::Option::Some(is.read_int32()?);
                },
                48 => {
                    self.clickbehavior = ::std::option::Option::Some(is.read_uint32()?);
                },
                56 => {
                    self.statspanel = ::std::option::Option::Some(is.read_uint32()?);
                },
                64 => {
                    self.shoppanel = ::std::option::Option::Some(is.read_uint32()?);
                },
                72 => {
                    self.stats_dropdown = ::std::option::Option::Some(is.read_uint32()?);
                },
                80 => {
                    self.stats_dropdown_sort = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.spectator_query_unit_entindex {
            my_size += ::protobuf::rt::int32_size(2, v);
        }
        if let Some(v) = self.crosshairtrace.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.cameraposition_x {
            my_size += ::protobuf::rt::int32_size(4, v);
        }
        if let Some(v) = self.cameraposition_y {
            my_size += ::protobuf::rt::int32_size(5, v);
        }
        if let Some(v) = self.clickbehavior {
            my_size += ::protobuf::rt::uint32_size(6, v);
        }
        if let Some(v) = self.statspanel {
            my_size += ::protobuf::rt::uint32_size(7, v);
        }
        if let Some(v) = self.shoppanel {
            my_size += ::protobuf::rt::uint32_size(8, v);
        }
        if let Some(v) = self.stats_dropdown {
            my_size += ::protobuf::rt::uint32_size(9, v);
        }
        if let Some(v) = self.stats_dropdown_sort {
            my_size += ::protobuf::rt::uint32_size(10, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.base.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.spectator_query_unit_entindex {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.crosshairtrace.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.cameraposition_x {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.cameraposition_y {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.clickbehavior {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.statspanel {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.shoppanel {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.stats_dropdown {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.stats_dropdown_sort {
            os.write_uint32(10, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CDota2UserCmdPB {
        CDota2UserCmdPB::new()
    }

    fn clear(&mut self) {
        self.base.clear();
        self.spectator_query_unit_entindex = ::std::option::Option::None;
        self.crosshairtrace.clear();
        self.cameraposition_x = ::std::option::Option::None;
        self.cameraposition_y = ::std::option::Option::None;
        self.clickbehavior = ::std::option::Option::None;
        self.statspanel = ::std::option::Option::None;
        self.shoppanel = ::std::option::Option::None;
        self.stats_dropdown = ::std::option::Option::None;
        self.stats_dropdown_sort = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDota2UserCmdPB {
        static instance: CDota2UserCmdPB = CDota2UserCmdPB {
            base: ::protobuf::MessageField::none(),
            spectator_query_unit_entindex: ::std::option::Option::None,
            crosshairtrace: ::protobuf::MessageField::none(),
            cameraposition_x: ::std::option::Option::None,
            cameraposition_y: ::std::option::Option::None,
            clickbehavior: ::std::option::Option::None,
            statspanel: ::std::option::Option::None,
            shoppanel: ::std::option::Option::None,
            stats_dropdown: ::std::option::Option::None,
            stats_dropdown_sort: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDota2UserCmdPB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDota2UserCmdPB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDota2UserCmdPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDota2UserCmdPB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12dota_usercmd.proto\x12\x04dota\x1a\x16networkbasetypes.proto\x1a\r\
    usercmd.proto\"\xc9\x03\n\x0fCDota2UserCmdPB\x12(\n\x04base\x18\x01\x20\
    \x01(\x0b2\x14.dota.CBaseUserCmdPBR\x04base\x12A\n\x1dspectator_query_un\
    it_entindex\x18\x02\x20\x01(\x05R\x1aspectatorQueryUnitEntindex\x128\n\
    \x0ecrosshairtrace\x18\x03\x20\x01(\x0b2\x10.dota.CMsgVectorR\x0ecrossha\
    irtrace\x12)\n\x10cameraposition_x\x18\x04\x20\x01(\x05R\x0fcamerapositi\
    onX\x12)\n\x10cameraposition_y\x18\x05\x20\x01(\x05R\x0fcamerapositionY\
    \x12$\n\rclickbehavior\x18\x06\x20\x01(\rR\rclickbehavior\x12\x1e\n\nsta\
    tspanel\x18\x07\x20\x01(\rR\nstatspanel\x12\x1c\n\tshoppanel\x18\x08\x20\
    \x01(\rR\tshoppanel\x12%\n\x0estats_dropdown\x18\t\x20\x01(\rR\rstatsDro\
    pdown\x12.\n\x13stats_dropdown_sort\x18\n\x20\x01(\rR\x11statsDropdownSo\
    rtB%Z#github.com/dotabuff/manta/dota;dotaJ\x91\x06\n\x06\x12\x04\0\0\x13\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0c\
    \n\x08\n\x01\x08\x12\x03\x03\0:\n\t\n\x02\x08\x0b\x12\x03\x03\0:\n\t\n\
    \x02\x03\0\x12\x03\x05\x07\x1f\n\t\n\x02\x03\x01\x12\x03\x06\x07\x16\n\n\
    \n\x02\x04\0\x12\x04\x08\0\x13\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\
    \x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x08)\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\t\x08\x10\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\t\x11\x1f\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\t\x20$\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \t'(\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x089\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\n\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x11\x16\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\x174\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\n78\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x08/\n\x0c\n\x05\
    \x04\0\x02\x02\x04\x12\x03\x0b\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03\x0b\x11\x1b\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\x1c*\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x0b-.\n\x0b\n\x04\x04\0\x02\x03\x12\x03\
    \x0c\x08,\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x0c\x08\x10\n\x0c\n\x05\
    \x04\0\x02\x03\x05\x12\x03\x0c\x11\x16\n\x0c\n\x05\x04\0\x02\x03\x01\x12\
    \x03\x0c\x17'\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c*+\n\x0b\n\x04\
    \x04\0\x02\x04\x12\x03\r\x08,\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\r\
    \x08\x10\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\r\x11\x16\n\x0c\n\x05\x04\
    \0\x02\x04\x01\x12\x03\r\x17'\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\r*+\
    \n\x0b\n\x04\x04\0\x02\x05\x12\x03\x0e\x08*\n\x0c\n\x05\x04\0\x02\x05\
    \x04\x12\x03\x0e\x08\x10\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x0e\x11\
    \x17\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0e\x18%\n\x0c\n\x05\x04\0\
    \x02\x05\x03\x12\x03\x0e()\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0f\x08'\n\
    \x0c\n\x05\x04\0\x02\x06\x04\x12\x03\x0f\x08\x10\n\x0c\n\x05\x04\0\x02\
    \x06\x05\x12\x03\x0f\x11\x17\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x0f\
    \x18\"\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0f%&\n\x0b\n\x04\x04\0\x02\
    \x07\x12\x03\x10\x08&\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\x10\x08\x10\
    \n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x10\x11\x17\n\x0c\n\x05\x04\0\x02\
    \x07\x01\x12\x03\x10\x18!\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x10$%\n\
    \x0b\n\x04\x04\0\x02\x08\x12\x03\x11\x08+\n\x0c\n\x05\x04\0\x02\x08\x04\
    \x12\x03\x11\x08\x10\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\x11\x11\x17\n\
    \x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x11\x18&\n\x0c\n\x05\x04\0\x02\x08\
    \x03\x12\x03\x11)*\n\x0b\n\x04\x04\0\x02\t\x12\x03\x12\x081\n\x0c\n\x05\
    \x04\0\x02\t\x04\x12\x03\x12\x08\x10\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\
    \x12\x11\x17\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\x12\x18+\n\x0c\n\x05\
    \x04\0\x02\t\x03\x12\x03\x12.0\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::networkbasetypes::file_descriptor().clone());
            deps.push(super::usercmd::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CDota2UserCmdPB::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
