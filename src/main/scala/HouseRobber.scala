import HouseRobber.Solution.rob
import org.scalatest.funsuite.AnyFunSuite

object HouseRobber {
    object Solution {
        def rob(nums: Array[Int]): Int = {
            var max = 0
            var prevMax = 0

            for (n <- nums) {
                val temp = max
                max = Math.max(max, n + prevMax)
                prevMax = temp
            }
            max
        }
    }
}

class HouseRobberTests extends AnyFunSuite {
    test("1") {
        assert(rob(Array(1, 2, 3, 1)) == 4)
    }
    test("2") {
        assert(rob(Array(2, 7, 9, 3, 1)) == 12)
    }
    test("3") {
        assert(rob(Array(2, 5, 7, 1, 1)) == 10)
    }
    test("4") {
        assert(rob(Array(2, 1, 1, 2)) == 4)
    }
    test("5") {
        assert(rob(Array(4, 1, 2)) == 6)
    }
}
