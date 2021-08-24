#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum DXGI_Encoding {
    DXGI_FORMAT_BC1_UNORM =  8, // DXT1
    DXGI_FORMAT_BC3_UNORM = 24, // DXT5
    DXGI_FORMAT_BC5_UNORM = 32, // ATI2
}

impl Default for DXGI_Encoding {
    fn default() -> DXGI_Encoding {
        DXGI_Encoding::DXGI_FORMAT_BC3_UNORM
    }
}

impl From<u32> for DXGI_Encoding {
    fn from(num: u32) -> DXGI_Encoding {
        match num {
             8 => DXGI_Encoding::DXGI_FORMAT_BC1_UNORM,
            24 => DXGI_Encoding::DXGI_FORMAT_BC3_UNORM,
            32 => DXGI_Encoding::DXGI_FORMAT_BC5_UNORM,
            // Default
            _ => DXGI_Encoding::DXGI_FORMAT_BC3_UNORM,
        }
    }
}