import TwoSum.Solution.twoSum
import org.scalatest.funsuite.AnyFunSuite

import scala.collection.immutable.HashMap

object TwoSum {

    object Solution {
        def twoSum(nums: Array[Int], target: Int): Array[Int] = {
            var map: Map[Int, Int] = new HashMap()
            var i = 0
            while (i < nums.length) {
                if (map.contains(target - nums(i)))
                    return Array(map(target - nums(i)), i)
                else
                    map += (nums(i) -> i)
                i = i + 1
            }
            Array(0, 0)
        }
    }
}

class TwoSumTests extends AnyFunSuite {
    test("1") {
        assert(twoSum(Array(2, 7, 11, 15), 9) sameElements Array(0, 1))
    }
    test("2") {
        assert(twoSum(Array(3, 2, 4), 6) sameElements Array(1, 2))
    }
    test("3") {
        assert(twoSum(Array(3, 3), 6) sameElements Array(0, 1))
    }
}
