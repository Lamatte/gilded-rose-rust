use std::string;
use std::vec;

pub trait Aging {
    fn update(&self, item: &mut Item);
}

struct DefaultAging;

struct SulfurasAging;

struct BrieAging;

struct BackStageAging;

impl Aging for DefaultAging {
    fn update(&self, item: &mut Item) {
        item.decrease_quality();
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.decrease_quality();
        }
    }
}

impl Aging for SulfurasAging {
    fn update(&self, _item: &mut Item) {}
}

impl Aging for BrieAging {
    fn update(&self, item: &mut Item) {
        item.increase_quality();
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.increase_quality();
        }
    }
}

impl Aging for BackStageAging {
    fn update(&self, item: &mut Item) {
        item.increase_quality();
        if item.sell_in < 11 {
            item.increase_quality();
        }
        if item.sell_in < 6 {
            item.increase_quality();
        }
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.quality = 0;
        }
    }
}

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

fn get_aging(item: &Item) -> Box<dyn Aging> {
    if item.name == "Aged Brie" {
        Box::new(BrieAging {})
    } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
        Box::new(BackStageAging {})
    } else if item.name == "Sulfuras, Hand of Ragnaros" {
        Box::new(SulfurasAging {})
    } else {
        Box::new(DefaultAging {})
    }
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item { name: name, sell_in: sell_in, quality: quality }
    }

    fn update(&mut self) {
        get_aging(self).update(self)
    }

    fn increase_quality(&mut self) {
        if self.quality < 50 {
            self.quality = self.quality + 1;
        }
    }

    fn decrease_quality(&mut self) {
        if self.quality > 0 {
            self.quality = self.quality - 1;
        }
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose { items: items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.update()
        }
    }
}

#[cfg(test)]
mod test;
