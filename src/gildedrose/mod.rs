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
        GildedRose::increase_quality(item);
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            GildedRose::increase_quality(item);
        }
    }

    fn update_backstage(item: &mut Item) -> () {
        GildedRose::increase_quality(item);
        if item.sell_in < 11 {
            GildedRose::increase_quality(item);
        }
        if item.sell_in < 6 {
            GildedRose::increase_quality(item);
        }
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            item.quality = item.quality - item.quality;
        }
    }

    fn update_normal(item: &mut Item) -> () {
        GildedRose::decrease_quality(item);
        item.sell_in = item.sell_in - 1;
        if item.sell_in < 0 {
            GildedRose::decrease_quality(item);
        }
    }

    fn decrease_quality(item: &mut Item) {
        if item.quality > 0 {
            item.quality = item.quality - 1;
        }
    }

    fn increase_quality(item: &mut Item) -> () {
        if item.quality < 50 {
            item.quality = item.quality + 1;
        }
    }
}

#[cfg(test)]
mod test;
