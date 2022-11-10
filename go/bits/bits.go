// / Bits manipulation algorithms
package bits

// BitArray represents an array of bits.
type BitArray []uint

// Add two BitArrays
func (b BitArray) Add(a BitArray) BitArray {
	res := make(BitArray, len(b))
	var carry uint = 0
	for i := len(b) - 1; i >= 0; i-- {
		sum := b[i] + a[i] + carry
		res[i] = sum % 2
		carry = sum / 2
	}
	if carry != 0 {
		res = append(res, 0)
		copy(res[1:], res[:len(res)-1])
		res[0] = carry
	}
	return res

}
