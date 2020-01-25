use std::string;
use std::vec;

pub trait Aging {
    fn update(&self, item: &mut Item);
}

struct DefaultAging;
impl Aging for DefaultAging {
    fn update(&self, item: &mut Item) {
        if item.name == "Aged Brie" {
            item.update_brie()
        } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
            item.update_backstage()
        } else if item.name == "Sulfuras, Hand of Ragnaros" {
            item.update_sulfuras();
        } else {
            item.update_normal()
        }
    }
}

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item { name: name, sell_in: sell_in, quality: quality }
    }

    fn update(&mut self) {
        let aging = Box::new(DefaultAging{});
        aging.update(self)
    }

    fn update_sulfuras(&mut self) {
        // nop !
    }

    fn update_brie(&mut self) -> () {
        self.increase_quality();
        self.sell_in = self.sell_in - 1;
        if self.sell_in < 0 {
            self.increase_quality();
        }
    }

    fn update_backstage(&mut self) -> () {
        self.increase_quality();
        if self.sell_in < 11 {
            self.increase_quality();
        }
        if self.sell_in < 6 {
            self.increase_quality();
        }
        self.sell_in = self.sell_in - 1;
        if self.sell_in < 0 {
            self.quality = 0;
        }
    }

    fn update_normal(&mut self) -> () {
        self.decrease_quality();
        self.sell_in = self.sell_in - 1;
        if self.sell_in < 0 {
            self.decrease_quality();
        }
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
