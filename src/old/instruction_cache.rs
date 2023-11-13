use crate::types::*;
use indexmap::IndexMap;

type InstructionCacheValue = [InstructionValue; VALUE_IN_LINE_NUM];
const CACHE_SIZE: usize = 1024 * 1024;
const INSTRUCTION_LINE_SIZE: usize = 256;
const TOTAL_LINE_NUM: usize = CACHE_SIZE / INSTRUCTION_LINE_SIZE;
const LINE_NUM: usize = TOTAL_LINE_NUM / WAY_NUM;
const WAY_NUM: usize = 1;
const INSTRUCTION_VALUE_SIZE: usize = 4;
pub const VALUE_IN_LINE_NUM: usize = INSTRUCTION_LINE_SIZE / INSTRUCTION_VALUE_SIZE;

#[derive(Debug, Clone)]
pub struct InstructionCacheLine {
    valid: bool,
    dirty: bool,
    accessed: bool,
    tag: Tag,
    value: InstructionCacheValue,
}

pub struct InstructionCache {
    values: [IndexMap<Tag, InstructionCacheLine>; LINE_NUM],
    way_num: usize,
    tag_bit_num: usize,
    index_bit_num: usize,
    offset_bit_num: usize,
}

pub enum InstructionCacheAccess {
    HitSet,
    HitGet(InstructionValue),
    Miss,
}

impl InstructionCache {
    pub fn new() -> Self {
        let mut values = vec![];
        for _ in 0..LINE_NUM {
            let mut map = IndexMap::with_capacity(WAY_NUM);
            for _ in 0..WAY_NUM {
                map.insert(
                    std::u32::MAX,
                    InstructionCacheLine {
                        valid: false,
                        dirty: false,
                        accessed: false,
                        tag: std::u32::MAX,
                        value: [0; VALUE_IN_LINE_NUM],
                    },
                );
            }
            values.push(map);
        }
        let values: [IndexMap<Tag, InstructionCacheLine>; LINE_NUM] = values.try_into().unwrap();
        let way_num = WAY_NUM;
        let line_size = INSTRUCTION_LINE_SIZE;
        let line_num = LINE_NUM;
        let index_bit_num = (line_num as u32).trailing_zeros() as usize;
        let offset_bit_num = (line_size as u32).trailing_zeros() as usize;
        let tag_bit_num = 32 - index_bit_num - offset_bit_num;
        InstructionCache {
            values,
            way_num,
            tag_bit_num,
            index_bit_num,
            offset_bit_num,
        }
    }

    pub fn get_offset_bit_num(&self) -> usize {
        self.offset_bit_num
    }

    fn get_tag(&self, addr: Address) -> Tag {
        addr >> (32 - self.tag_bit_num) as Tag
    }

    fn get_index(&self, addr: Address) -> InstructionCacheIndex {
        ((addr << self.tag_bit_num) >> (32 - self.index_bit_num)) as InstructionCacheIndex
    }

    fn get_offset(&self, addr: Address) -> usize {
        ((addr << (self.tag_bit_num + self.index_bit_num)) >> (32 - self.offset_bit_num)) as usize
    }

    fn get_status(&self, addr: Address) -> (Tag, InstructionCacheIndex, usize) {
        let tag = self.get_tag(addr);
        let index = self.get_index(addr);
        let offset = self.get_offset(addr);
        (tag, index, offset)
    }

    fn update_on_get(
        &mut self,
        cache_line: &InstructionCacheLine,
        tag: Tag,
        index: InstructionCacheIndex,
    ) {
        let mut cache_line = (*cache_line).clone();
        cache_line.accessed = true;
        cache_line.valid = true;
        self.values[index].swap_remove(&tag);
        self.values[index].insert(tag, cache_line);
    }

    fn update_on_set(
        &mut self,
        mut cache_line: InstructionCacheLine,
        tag: Tag,
        index: InstructionCacheIndex,
    ) {
        cache_line.dirty = true;
        cache_line.accessed = true;
        cache_line.valid = true;
        self.values[index].swap_remove(&tag);
        self.values[index].insert(tag, cache_line);
    }

    pub fn get(&mut self, addr: Address) -> InstructionCacheAccess {
        let (tag, index, offset) = self.get_status(addr);
        let cache_line_candidates = self.values[index].clone();
        let cache_line = cache_line_candidates.get(&tag);
        match cache_line {
            Some(cache_line) => {
                if !cache_line.valid {
                    return InstructionCacheAccess::Miss;
                }
                let value = cache_line.value[offset >> 2];
                self.update_on_get(cache_line, tag, index);
                return InstructionCacheAccess::HitGet(value);
            }
            None => {
                return InstructionCacheAccess::Miss;
            }
        }
    }

    pub fn set_line(
        &mut self,
        addr: Address,
        line: [InstructionValue; VALUE_IN_LINE_NUM],
    ) -> Option<[(Address, InstructionValue); VALUE_IN_LINE_NUM]> {
        let tag = self.get_tag(addr);
        let index = self.get_index(addr);
        let cache_line_candidates = &self.values[index];
        let cache_line = cache_line_candidates.get(&tag);
        assert!(cache_line.is_none());

        let mut dirty_line_evicted = false;
        let mut evicted_values = [(0, 0); VALUE_IN_LINE_NUM];
        if self.values[index].len() >= self.way_num {
            let lru_key = self.values[index].keys().next().cloned();
            if let Some(k) = lru_key {
                let cache_line = self.values[index].get(&k).unwrap();
                if cache_line.dirty {
                    dirty_line_evicted = true;
                    let addr = (cache_line.tag << (self.index_bit_num + self.offset_bit_num))
                        as Address
                        + (index << self.offset_bit_num) as Address;
                    for i in 0..VALUE_IN_LINE_NUM {
                        evicted_values[i] = (addr + i as Address, cache_line.value[i]);
                    }
                }
                self.values[index].swap_remove(&k);
            }
        }
        let mut cache_line = InstructionCacheLine {
            valid: true,
            dirty: false,
            accessed: true,
            tag,
            value: [0; VALUE_IN_LINE_NUM],
        };
        for i in 0..VALUE_IN_LINE_NUM {
            cache_line.value[i] = line[i];
        }
        self.values[index].insert(tag, cache_line);

        if dirty_line_evicted {
            return Some(evicted_values);
        } else {
            return None;
        }
    }

    pub fn set(&mut self, addr: Address, value: InstructionValue) -> InstructionCacheAccess {
        let (tag, index, offset) = self.get_status(addr);
        let cache_line_candidates = &self.values[index];
        let cache_line = cache_line_candidates.get(&tag);
        match cache_line {
            Some(cache_line) => {
                let mut cache_line = (*cache_line).clone();
                if !cache_line.valid {
                    return InstructionCacheAccess::Miss;
                }
                cache_line.value[offset >> 2] = value;
                self.update_on_set(cache_line, tag, index);
                return InstructionCacheAccess::HitSet;
            }
            None => {
                return InstructionCacheAccess::Miss;
            }
        }
    }
}
