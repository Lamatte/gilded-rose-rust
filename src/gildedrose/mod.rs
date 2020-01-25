use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item { name: name, sell_in: sell_in, quality: quality }
    }

    pub fn increase_quality(&mut self) {
        if self.quality < 50 {
            self.quality = self.quality + 1;
        }
    }

    pub fn decrease_quality(&mut self) {
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
            if item.name == "Aged Brie" {
                GildedRose::update_brie(item)
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                GildedRose::update_backstage(item)
            } else if item.name == "Sulfuras, Hand of Ragnaros" {
                GildedRose::update_sulfuras();
            } else {
                GildedRose::update_normal(item)
            }
        }
    }

    fn update_sulfuras() {
        // nop !
    }

    fn update_brie(item: &mut Item) -> () {
        item.increase_quality();
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.increase_quality();
        }
    }

    fn update_backstage(item: &mut Item) -> () {
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

    fn update_normal(item: &mut Item) -> () {
        item.decrease_quality();
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.decrease_quality();
        }
    }
}

#[cfg(test)]
mod test;
