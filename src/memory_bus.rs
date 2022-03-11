const BIOS_SIZE: usize = 256;
// TODO: get correct memory sizes
const ROM_SIZE: usize = 256;
const ERAM_SIZE: usize = 256;
const WRAM_SIZE: usize = 256;
const ZRAM_SIZE: usize = 256;
const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy,Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU {
    fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        // If our index is greater than 0x1800, we're not writing to the tile set storage
        // so we can just return.
        if index >= 0x1800 { return }
        
        // Tiles rows are encoded in two bytes with the first byte always
        // on an even address. Bitwise ANDing the address with 0xffe
        // gives us the address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;
        
        // First we need to get the two bytes that encode the tile row.
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];
        
        // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
        // is therefore 16 bytes in total.
        let tile_index = index / 16;
        // Every two bytes is a new row
        let row_index = (index % 16) / 2;
        
        // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // To determine a pixel's value we must first find the corresponding bit that encodes
            // that pixels value:
            // 1111_1111
            // 0123 4567
            //
            // As you can see the bit that corresponds to the nth pixel is the bit in the nth
            // position *from the left*. Bits are normally indexed from the right.
            //
            // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
            // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
            // so on.
            //
            // We then create a mask with a 1 at that position and 0s everywhere else.
            //
            // Bitwise ANDing this mask with our bytes will leave that particular bit with its
            // original value and every other bit with a 0.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            
            // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
            // bit must be 0.
            //
            // Finally we can tell which of the four tile values the pixel is. For example, if the least
            // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
            // have tile value `Three`.
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };
            
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    } 

    fn read_vram(&self, addr: usize) -> u8 {
        self.vram[addr]
    }
}

struct MemoryBus {
    bios: [u8; BIOS_SIZE],
    rom: [u8; ROM_SIZE],
    eram: [u8; ERAM_SIZE],
    wram: [u8; WRAM_SIZE],
    zram: [u8; ZRAM_SIZE],
    gpu: GPU,
    in_bios: bool,
}

impl MemoryBus {
    // todo: add load function for rom: given file path, load rom
    fn load(&mut self, file_path: str) {

    }

    fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.read_vram(address)
            }
            _ => panic!("TODO: support other areas of memory")
        }
    }

    fn write_byte(&self, address: u16, byte: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte)
            }
            _ => panic!("TODO: support other areas of memory")
        }
    }
}
