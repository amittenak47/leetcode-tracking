// Last updated: 6/7/2025, 2:42:09 AM
class Solution {
public:
    // uint32_t reverseBits(uint32_t n) {
    // //     uint32_t ret = 0, power = 31;
    // //     while (n != 0) {
    // //         ret += (n & 1) << power; // AND (1) and shift left leaving last bit to ret 
    // //         n = n >> 1; // shift original number 1 bit right
    // //         power -= 1; // decrement bitshift counter by 1
    // //     }
    // //     return ret;        
    // // }
    //     uint32_t ret = 0, power = 24;
    //     map<uint32_t, uint32_t> cache;

    //     while (n != 0) {
    //         ret += reverseByte(n & 0xff, cache) << power; // AND bitmask 1 byte at a time, shift power bits left and add to ret
    //         n = n >> 8; // shift original number 1 byte right
    //         power -= 8; // decrement power by 1 byte
    //     }
    //     return ret;
    // }

    uint32_t reverseByte(uint32_t byte, map<uint32_t, uint32_t> cache) { // cache byte
        if (cache.find(byte) != cache.end()) return cache[byte];
        
        uint32_t value = (byte * 0x0202020202 & 0x010884422010) % 1023;
        // 
        // return (b * 0x0202020202 & 0x010884422010) % 1023
        // https://graphics.stanford.edu/~seander/bithacks.html#ReverseByteWith64BitsDiv
        // 
        // MUL = (b * 0x0202020202)
        // creates five separate copies of the 8-bit byte pattern to fan-out into a 64-bit value. 
        // 
        // MUL_AND = MUL & 0x010884422010 {010 = 4b, (2^(321) => 842 = 884422) => 4b(321)4b}
        // selects the bits that are in the correct (reversed) positions, relative to each 10-bit groups of bits. The multiply and 
        // the AND operations copy the bits from the original byte so they each appear in only one of the 10-bit sets. The 
        // reversed positions of the bits from the original byte coincide with their relative positions within any 10-bit set. 
        // 
        // MUL_AND % 1023   1. 1023 = 2 ^ (10 - 1)
        // modulus division by 2^10 - 1 merges each set of 10 bits (from positions 0-9, 10-19, 20-29, ...) in the 64-bit value. 
        // They do not overlap, so the addition steps underlying the modulus division behave like or operations.
        
        cache.emplace(byte, value);
        return value;
    }

    uint32_t reverseBits(uint32_t n) {
        n = (n >> 16) | (n << 16); // create 2 groups of numbers from "n" by shifting half bits left and right and bitwise-OR 
        n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8); // continue dividing into smaller groups and bitwise-AND  
        n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4); // a = 0010 5 = 0101, c = 1100 3 = 0011, 
        n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2); // f0 = 11110000, 0f = 00001111
        n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
        return n;
    }

};