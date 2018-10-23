#[cfg(windows)] extern crate winapi;

//dont know if i like it

#[allow(unused_imports)]
use std::path::PathBuf;

type DWORD = u32;
#[allow(dead_code)]
type WORD = u16;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[repr(C)]
struct VS_FIXEDFILEINFO {
    dwSignature: DWORD,
    dwStrucVersion: DWORD,
    dwFileVersionMS: DWORD,
    dwFileVersionLS: DWORD,
    dwProductVersionMS: DWORD,
    dwProductVersionLS: DWORD,
    dwFileFlagsMask: DWORD,
    dwFileFlags: DWORD,
    dwFileOS: DWORD,
    dwFileType: DWORD,
    dwFileSubtype: DWORD,
    dwFileDateMS: DWORD,
    dwFileDateLS: DWORD,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[inline]
fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[inline]
fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}


#[cfg(not(windows))]
#[allow(unused_variables)]
pub fn get_file_version(file: PathBuf) -> (u16,u16,u16,u16) {
     (0,0,0,0)
}

#[cfg(windows)]
pub fn get_file_version(file: PathBuf) -> (u16,u16,u16,u16) {
    
    use std::ffi::{OsStr,OsString};
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::ffi::OsStringExt;
    use std::iter::once;
    use std::ptr::null_mut;
    use std::mem;

    use self::winapi::*;
    use self::winapi::um::winver::{GetFileVersionInfoW, GetFileVersionInfoSizeW, VerQueryValueW};


    let wide: Vec<_> = OsStr::new(&file.display()).encode_wide().chain(once(0)).collect();

    let ret_size = unsafe {
        GetFileVersionInfoSizeW(wide.as_ptr(), null_mut())
    };

    if ret_size == 0 {
        return (0,0,0,0);
    }

    let mut buffer:Vec<u8> = Vec::with_capacity(ret_size as usize);

    if unsafe {
        GetFileVersionInfoW(
            wide.as_ptr(), 
            0, 
            ret_size, 
            buffer.as_mut_ptr() as *mut _,
        )
    } == 0 
    {
        return (0,0,0,0);
    } else {
        unsafe {
            buffer.set_len(ret_size as usize)
        }
    }

    let mut block_size = 0;
    let mut block = unsafe { mem::uninitialized() };

    let sub_block: Vec<_> = OsStr::new("\\")
        .encode_wide()
        .chain(once(0))
        .collect();
    
    if unsafe {
        VerQueryValueW(
            buffer.as_ptr() as *const _,
            sub_block.as_ptr(),
            &mut block,
            &mut block_size,
        ) == 0
    }  {
        if block_size < mem::size_of::<VS_FIXEDFILEINFO>() as u32 {
            return (0,0,0,0)
        }
    }

    let info = unsafe { &*(block as *const VS_FIXEDFILEINFO)};


    return (
        HIWORD(info.dwProductVersionMS),
        LOWORD(info.dwProductVersionMS),
        HIWORD(info.dwProductVersionLS),
        LOWORD(info.dwProductVersionLS),
    )
}


#[cfg(test)]
mod tests {

    use super::*;

    /*#[cfg(windows)]
    #[test]
    fn test_file_version_in_windows() {
        let r = get_file_version(PathBuf::from("abcdefg"));
        assert_eq!(r, (0,0,0,0));
    }*/

    /*#[cfg(not(windows))]
    #[test]
    fn test_file_version_not_in_windows() {
        let r = get_file_version(PathBuf::from("abcdefg"));
        assert_eq!(r, (0,0,0,0));
    }*/

    #[test]
    fn test_same_result_on_all_systems() {
        let r = get_file_version(PathBuf::from("abcdefg"));
        assert_eq!(r, (0,0,0,0));
    }

}