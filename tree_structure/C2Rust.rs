#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub data: libc::c_int,
    pub key: libc::c_int,
    pub level: libc::c_int,
    pub child: [*mut node; 2],
}
pub type AANode = node;
pub type AATree = *mut node;
pub type Direction = libc::c_uint;
pub const Left: Direction = 1;
pub const Right: Direction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trunk {
    pub prev: *mut trunk,
    pub str_0: *mut libc::c_char,
}
unsafe extern "C" fn new_node(mut key: libc::c_int, data: libc::c_int)
 -> *mut AANode {
    let mut t: AATree =
        malloc(::std::mem::size_of::<AANode>() as libc::c_ulong) as
            *mut AANode;
    (*t).key = key;
    (*t).data = data;
    (*t).child[0 as libc::c_int as usize] = 0 as *mut node;
    (*t).child[1 as libc::c_int as usize] = 0 as *mut node;
    (*t).level = 1 as libc::c_int;
    return t;
}
unsafe extern "C" fn rotate(mut t: AATree, mut dir: Direction) -> AATree {
    let mut rl: AATree = 0 as *mut node;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 1 as libc::c_int;
    if dir as libc::c_uint == Right as libc::c_int as libc::c_uint {
        a = 1 as libc::c_int;
        b = 0 as libc::c_int
    }
    rl = (*t).child[a as usize];
    (*t).child[a as usize] = (*rl).child[b as usize];
    (*rl).child[b as usize] = t;
    if dir as libc::c_uint == Right as libc::c_int as libc::c_uint {
        (*rl).level += 1
    }
    return rl;
}
unsafe extern "C" fn skew(mut t: AATree) -> AATree {
    if t.is_null() { return 0 as AATree }
    if (*t).chid[0 as libc::c_int as usize].is_null() { return t }
    if (*(*t).child[0 as libc::c_int as usize]).level == (*t).level {
        return rotate(t, Left)
    }
    return t;
}
unsafe extern "C" fn split(mut t: AATree) -> AATree {
    if t.is_null() { return 0 as AATree }
    if (*t).child[1 as libc::c_int as usize].is_null() ||
           (*(*t).child[1 as libc::c_int as
                            usize]).child[1 as libc::c_int as usize].is_null()
       {
        return t
    }
    if (*(*(*t).child[1 as libc::c_int as
                          usize]).child[1 as libc::c_int as usize]).level ==
           (*t).level {
        return rotate(t, Right)
    }
    return t;
}
unsafe extern "C" fn insert(mut t: AATree, mut key: libc::c_int,
                            data: libc::c_int) -> AATree {
    if t.is_null() {
        return new_node(key, data)
    } else {
        if key < (*t).key {
            (*t).child[0 as libc::c_int as usize] =
                insert((*t).child[0 as libc::c_int as usize], key, data)
        } else if key > (*t).key {
            (*t).child[1 as libc::c_int as usize] =
                insert((*t).child[1 as libc::c_int as usize], key, data)
        }
    }
    t = skew(t);
    t = split(t);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn insert_(mut t: *mut AATree, mut key: libc::c_int,
                                 data: libc::c_int) {
    *t = insert(*t, key, data);
}
unsafe extern "C" fn delete(mut t: AATree, mut key: libc::c_int) -> AATree {
    if t.is_null() { return 0 as AATree }
    if key < (*t).key {
        (*t).child[0 as libc::c_int as usize] =
            delete((*t).child[0 as libc::c_int as usize], key)
    } else if key > (*t).key {
        (*t).child[1 as libc::c_int as usize] =
            delete((*t).child[1 as libc::c_int as usize], key)
    } else if (*t).child[0 as libc::c_int as usize].is_null() &&
                  (*t).child[1 as libc::c_int as usize].is_null() {
        free(t as *mut libc::c_void);
        return 0 as AATree
    }
    if (*t).child[0 as libc::c_int as usize].is_null() {
        let mut l: AATree = 0 as *mut node;
        l = (*t).child[1 as libc::c_int as usize];
        while !(*l).child[0 as libc::c_int as usize].is_null() {
            l = (*l).child[0 as libc::c_int as usize]
        }
        (*t).key = (*l).key;
        (*t).data = (*l).data;
        (*t).child[1 as libc::c_int as usize] =
            delete((*t).child[1 as libc::c_int as usize], (*l).key)
    } else {
        let mut l_0: AATree = 0 as *mut node;
        l_0 = (*t).child[0 as libc::c_int as usize];
        while !(*l_0).child[0 as libc::c_int as usize].is_null() {
            l_0 = (*l_0).child[1 as libc::c_int as usize]
        }
        (*t).key = (*l_0).key;
        (*t).data = (*l_0).data;
        (*t).child[0 as libc::c_int as usize] =
            delete((*t).child[0 as libc::c_int as usize], (*l_0).key)
    }
    if !(*t).child[0 as libc::c_int as usize].is_null() &&
           !(*t).child[1 as libc::c_int as usize].is_null() {
        let mut lvl: libc::c_int =
            (if (*(*t).child[0 as libc::c_int as usize]).level <
                    (*(*t).child[1 as libc::c_int as usize]).level {
                 (*(*t).child[0 as libc::c_int as usize]).level
             } else { (*(*t).child[1 as libc::c_int as usize]).level }) +
                1 as libc::c_int;
        if lvl < (*t).level {
            (*t).level = lvl;
            if !(*t).child[1 as libc::c_int as usize].is_null() &&
                   lvl < (*(*t).child[1 as libc::c_int as usize]).level {
                (*(*t).child[1 as libc::c_int as usize]).level = lvl
            }
        }
    }
    t = skew(t);
    (*t).child[1 as libc::c_int as usize] =
        skew((*t).child[1 as libc::c_int as usize]);
    if !(*t).child[1 as libc::c_int as usize].is_null() &&
           !(*(*t).child[1 as libc::c_int as
                             usize]).child[1 as libc::c_int as
                                               usize].is_null() {
        (*(*t).child[1 as libc::c_int as
                         usize]).child[1 as libc::c_int as usize] =
            skew((*(*t).child[1 as libc::c_int as
                                  usize]).child[1 as libc::c_int as usize])
    }
    t = split(t);
    (*t).child[1 as libc::c_int as usize] =
        split((*t).child[1 as libc::c_int as usize]);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn delete_(mut t: *mut AATree, mut key: libc::c_int) {
    *t = delete(*t, key);
}
#[no_mangle]
pub unsafe extern "C" fn show_trunks(mut p: *mut trunk) {
    if p.is_null() { return }
    show_trunks((*p).prev);
    printf(b"%s\x00" as *const u8 as *const libc::c_char, (*p).str_0);
}
// this is very haphazzard
#[no_mangle]
pub unsafe extern "C" fn show_tree(mut root: AATree, mut prev: *mut trunk,
                                   mut is_left: libc::c_int) {
    if root.is_null() { return }
    let mut this_disp: trunk =
        {
            let mut init =
                trunk{prev: prev,
                      str_0:
                          b"    \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,};
            init
        };
    let mut prev_str: *mut libc::c_char = this_disp.str_0;
    show_tree((*root).child[0 as libc::c_int as usize], &mut this_disp,
              1 as libc::c_int);
    if prev.is_null() {
        this_disp.str_0 =
            b"---\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if is_left != 0 {
        this_disp.str_0 =
            b".--\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        prev_str =
            b"   |\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else {
        this_disp.str_0 =
            b"`--\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        (*prev).str_0 = prev_str
    }
    show_trunks(&mut this_disp);
    printf(b"%d (%d)\n\x00" as *const u8 as *const libc::c_char, (*root).data,
           (*root).key);
    if !prev.is_null() { (*prev).str_0 = prev_str }
    this_disp.str_0 =
        b"   |\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    show_tree((*root).child[1 as libc::c_int as usize], &mut this_disp,
              0 as libc::c_int);
    if prev.is_null() { puts(b"\x00" as *const u8 as *const libc::c_char); };
}
unsafe fn main_0() -> libc::c_int {
    let mut aat: AATree = 0 as AATree;
    insert_(&mut aat, 0 as libc::c_int, 5 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 1 as libc::c_int, 6 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 8 as libc::c_int, 8 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 3 as libc::c_int, 6 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 4 as libc::c_int, 10 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 5 as libc::c_int, 10 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    insert_(&mut aat, 10 as libc::c_int, 10 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    delete_(&mut aat, 8 as libc::c_int);
    show_tree(aat, 0 as *mut trunk, 0 as libc::c_int);
    return 0 as libc::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }

