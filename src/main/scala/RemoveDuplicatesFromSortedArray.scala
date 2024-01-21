import RemoveDuplicatesFromSortedArray.Solution.removeDuplicates
import org.scalatest.funsuite.AnyFunSuite

object RemoveDuplicatesFromSortedArray {
    object Solution {
        def removeDuplicates(nums: Array[Int]): Int = {
            var i = 1
            var prevValue = nums(0)
            var uniqueIndex = 1

            while (i < nums.length) {
                val value = nums(i)
                if (value != prevValue) {
                    prevValue = value
                    nums(uniqueIndex) = value
                    uniqueIndex += 1
                }
                i += 1
            }
            uniqueIndex
        }
    }
}

class RemoveDuplicatesFromSortedArrayTest extends AnyFunSuite {
    test("1") {
        assert(removeDuplicates(Array(1, 1, 2)) == 2)
    }
}