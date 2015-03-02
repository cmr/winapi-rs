/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_tWAVEFORMATEX {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
    pub wBitsPerSample: WORD,
    pub cbSize: WORD,
}
impl ::std::default::Default for Struct_tWAVEFORMATEX {
    fn default() -> Struct_tWAVEFORMATEX { unsafe { ::std::mem::zeroed() } }
}
pub type WAVEFORMATEX = Struct_tWAVEFORMATEX;
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type PCWAVEFORMATEX = *const WAVEFORMATEX;
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub Format: WAVEFORMATEX,
    pub Samples: Union_Unnamed2,
    pub dwChannelMask: DWORD,
    pub SubFormat: GUID,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed2 {
    pub _bindgen_data_: [u8; 2usize],
}
impl Union_Unnamed2 {
    pub unsafe fn wValidBitsPerSample(&mut self) -> *mut WORD {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn wSamplesPerBlock(&mut self) -> *mut WORD {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn wReserved(&mut self) -> *mut WORD {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed2 {
    fn default() -> Union_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
pub type WAVEFORMATEXTENSIBLE = Struct_Unnamed1;
pub type PWAVEFORMATEXTENSIBLE = *mut WAVEFORMATEXTENSIBLE;
pub type LPWAVEFORMATEXTENSIBLE = *mut WAVEFORMATEXTENSIBLE;
pub type PCWAVEFORMATEXTENSIBLE = *const WAVEFORMATEXTENSIBLE;
pub type LPCWAVEFORMATEXTENSIBLE = *const WAVEFORMATEXTENSIBLE;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_waveformat_tag {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
}
impl ::std::default::Default for Struct_waveformat_tag {
    fn default() -> Struct_waveformat_tag { unsafe { ::std::mem::zeroed() } }
}
pub type WAVEFORMAT = Struct_waveformat_tag;
pub type PWAVEFORMAT = *mut Struct_waveformat_tag;
pub type NPWAVEFORMAT = *mut Struct_waveformat_tag;
pub type LPWAVEFORMAT = *mut Struct_waveformat_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_pcmwaveformat_tag {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: WORD,
}
impl ::std::default::Default for Struct_pcmwaveformat_tag {
    fn default() -> Struct_pcmwaveformat_tag {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PCMWAVEFORMAT = Struct_pcmwaveformat_tag;
pub type PPCMWAVEFORMAT = *mut Struct_pcmwaveformat_tag;
pub type NPPCMWAVEFORMAT = *mut Struct_pcmwaveformat_tag;
pub type LPPCMWAVEFORMAT = *mut Struct_pcmwaveformat_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_adpcmcoef_tag {
    pub iCoef1: ::libc::c_short,
    pub iCoef2: ::libc::c_short,
}
impl ::std::default::Default for Struct_adpcmcoef_tag {
    fn default() -> Struct_adpcmcoef_tag { unsafe { ::std::mem::zeroed() } }
}
pub type ADPCMCOEFSET = Struct_adpcmcoef_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_adpcmwaveformat_tag {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: WORD,
    pub wNumCoef: WORD,
    pub aCoef: *mut ADPCMCOEFSET,
}
impl ::std::default::Default for Struct_adpcmwaveformat_tag {
    fn default() -> Struct_adpcmwaveformat_tag {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ADPCMWAVEFORMAT = Struct_adpcmwaveformat_tag;
