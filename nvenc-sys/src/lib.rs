#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/nvenc.rs"));
use std::{cell::OnceCell, sync::{Mutex, OnceLock}};

use libloading::Library;
use thiserror::Error;

macro_rules! guid {
    {$d1:expr,$d2:expr,$d3:expr,{$d4:expr,$d5:expr,$d6:expr,$d7:expr,$d8:expr,$d9:expr,$d10:expr,$d11:expr}} => {
        _GUID {
            Data1: $d1,
            Data2: $d2,
            Data3: $d3,
            Data4: [$d4,$d5,$d6,$d7,$d8,$d9,$d10,$d11],
        }
    };
}

pub const NV_ENC_CODEC_H264_GUID: GUID =
    guid! { 0x6bc82762, 0x4e63, 0x4ca4, { 0xaa, 0x85, 0x1e, 0x50, 0xf3, 0x21, 0xf6, 0xbf } };
pub const NV_ENC_CODEC_HEVC_GUID: GUID =
    guid! { 0x790cdc88, 0x4522, 0x4d7b, { 0x94, 0x25, 0xbd, 0xa9, 0x97, 0x5f, 0x76, 0x3 } };

pub const NV_ENC_PRESET_P1_GUID: GUID =
    guid! { 0xfc0a8d3e, 0x45f8, 0x4cf8, { 0x80, 0xc7, 0x29, 0x88, 0x71, 0x59, 0xe, 0xbf } };

// {F581CFB8-88D6-4381-93F0-DF13F9C27DAB}
pub const NV_ENC_PRESET_P2_GUID: GUID =
    guid! { 0xf581cfb8, 0x88d6, 0x4381, { 0x93, 0xf0, 0xdf, 0x13, 0xf9, 0xc2, 0x7d, 0xab } };

// {36850110-3A07-441F-94D5-3670631F91F6}
pub const NV_ENC_PRESET_P3_GUID: GUID =
    guid! { 0x36850110, 0x3a07, 0x441f, { 0x94, 0xd5, 0x36, 0x70, 0x63, 0x1f, 0x91, 0xf6 } };

// {90A7B826-DF06-4862-B9D2-CD6D73A08681}
pub const NV_ENC_PRESET_P4_GUID: GUID =
    guid! { 0x90a7b826, 0xdf06, 0x4862, { 0xb9, 0xd2, 0xcd, 0x6d, 0x73, 0xa0, 0x86, 0x81 } };

// {21C6E6B4-297A-4CBA-998F-B6CBDE72ADE3}
pub const NV_ENC_PRESET_P5_GUID: GUID =
    guid! { 0x21c6e6b4, 0x297a, 0x4cba, { 0x99, 0x8f, 0xb6, 0xcb, 0xde, 0x72, 0xad, 0xe3 } };

// {8E75C279-6299-4AB6-8302-0B215A335CF5}
pub const NV_ENC_PRESET_P6_GUID: GUID =
    guid! { 0x8e75c279, 0x6299, 0x4ab6, { 0x83, 0x2, 0xb, 0x21, 0x5a, 0x33, 0x5c, 0xf5 } };

// {84848C12-6F71-4C13-931B-53E283F57974}
pub const NV_ENC_PRESET_P7_GUID: GUID =
    guid! { 0x84848c12, 0x6f71, 0x4c13, { 0x93, 0x1b, 0x53, 0xe2, 0x83, 0xf5, 0x79, 0x74 } };

// {BFD6F8E7-233C-4341-8B3E-4818523803F4}
pub const NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID: GUID =
    guid! { 0xbfd6f8e7, 0x233c, 0x4341, { 0x8b, 0x3e, 0x48, 0x18, 0x52, 0x38, 0x3, 0xf4 } };

// {0727BCAA-78C4-4c83-8C2F-EF3DFF267C6A}
pub const NV_ENC_H264_PROFILE_BASELINE_GUID: GUID =
    guid! { 0x727bcaa, 0x78c4, 0x4c83, { 0x8c, 0x2f, 0xef, 0x3d, 0xff, 0x26, 0x7c, 0x6a } };

// {60B5C1D4-67FE-4790-94D5-C4726D7B6E6D}
pub const NV_ENC_H264_PROFILE_MAIN_GUID: GUID =
    guid! { 0x60b5c1d4, 0x67fe, 0x4790, { 0x94, 0xd5, 0xc4, 0x72, 0x6d, 0x7b, 0x6e, 0x6d } };

// {E7CBC309-4F7A-4b89-AF2A-D537C92BE310}
pub const NV_ENC_H264_PROFILE_HIGH_GUID: GUID =
    guid! { 0xe7cbc309, 0x4f7a, 0x4b89, { 0xaf, 0x2a, 0xd5, 0x37, 0xc9, 0x2b, 0xe3, 0x10 } };

// {7AC663CB-A598-4960-B844-339B261A7D52}
pub const NV_ENC_H264_PROFILE_HIGH_444_GUID: GUID =
    guid! { 0x7ac663cb, 0xa598, 0x4960, { 0xb8, 0x44, 0x33, 0x9b, 0x26, 0x1a, 0x7d, 0x52 } };

// {40847BF5-33F7-4601-9084-E8FE3C1DB8B7}
pub const NV_ENC_H264_PROFILE_STEREO_GUID: GUID =
    guid! { 0x40847bf5, 0x33f7, 0x4601, { 0x90, 0x84, 0xe8, 0xfe, 0x3c, 0x1d, 0xb8, 0xb7 } };

// {B405AFAC-F32B-417B-89C4-9ABEED3E5978}
pub const NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID: GUID =
    guid! { 0xb405afac, 0xf32b, 0x417b, { 0x89, 0xc4, 0x9a, 0xbe, 0xed, 0x3e, 0x59, 0x78 } };

// {7AC663CB-A598-4960-B844-339B261A7D52}
pub const NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_444_GUID: GUID =
    guid! { 0x7ac663cb, 0xa598, 0x4960, { 0xb8, 0x44, 0x33, 0x9b, 0x26, 0x1a, 0x7d, 0x52 } };

// {AEC1BD87-E85B-48f2-84C3-98BCA6285072}
pub const NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID: GUID =
    guid! { 0xaec1bd87, 0xe85b, 0x48f2, { 0x84, 0xc3, 0x98, 0xbc, 0xa6, 0x28, 0x50, 0x72 } };

// {B514C39A-B55B-40fa-878F-F1253B4DFDEC}
pub const NV_ENC_HEVC_PROFILE_MAIN_GUID: GUID =
    guid! { 0xb514c39a, 0xb55b, 0x40fa, { 0x87, 0x8f, 0xf1, 0x25, 0x3b, 0x4d, 0xfd, 0xec } };

// {fa4d2b6c-3a5b-411a-8018-0a3f5e3c9be5}
pub const NV_ENC_HEVC_PROFILE_MAIN10_GUID: GUID =
    guid! { 0xfa4d2b6c, 0x3a5b, 0x411a, { 0x80, 0x18, 0x0a, 0x3f, 0x5e, 0x3c, 0x9b, 0xe5 } };

// For HEVC Main 444 8 bit and HEVC Main 444 10 bit profiles only
// {51ec32b5-1b4c-453c-9cbd-b616bd621341}
pub const NV_ENC_HEVC_PROFILE_FREXT_GUID: GUID =
    guid! { 0x51ec32b5, 0x1b4c, 0x453c, { 0x9c, 0xbd, 0xb6, 0x16, 0xbd, 0x62, 0x13, 0x41 } };

#[derive(Error, Debug, Clone)]
pub enum NvencError {
    #[error("library failed to load: {0}")]
    LibraryLoadFailure(String),
    #[error("invalid parameter")]
    InvalidParam,
    #[error("unsupported parameter")]
    UnsupportedParam,
    #[error("unknown error: {0}")]
    Unknown(i32),
}

fn check_error(status: NVENCSTATUS) -> Result<(), NvencError> {
    if status == _NVENCSTATUS_NV_ENC_SUCCESS {
        return Ok(());
    }

    match status {
        _NVENCSTATUS_NV_ENC_SUCCESS => Ok(()),
        _NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM => Err(NvencError::InvalidParam),
        _NVENCSTATUS_NV_ENC_ERR_UNSUPPORTED_PARAM => Err(NvencError::UnsupportedParam),
        _ => Err(NvencError::Unknown(status)),
    }
}

pub const NVENC_DLL_NAME: &str = "nvEncodeAPI64.dll";
pub const NV_ENCODE_API_CREATE_INSTANCE_SYMBOL: &[u8] = b"NvEncodeAPICreateInstance\0";
pub type NvEncodeApiCreateInstanceFn =
    unsafe extern "C" fn(functionList: *mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS;

static INSTANCE: OnceLock<Result<Library, NvencError>> = OnceLock::new();

pub fn create_api_instance() -> Result<NV_ENCODE_API_FUNCTION_LIST, NvencError> {
    
    let nvenc = INSTANCE.get_or_init(|| {
        unsafe { libloading::Library::new(NVENC_DLL_NAME).map_err(|e| NvencError::LibraryLoadFailure(e.to_string())) }
    });

    if let Err(e) = nvenc {
        return Err(e.to_owned())
    }

    let nvenc = nvenc.as_ref().unwrap();

    let nv_encode_api_create_instance_fn = unsafe {
        nvenc
            .get::<NvEncodeApiCreateInstanceFn>(NV_ENCODE_API_CREATE_INSTANCE_SYMBOL)
            .expect("failed to load nvenc init function")
    };

    let mut function_list: NV_ENCODE_API_FUNCTION_LIST = unsafe { std::mem::zeroed() };
    function_list.version = NV_ENCODE_API_FUNCTION_LIST_VER;

    check_error(unsafe { nv_encode_api_create_instance_fn(&mut function_list) })?;

    println!("Function: {:#?}", function_list.nvEncOpenEncodeSessionEx.unwrap());

    Ok(function_list)
}
