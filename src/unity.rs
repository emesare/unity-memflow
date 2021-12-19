use memflow::prelude::v1::*;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseObject {
    pub previous_object: Pointer64<BaseObject>,
    pub next_object: Pointer64<BaseObject>,
    pub object: Pointer64<GameObject>,
}

unsafe impl Pod for BaseObject {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct GameObject {
    pad_0x0: [u8; 0x28],
    pub object: Address,
    //pub components: Address,
    pub components: Pointer64<ComponentList>,
    pub component_count: u32,
    pad_0x38: [u8; 0x22],
    pub name: Address,
}

impl GameObject {
    pub fn find_component<E: VirtualMemory>(&mut self, virt_mem: &mut E, name: &str) {
        
    }
}

unsafe impl Pod for GameObject {}

#[repr(C)]
#[derive(Clone, Debug, Pod)]
pub struct ComponentList {
         
}

#[repr(C)]
#[derive(Clone, Debug, Pod)]
pub struct GameObjectManager {
    pub last_tagged_object: Pointer64<BaseObject>,
    pub first_tagged_object: Pointer64<BaseObject>,
    pub last_active_object: Pointer64<BaseObject>,
    pub first_active_object: Pointer64<BaseObject>,
}

impl GameObjectManager {
    pub fn find_active<E: VirtualMemory>(&mut self, virt_mem: &mut E, name: &str) -> Option<GameObject> {
        let last_obj = self
            .last_active_object
            .deref(virt_mem)
            .expect("failed to deref first object");
        let mut current_obj = self
            .first_active_object
            .deref(virt_mem)
            .expect("failed to deref last object");

        // we are skipping the last object, FIX
        while !current_obj.next_object.is_null() && current_obj.ne(&last_obj) {
            if !current_obj.object.is_null() {
                let game_object = current_obj.object.deref(virt_mem).expect("failed to get object");
                
                if virt_mem.virt_read_cstr(game_object.name, name.len()).expect("failed to read name string").eq(name) {
                    return Some(game_object);
                }
            }

            current_obj.next_object.deref_into(virt_mem, &mut current_obj).expect("failed to deref into current object");     
        }

        None
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct InnerUnityString {
    pad_0x0: [u8; 0x10],
    pub size: i32,
    pub base: u8,
}

unsafe impl Pod for InnerUnityString {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct UnityString(Pointer64<InnerUnityString>);

unsafe impl Pod for UnityString {}

impl UnityString {
    pub fn to_string<E: VirtualMemory>(&mut self, virt_mem: &mut E) -> String {
        let inner = self.0.deref(virt_mem).expect("failed to deref");
        if inner.size > 0 && inner.size < 256 {
            let address: Address = self.0.into();
            let raw = virt_mem
                .virt_read_raw(address + 0x14, inner.size as usize * 2)
                .expect("failed to read string");

            let string: &[u16] = unsafe {
                std::slice::from_raw_parts(raw.as_ptr() as *const u16, inner.size as usize)
            };
            String::from_utf16_lossy(string).to_string()
        } else {
            "".to_string() //make this error instead later.
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct UnityListInner<T: Pod> {
    pad_0x0: [u8; 0x20],
    pub first_entry: Pointer64<T>,
}

unsafe impl<T: Pod> Pod for UnityListInner<T> {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct UnityList<T: Pod> {
    pad_0x0: [u8; 0x10],
    pub inner: Pointer64<UnityListInner<T>>,
    pub count: i32,
}

unsafe impl<T: Pod> Pod for UnityList<T> {}

impl<T: Pod> UnityList<T> {
    pub fn collect<E: VirtualMemory>(&mut self, virt_mem: &mut E) -> Vec<T> {
        let inner: UnityListInner<T> = self.inner.deref(virt_mem).expect("failed to deref");
        let mut list: Vec<T> = Vec::new();
        for i in 0..self.count {
            let addr: Address =
                (inner.first_entry + (std::mem::size_of::<T>() * i as usize)).into();
            let item: T = virt_mem.virt_read(addr).expect("failed to read");
            list.push(item);
        }
        list
    }

    pub fn read_first<E: VirtualMemory>(&mut self, virt_mem: &mut E) -> T {
        let inner: UnityListInner<T> = self.inner.deref(virt_mem).expect("failed to deref");
        inner.first_entry.deref(virt_mem).expect("failed to deref")
    }
}
