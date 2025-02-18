class Solution {
    fun smallestNumber(pattern: String): String {
        fun find(num: Int, i: Int, used: Int): String? {
            if (i == pattern.length) return num.toString()
            
            var min = 1
            var max = 9
            
            if (pattern[i] == 'I') {
                min = num % 10 + 1
            } else {
                max = num % 10 - 1
            }

            for (d in min..max) {
                val bit = 1 shl d
                if (used and bit == 0) {
                    find(num * 10 + d, i + 1, used or bit)?.let { return it }
                }
            }
            
            return null
        }

        for (i in 1..9) {
            find(i, 0, 1 shl i)?.let { return it }
        }

        error("")
    }
}